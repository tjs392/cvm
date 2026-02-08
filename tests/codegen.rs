use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn run_compiler(code: &str) -> (bool, String) {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let path = format!("/tmp/test_codegen_{}.c", id);
    
    std::fs::write(&path, code).unwrap();
    
    Command::new("cargo")
        .args(["build", "--quiet"])
        .status()
        .unwrap();
    
    let output = Command::new("./target/debug/cvm")
        .arg(&path)
        .output()
        .unwrap();
    
    let _ = std::fs::remove_file(&path);
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    (output.status.success(), stdout)
}

#[test]
fn test_simple_add() {
    let code = r#"
    int main() {
        5 + 3;
    }
    "#;
    
    let (success, output) = run_compiler(code);
    assert!(success);
    assert!(output.contains("LOADK"));
    assert!(output.contains("ADD"));
    assert!(output.contains("K0: 5"));
    assert!(output.contains("K1: 3"));
}

#[test]
fn test_nested_arithmetic() {
    let code = r#"
int main() {
    (10 + 5) * 2;
}
"#;
    
    let (success, output) = run_compiler(code);
    assert!(success);
    assert!(output.contains("ADD"));
    assert!(output.contains("MUL"));
}

#[test]
fn test_multiple_operations() {
    let code = r#"
int main() {
    1 + 2;
    3 * 4;
}
"#;
    
    let (success, output) = run_compiler(code);
    assert!(success);
    assert!(output.contains("ADD"));
    assert!(output.contains("MUL"));
}