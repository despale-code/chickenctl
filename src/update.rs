//! `chickenctl update`: runs the installer's update mode (docs/update.ps1 on the website), which
//! compares every installed program with its repository and rebuilds the ones that changed.

use anyhow::{bail, Context, Result};
use std::os::windows::process::CommandExt;
use std::process::Command;

const UPDATE_SCRIPT: &str = "https://despale-code.github.io/chickenwm/update.ps1";
const CREATE_NEW_CONSOLE: u32 = 0x0000_0010;

pub fn run(check: bool) -> Result<()> {
    let script = format!("irm '{UPDATE_SCRIPT}' | iex");
    let mut cmd = Command::new("powershell.exe");
    cmd.args(["-NoProfile", "-ExecutionPolicy", "Bypass"]);
    if check {
        // Only reads: runs right here and prints the list.
        let status = cmd
            .env("CHICKENWM_UPDATE", "check")
            .args(["-Command", &script])
            .status()
            .context("could not start PowerShell")?;
        if !status.success() {
            bail!("the update check failed");
        }
        return Ok(());
    }
    // A real update closes every ChickenWM program, this one included, and replaces its exe, so
    // it runs in a window of its own that stays open to show the result.
    cmd.env("CHICKENWM_UPDATE", "1")
        .args(["-NoExit", "-Command", &script])
        .creation_flags(CREATE_NEW_CONSOLE)
        .spawn()
        .context("could not start PowerShell")?;
    println!("ok: the updater is running in a new window.");
    Ok(())
}
