use heim;
use humantime;
use std::time::Duration;

pub async fn get_uptime() -> heim::Result<humantime::FormattedDuration> {
    let ut = heim::host::uptime().await?;
    Ok(humantime::format_duration(Duration::from_secs_f64(
        ut.get::<heim::units::time::second>(),
    )))
}
