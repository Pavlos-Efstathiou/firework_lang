use home::home_dir;
use std::error::Error;
use std::fs::{self};
use std::path::PathBuf;

pub fn get_core_path() -> PathBuf {
    let home = home_dir().unwrap_or_else(|| panic!("Could access your home directory"));
    home.join(".firework_lang").join("core.firework")
}

pub fn is_core_installed() -> bool {
    get_core_path().exists()
}

pub fn install_core() -> Result<(), Box<dyn Error>> {
    let core = reqwest::blocking::get("https://firework-lang.netlify.app/core.firework")?.text()?;
    let core_path = get_core_path();

    fs::create_dir_all(core_path.parent().unwrap())?;
    fs::write(core_path, core)?;
    Ok(())
}
