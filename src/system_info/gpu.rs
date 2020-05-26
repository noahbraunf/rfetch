use metal::Device;

pub fn get_gpu_info() -> String {
    let devices: Vec<String> = Device::all().iter().map(|a| a.name().to_string()).collect();

    devices.join(", ")
}
