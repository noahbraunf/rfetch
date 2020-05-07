use heim::{memory, units::information};

const GIBIBYTE: f32 = 1024.;

pub async fn get_memory_info() -> heim::Result<String> {
    let memory = memory::memory().await?;

    Ok(format!(
        "{:.1} (total), {:.1} (available)",
        memory.total().get::<information::mebibyte>() as f32 / GIBIBYTE,
        memory.available().get::<information::mebibyte>() as f32 / GIBIBYTE,
    ))
}
