use std::str::FromStr;
use std::fs::read_to_string;
use nix::libc::pid_t;
use crate::error::BGError;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

/// Get supervisord's pid from its pid file.
pub fn get_pid() -> Result<u32, BGError> {
    let pid_string = read_to_string("/etc/bene-gesserit/supervisord.pid")?;
    Ok(u32::from_str(pid_string.as_str())?)
}

/// Reload supervisord and all processes.
pub fn hup(pid: u32) -> Result<(), BGError> {
    let npid = Pid::from_raw(pid as pid_t);
    Ok(kill(npid, Signal::SIGHUP)?)
}

/// Stop supervisord and all of its child processes
pub fn term(pid: u32) -> Result<(), BGError> {
    let npid = Pid::from_raw(pid as pid_t);
    Ok(kill(npid, Signal::SIGTERM)?)
}