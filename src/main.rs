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

#[macro_export]
macro_rules! colorize {
    ($t:expr, $s:expr) => {
        format!("{}: {}", ::ansi_term::Color::Yellow.bold().paint($t), $s)
    };
    ($c:tt, $t:expr, $s:expr) => {
        format!("{}: {}", $c.bold().paint($t), $s)
    };
}

#[async_std::main]
async fn main() -> heim::Result<()> {
    convert_to_table().await?.printstd();
    println!();
    Ok(())
}

async fn convert_to_table() -> heim::Result<Table> {
    use system_info::*;
    let uptime = colorize!("Uptime", time::get_uptime().await?);
    let memory_info = colorize!("Memory", memory::get_memory_info().await?);
    let host_info = host::get_host_info().await?;
    let cpu_info = colorize!("CPU", cpu::get_cpu_info());
    let gpu_info = colorize!("GPU", gpu::get_gpu_info());
    let shell = colorize!("Shell", host::get_shell_info());
    let colors = colors::get_16_terminal_colors();

    let name = host_info.0;
    let os = colorize!("OS", host_info.1);

    let mut inner_table = if let Some(pacman) = package::get_package_amount() {
        table!(
            [name.0],
            [std::iter::repeat("-").take(name.1).collect::<String>()],
            [os],
            [uptime],
            [colorize!("Packages", pacman)],
            [shell],
            [cpu_info],
            [gpu_info],
            [memory_info],
            [""],
            [colors]
        )
    } else {
        table!(
            [name.0],
            [std::iter::repeat("-").take(name.1).collect::<String>()],
            [os],
            [uptime],
            [shell],
            [cpu_info],
            [gpu_info],
            [memory_info],
            [""],
            [colors]
        )
    };
    inner_table.set_format(*FORMAT_CLEAN);
    let mut outer_table = Table::new();
    outer_table.add_row(row![APPLE_LOGO, inner_table]);
    outer_table.set_format(*FORMAT_CLEAN);
    Ok(outer_table)
}
