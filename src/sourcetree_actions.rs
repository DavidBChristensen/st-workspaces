use std::process::Output;

use anyhow::bail;

pub fn close_sourcetree() -> anyhow::Result<Output> {
    let mut command = std::process::Command::new("taskkill");
    command.arg("/IM").arg("SourceTree.exe");
    let output = command.output()?;
    println!("Asked to kill SourceTree.exe status --> {} ", output.status);
    match output.status.code() {
        Some(0) => Ok(output),
        Some(128) => {
            bail!("Process not running")
        }
        _ => bail!("Unknown error"),
    }
}
