use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

pub fn get_package_amount() -> Option<usize> {
    let folder = get_package_folder().unwrap();
    fs::read_dir(folder).map_or(None, |dirs| Some(dirs.count()))
}

fn get_package_folder() -> io::Result<PathBuf> {
    let brew_package_location =
        String::from_utf8(Command::new("brew").arg("--cellar").output()?.stdout);

    if let Ok(string) = brew_package_location {
        return Ok(PathBuf::from(string));
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Brew not working"));
    }
}
