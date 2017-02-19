#[macro_use]
extern crate json;
extern crate winreg;

use std::io::prelude::*;
use std::fs::File;
use winreg::RegKey;
use winreg::enums::*;
use std::error::Error;
use std::env;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::SeekFrom;

/// Finds the path where Steam is installed. Uses the Registry key located at `HKEY_CURRENT_USER\Software\Valve\Steam\SteamPath`.
///
/// # Examples
/// ```
/// let path_to_steam = find_steam().expect("Steam not found!");
/// ```
fn find_steam() -> Result<String, Box<Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let steam_reg: RegKey = hkcu.open_subkey_with_flags("Software\\Valve\\Steam", KEY_READ)?;
    return Ok(steam_reg.get_value("SteamPath")?);
}

fn main() {
    // -------------------------------
    // Find config file
    // -------------------------------
    let steam_path = find_steam().expect("Could not determine path to Steam!");
    let args: Vec<String> = env::args().collect();
    let driver: &str = if args.len() < 2 {
        println!("Falling back to default driver");
        "lighthouse"
    } else { &args[1] };
    let config_file = Path::new(&steam_path).join("config/steamvr.vrsettings");

    if !config_file.as_path().exists() {
        panic!("Could not find steamvr.vrsettings!");
    }

    // println!("Steam: {}, Driver: {}, Config file: {}", steam_path, driver, config_file.as_path().display());

    // -------------------------------
    // Parse config file
    // -------------------------------
    let mut file: File = OpenOptions::new().read(true).write(true).open(&config_file).unwrap();
    let mut contents = String::new();

    // Read everything
    file.read_to_string(&mut contents).unwrap();

    // Parse everything
    let mut parsed = json::parse(&contents).expect("Could not parse JSON!");

    // Change driver
    parsed["steamvr"]["forcedDriver"] = driver.into();

    // -------------------------------
    // Write back
    // -------------------------------
    // Truncate
    match file.set_len(0).and_then(|_| file.seek(SeekFrom::Start(0))).and_then(|_| parsed.write_pretty(&mut file, 2)) {
        Ok(_) => println!("Switched to {}", driver),
        Err(e) => panic!("Failed to write config: {}", e)
    }
}
