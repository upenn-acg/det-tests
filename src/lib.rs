use std::fs::File;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::io::Read;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use wait_timeout::ChildExt;

mod bash;
mod executables;

// Time out in seconds any given process command.
const TIMEOUT: u64 = 10;

type Stdout = String;
type Stderr = String;

lazy_static! {
    static ref DET_EXE: String = {
        // TODO Change to envvar
        compile_error!("Please specify full path to detTrace executable here:");
        // Example:
        // String::from("/home/gatowololo/Research/detTrace/bin/dettrace")
    };
}

#[derive(Debug)]
pub enum Error {
    NonZeroReturnStatus(Stderr),
    Timeout(Stderr),
}

pub fn init_state() {}

/// Turn stdout from process into string.
pub fn get_process_output(mut output_file: File) -> String {
    let mut out = String::new();
    output_file.read_to_string(&mut out)
        .expect("Unable to read output to string.");
    out
}


/// Main function to run commands and get their output.
/// Should be wrapped to narrow it's use. See `bash::bash_command` for an example.
/// # Arguments
///
/// * `det_exe` - Full path to executable to determinize program.
/// * `det_exe` - determinizing program specific options.
/// * `command` - Command to run under determinizing program.
/// * `working_dir` - Working dir to run command from.
pub fn command_with_output(
    det_exe: &str,
    det_exe_options: Option<&[&str]>,
    command: &[&str],
    working_dir: Option<&str>,
) -> Result<Stdout, Error> {
    // Write outputs to files to avoid deadlocks for really long outputs.
    let stdout_contents = tempfile::NamedTempFile::new().unwrap();
    let stderr_contents = tempfile::NamedTempFile::new().unwrap();

    let mut child = Command::new(det_exe);

    // add options here if any.
    if let Some(options) = det_exe_options {
        child.args(options);
    }

    let child = child
        .args(command)
        // stdout/stderr require their own handle to file, reopen does this.
        .stdout(Stdio::from(stdout_contents.reopen().unwrap()))
        .stderr(Stdio::from(stderr_contents.reopen().unwrap()));

    // Use specified cwd if any.
    if let Some(wd) = working_dir {
        child.current_dir(wd);
    }

    // Let child run!
    let mut child = child.spawn().expect("Cannot spawn process");

    // Wait until timeout here for it to finish.
    let timeout = Duration::from_secs(TIMEOUT);
    let timeout_results = child
        .wait_timeout(timeout)
        .expect("Unable to wait for process to finish");
    match timeout_results {
        None => {
            // child hasn't exited yet
            child.kill().unwrap();
            child.wait().unwrap().code();
            panic!("Process timed out...");
        }
        Some(status) => {
            if status.success() {
                Ok(get_process_output(stdout_contents.into_file()))
            } else {
                let stderr = get_process_output(stderr_contents.into_file());
                Err(Error::NonZeroReturnStatus(stderr))
            }
        }
    }
}
