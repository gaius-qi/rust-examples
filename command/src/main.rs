use std::error::Error;
use tokio::process::{Child, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = run_ping()?;
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    Ok(())
}

fn run_ping() -> Result<Child, Box<dyn Error>> {
    let mut cmd = Command::new("ping");
    cmd.arg("google.com");
    cmd.stdout(std::process::Stdio::null());
    cmd.stderr(std::process::Stdio::null());
    cmd.stdin(std::process::Stdio::null());

    // Create a new session for the process and make it the leader, this will
    // ensures that the child process is fully detached from its parent and will
    // continue running in the background even after the parent process exits
    //
    // SAFETY: This closure runs in the forked child process before it starts
    // executing, this is a highly unsafe environment because the process isn't
    // running yet so seemingly innocuous operation like allocating memory may
    // hang indefinitely.
    // The only thing we do here is issuing a syscall, which is safe to do in
    // this state but still "unsafe" in Rust semantics because it's technically
    // mutating the shared global state of the process
    unsafe {
        cmd.pre_exec(|| {
            libc::setsid();
            Ok(())
        });
    }

    let child = cmd.spawn()?;
    Ok(child)
}
