use ansi_term::Color::Green;
use futures::stream::StreamExt;
use heim::host;
use std::env;

pub async fn get_host_info() -> heim::Result<(String, String)> {
    let host = host::platform().await?;
    let username = host::users().next().await.unwrap()?.username().to_string();

    Ok((
        format!(
            "{}@{}",
            Green.bold().paint(username),
            Green.bold().paint(host.hostname())
        ),
        format!(
            "{} {} {}",
            host.system(),
            host.release(),
            host.architecture().as_str()
        ),
    ))
}

pub fn get_shell_info() -> String {
    env::var("SHELL").map_or("".to_string(), |s| s)
}
