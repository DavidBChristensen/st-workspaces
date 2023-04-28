use anyhow::bail;

pub enum CloseResult{
    Closed,
    ProcessNotRunning,
}

/// Closes SourceTree. Only works in Windows.
pub fn close_sourcetree() -> anyhow::Result<CloseResult> {
    let mut command = std::process::Command::new("taskkill");
    command.arg("/IM").arg("SourceTree.exe");
    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(CloseResult::Closed),
        Some(128) => Ok(CloseResult::ProcessNotRunning),
        Some(code) => bail!("Unknown error closing SourceTree. code {code}"),
        None => bail!("Unknown error closing SourceTree."),
    }
}
