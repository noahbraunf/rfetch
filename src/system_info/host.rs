use ansi_term::Color::Green;
use futures::stream::StreamExt;
use heim::host;
use std::env;
use unicode_segmentation::UnicodeSegmentation;

pub async fn get_host_info() -> heim::Result<((String, usize), String)> {
    let host = host::platform().await?;
    let username = host::users().next().await.unwrap()?.username().to_string();

    Ok((
        (
            format!(
                "{}@{}",
                Green.bold().paint(username.clone()),
                Green.bold().paint(host.hostname())
            ),
            format!("{}@{}", username, host.hostname())
                .graphemes(true)
                .count(),
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
