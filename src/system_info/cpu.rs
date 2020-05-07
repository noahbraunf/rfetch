use raw_cpuid;

const GIGAHERTZ_IN_MEGAHERTZ: u16 = 1000;

pub fn get_cpu_info() -> String {
    let cpuid = raw_cpuid::CpuId::new();
    format!(
        "{model} (base), {boost_frequency:.2}GHz (boost)",
        model = cpuid
            .get_extended_function_info()
            .as_ref()
            .map_or_else(
                || "n/a",
                |info| info.processor_brand_string().unwrap_or("unreadable"),
            )
            .replace("(R)", "")
            .replace("(TM)", ""),
        boost_frequency = cpuid.get_processor_frequency_info().as_ref().map_or_else(
            || 0.0,
            |f| f.processor_max_frequency() as f32 / GIGAHERTZ_IN_MEGAHERTZ as f32,
        )
    )
}
