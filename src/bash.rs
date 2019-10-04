///! Bash specific tests.
use crate::*;

fn bash_command(command: &str) -> Result<Stdout, Error> {
    command_with_output(&DET_EXE, None, &["bash", "-c", command], None)
}

#[test]
fn test_uid() {
    let stdout = bash_command("id -u").expect("Non-zero exit status");
    assert_eq!(stdout.trim_end(), "0");
}

#[test]
fn test_gid() {
    let stdout1 = bash_command("times").expect("Non-zero exit status");
    let stdout2 = bash_command("times").expect("Non-zero exit status");
    assert_eq!(stdout1, stdout2);
}
