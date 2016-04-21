use std::process::Command;

#[test]
fn ghp_bash_test_script() {
    let output = Command::new("./test.sh")
                     .current_dir("tests")
                     .output()
                     .unwrap();

    assert!(output.status.success());
}
