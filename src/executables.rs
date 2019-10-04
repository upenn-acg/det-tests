///! Programs that require building executables.
use crate::*;

/// Run command on looking for executable in ./resources/executables.
pub fn make_command(command: &str) -> impl Fn() -> Result<Stdout, Error> {
    let command = command.to_string();
    move || command_with_output(&DET_EXE, None, &[&command], Some("./resources/executables"))
}

/// Build program from ./resources/programs/ and placing the executable
/// in ./resources/executables
pub fn build_c_program(program: &str) {
    let input_file = &format!("resources/test_programs/{}.c", program);
    let output_file = &format!("resources/executables/{}", program);
    let mut child = Command::new("gcc")
        .args(&["-pthread", "-std=gnu99", input_file, "-o", output_file])
        .stderr(Stdio::piped())
        .spawn()
        .expect("Unable to spawn gcc");

    let status = child.wait().expect("Unable to compile program");

    if !status.success() {
        let mut stderr = String::new();
        child
            .stderr
            .unwrap()
            .read_to_string(&mut stderr)
            .expect("Unable to read stdout to string.");
        panic!("Unable to compile program: {}", stderr);
    }
}

pub fn run_n_times(command: impl Fn() -> Result<Stdout, Error>, n: u32) {
    let stdout = command().expect("Unable to exec program");

    for _ in 0..n {
        let stdout2 = command().expect("Unable to exec program");
        assert_eq!(stdout, stdout2);
    }
}


#[test]
fn test_inverse_fork() {
   build_c_program(&"inverseFork");
   let command = make_command(&format!("./{}", "inverseFork"));
   run_n_times(command, 2);
}

#[test]
fn test_nested_fork() {
   build_c_program(&"nestedFork");
   let command = make_command(&format!("./{}", "nestedFork"));
   run_n_times(command, 2);
}

#[test]
fn test_vfork() {
   build_c_program(&"vfork");
   let command = make_command(&format!("./{}", "vfork"));
   run_n_times(command, 2);
}

#[test]
fn test_clock_gettime() {
   build_c_program(&"clock_gettime");
   let command = make_command(&format!("./{}", "clock_gettime"));
   run_n_times(command, 2);
}

#[test]
fn test_getpid() {
   build_c_program(&"getpid");
   let command = make_command(&format!("./{}", "getpid"));
   run_n_times(command, 2);
}

#[test]
fn test_uname() {
   build_c_program(&"uname");
   let command = make_command(&format!("./{}", "uname"));
   run_n_times(command, 2);
}

#[test]
fn test_pipe() {
   build_c_program(&"pipe");
   let command = make_command(&format!("./{}", "pipe"));
   run_n_times(command, 2);
}

#[test]
fn test_get_random() {
   build_c_program(&"getRandom");
   let command = make_command(&format!("./{}", "getRandom"));
   run_n_times(command, 2);
}

#[test]
fn test_wait_on_child() {
   build_c_program(&"waitOnChild");
   let command = make_command(&format!("./{}", "waitOnChild"));
   run_n_times(command, 2);
}

#[test]
fn test_fchownat() {
   build_c_program(&"fchownat");
   let command = make_command(&format!("./{}", "fchownat"));
   run_n_times(command, 2);
}

#[test]
fn test_fork_and_pipe() {
   build_c_program(&"forkAndPipe");
   let command = make_command(&format!("./{}", "forkAndPipe"));
   run_n_times(command, 2);
}

#[test]
fn test_hello_world() {
   build_c_program(&"helloWorld");
   let command = make_command(&format!("./{}", "helloWorld"));
   run_n_times(command, 2);
}

#[test]
fn test_2writers1reader() {
   build_c_program(&"2writers1reader");
   let command = make_command(&format!("./{}", "2writers1reader"));
   run_n_times(command, 2);
}

#[test]
fn test_fuse_single_read() {
   build_c_program(&"fuse-single-read");
   let command = make_command(&format!("./{}", "fuse-single-read"));
   run_n_times(command, 2);
}

#[test]
fn test_fuse_single_write() {
   build_c_program(&"fuse-single-write");
   let command = make_command(&format!("./{}", "fuse-single-write"));
   run_n_times(command, 2);
}

#[test]
fn test_open() {
   build_c_program(&"open");
   let command = make_command(&format!("./{}", "open"));
   run_n_times(command, 2);
}

#[test]
fn test_openat() {
   build_c_program(&"openat");
   let command = make_command(&format!("./{}", "openat"));
   run_n_times(command, 2);
}

#[test]
fn test_creat() {
   build_c_program(&"creat");
   let command = make_command(&format!("./{}", "creat"));
   run_n_times(command, 2);
}

#[test]
fn test_sigsegv() {
   build_c_program(&"sigsegv");
   let command = make_command(&format!("./{}", "sigsegv"));
   run_n_times(command, 2);
}

#[test]
fn test_sigill() {
   build_c_program(&"sigill");
   let command = make_command(&format!("./{}", "sigill"));
   run_n_times(command, 2);
}

#[test]
fn test_sigabrt() {
   build_c_program(&"sigabrt");
   let command = make_command(&format!("./{}", "sigabrt"));
   run_n_times(command, 2);
}

#[test]
fn test_kill() {
   build_c_program(&"kill");
   let command = make_command(&format!("./{}", "kill"));
   run_n_times(command, 2);
}

#[test]
fn test_alarm_handler() {
   build_c_program(&"alarm-handler");
   let command = make_command(&format!("./{}", "alarm-handler"));
   run_n_times(command, 2);
}

#[test]
fn test_alarm_nohandler() {
   build_c_program(&"alarm-nohandler");
   let command = make_command(&format!("./{}", "alarm-nohandler"));
   run_n_times(command, 2);
}

// TODO
// #[test]
// fn test_alarm_ignore() {
//    build_c_program(&"alarm-ignore");
//    let command = make_command(&format!("./{}", "alarm-ignore"));
//    run_n_times(command, 2);
// }

#[test]
fn test_select_without_timeout() {
   build_c_program(&"selectWithoutTimeout");
   let command = make_command(&format!("./{}", "selectWithoutTimeout"));
   run_n_times(command, 2);
}

#[test]
fn test_select_with_timeout() {
   build_c_program(&"selectWithTimeout");
   let command = make_command(&format!("./{}", "selectWithTimeout"));
   run_n_times(command, 2);
}

#[test]
fn test_getdents() {
   build_c_program(&"getdents");
   let command = make_command(&format!("./{}", "getdents"));
   run_n_times(command, 2);
}

#[test]
fn test_getdents64() {
   build_c_program(&"getdents64");
   let command = make_command(&format!("./{}", "getdents64"));
   run_n_times(command, 2);
}

#[test]
fn test_poll_without_timeout() {
   build_c_program(&"pollWithoutTimeout");
   let command = make_command(&format!("./{}", "pollWithoutTimeout"));
   run_n_times(command, 2);
}

#[test]
fn test_pollWithPositiveTimeout() {
   build_c_program(&"pollWithPositiveTimeout");
   let command = make_command(&format!("./{}", "pollWithPositiveTimeout"));
   run_n_times(command, 2);
}

#[test]
fn test_pollWithNegativeTimeout() {
   build_c_program(&"pollWithNegativeTimeout");
   let command = make_command(&format!("./{}", "pollWithNegativeTimeout"));
   run_n_times(command, 2);
}

#[test]
fn test_rdtsc() {
   build_c_program(&"rdtsc");
   let command = make_command(&format!("./{}", "rdtsc"));
   run_n_times(command, 2);
}

#[test]
fn test_rdtscp() {
   build_c_program(&"rdtscp");
   let command = make_command(&format!("./{}", "rdtscp"));
   run_n_times(command, 2);
}

#[test]
fn test_nanosleep() {
   build_c_program(&"nanosleep");
   let command = make_command(&format!("./{}", "nanosleep"));
   run_n_times(command, 2);
}

#[test]
fn test_nanosleep_par() {
   build_c_program(&"nanosleep-par");
   let command = make_command(&format!("./{}", "nanosleep-par"));
   run_n_times(command, 2);
}

#[test]
fn test_alarm_resethand() {
   build_c_program(&"alarm-resethand");
   let command = make_command(&format!("./{}", "alarm-resethand"));
   run_n_times(command, 2);
}

#[test]
fn test_readDevRandom() {
   build_c_program(&"readDevRandom");
   let command = make_command(&format!("./{}", "readDevRandom"));
   run_n_times(command, 2);
}

#[test]
fn test_readDevRandomMultiple() {
   build_c_program(&"readDevRandomMultiple");
   let command = make_command(&format!("./{}", "readDevRandomMultiple"));
   run_n_times(command, 2);
}

#[test]
fn test_readDevUrandom() {
   build_c_program(&"readDevUrandom");
   let command = make_command(&format!("./{}", "readDevUrandom"));
   run_n_times(command, 2);
}

#[test]
fn test_exec_mkstemp() {
   build_c_program(&"exec-mkstemp");
   let command = make_command(&format!("./{}", "exec-mkstemp"));
   run_n_times(command, 2);
}

#[test]
fn test_complex_mkdirat_dirfd() {
   build_c_program(&"complex_mkdirat_dirfd");
   let command = make_command(&format!("./{}", "complex_mkdirat_dirfd"));
   run_n_times(command, 2);
}

#[test]
fn test_mkdir() {
   build_c_program(&"mkdir");
   let command = make_command(&format!("./{}", "mkdir"));
   run_n_times(command, 2);
}

#[test]
fn test_mkdirat_fdcwd() {
   build_c_program(&"mkdirat_fdcwd");
   let command = make_command(&format!("./{}", "mkdirat_fdcwd"));
   run_n_times(command, 2);
}

#[test]
fn test_mknod() {
   build_c_program(&"mknod");
   let command = make_command(&format!("./{}", "mknod"));
   run_n_times(command, 2);
}

#[test]
fn test_mknod_fullpath() {
   build_c_program(&"mknod_fullpath");
   let command = make_command(&format!("./{}", "mknod_fullpath"));
   run_n_times(command, 2);
}

#[test]
fn test_open_already_exists() {
   build_c_program(&"open_already_exists");
   let command = make_command(&format!("./{}", "open_already_exists"));
   run_n_times(command, 2);
}

#[test]
fn test_openat_already_exists() {
   build_c_program(&"openat_already_exists");
   let command = make_command(&format!("./{}", "openat_already_exists"));
   run_n_times(command, 2);
}

#[test]
fn test_simpleCreat() {
   build_c_program(&"simpleCreat");
   let command = make_command(&format!("./{}", "simpleCreat"));
   run_n_times(command, 2);
}

#[test]
fn test_simple_mkdirat_dirfd() {
   build_c_program(&"simple_mkdirat_dirfd");
   let command = make_command(&format!("./{}", "simple_mkdirat_dirfd"));
   run_n_times(command, 2);
}

#[test]
fn test_symlink() {
   build_c_program(&"symlink");
   let command = make_command(&format!("./{}", "symlink"));
   run_n_times(command, 2);
}

#[test]
fn test_symlinkat() {
   build_c_program(&"symlinkat");
   let command = make_command(&format!("./{}", "symlinkat"));
   run_n_times(command, 2);
}

#[test]
fn test_vdso_funcs() {
   build_c_program(&"vdso-funcs");
   let command = make_command(&format!("./{}", "vdso-funcs"));
   run_n_times(command, 2);
}

#[test]
fn test_multithreaded() {
   build_c_program(&"multithreaded");
   let command = make_command(&format!("./{}", "multithreaded"));
   run_n_times(command, 2);
}

#[test]
fn test_multipleThreads() {
   build_c_program(&"multipleThreads");
   let command = make_command(&format!("./{}", "multipleThreads"));
   run_n_times(command, 2);
}

#[test]
fn test_processAndThread() {
   build_c_program(&"processAndThread");
   let command = make_command(&format!("./{}", "processAndThread"));
   run_n_times(command, 2);
}

#[test]
fn test_processThreadProcess() {
   build_c_program(&"processThreadProcess");
   let command = make_command(&format!("./{}", "processThreadProcess"));
   run_n_times(command, 2);
}

#[test]
fn test_processThreadThread() {
   build_c_program(&"processThreadThread");
   let command = make_command(&format!("./{}", "processThreadThread"));
   run_n_times(command, 2);
}

#[test]
fn test_pthreadJoin() {
   build_c_program(&"pthreadJoin");
   let command = make_command(&format!("./{}", "pthreadJoin"));
   run_n_times(command, 2);
}

#[test]
fn test_pthreadNoJoin() {
   build_c_program(&"pthreadNoJoin");
   let command = make_command(&format!("./{}", "pthreadNoJoin"));
   run_n_times(command, 2);
}

#[test]
fn test_ptpThreadJoin() {
   build_c_program(&"ptpThreadJoin");
   let command = make_command(&format!("./{}", "ptpThreadJoin"));
   run_n_times(command, 2);
}

#[test]
fn test_ptpThreadNoJoin() {
   build_c_program(&"ptpThreadNoJoin");
   let command = make_command(&format!("./{}", "ptpThreadNoJoin"));
   run_n_times(command, 2);
}

#[test]
fn test_twoPthreadsJoin() {
   build_c_program(&"twoPthreadsJoin");
   let command = make_command(&format!("./{}", "twoPthreadsJoin"));
   run_n_times(command, 2);
}

#[test]
fn test_twoPthreadsNoJoin() {
   build_c_program(&"twoPthreadsNoJoin");
   let command = make_command(&format!("./{}", "twoPthreadsNoJoin"));
   run_n_times(command, 2);
}

#[test]
fn test_tenThreadJoin() {
   build_c_program(&"tenThreadJoin");
   let command = make_command(&format!("./{}", "tenThreadJoin"));
   run_n_times(command, 2);
}

#[test]
fn test_tenThreadNoJoin() {
   build_c_program(&"tenThreadNoJoin");
   let command = make_command(&format!("./{}", "tenThreadNoJoin"));
   run_n_times(command, 2);
}

#[test]
fn test_exitgroup() {
   build_c_program(&"exitgroup");
   let command = make_command(&format!("./{}", "exitgroup"));
   run_n_times(command, 2);
}

#[test]
fn test_exitgroupMainProcess() {
   build_c_program(&"exitgroupMainProcess");
   let command = make_command(&format!("./{}", "exitgroupMainProcess"));
   run_n_times(command, 2);
}

#[test]
fn test_condvar_parent_wait() {
   build_c_program(&"condvar-parent-wait");
   let command = make_command(&format!("./{}", "condvar-parent-wait"));
   run_n_times(command, 2);
}
