use ansi_term::Color::Yellow;
use prettytable::{cell, format::consts::FORMAT_CLEAN, row, table, Table};
mod colors;
mod system_info;

const APPLE_LOGO: &str = r#"          .:'
      __ :'__
   .'`__`-'__``.
  :__________.-'
  :_________:
   :_________`-;
    `.__.-.__.'"#;

macro_rules! colorize {
    ($t:expr, $s:expr) => {
        format!("{}: {}", Yellow.bold().paint($t), $s)
    };
}

#[async_std::main]
async fn main() -> heim::Result<()> {
    convert_to_table().await?.printstd();
    Ok(())
}

async fn convert_to_table() -> heim::Result<Table> {
    use system_info::*;
    let cpu_info = colorize!("CPU", cpu::get_cpu_info());
    let memory_info = colorize!("Memory", memory::get_memory_info().await?);
    let host_info = host::get_host_info().await?;
    let shell = colorize!("Shell", host::get_shell_info());
    let uptime = colorize!("Uptime", time::get_uptime().await?);
    let colors = colors::get_16_terminal_colors();

    let name = host_info.0;
    let OS = colorize!("OS", host_info.1);

    let mut inner_table = table!(
        [name],
        ["---------------"],
        [OS],
        [uptime],
        [shell],
        [cpu_info],
        [memory_info],
        [""],
        [colors]
    );
    inner_table.set_format(*FORMAT_CLEAN);
    let mut outer_table = Table::new();
    outer_table.add_row(row![APPLE_LOGO, inner_table]);
    outer_table.set_format(*FORMAT_CLEAN);
    Ok(outer_table)
}
