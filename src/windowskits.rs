use std::{env, fs, path::PathBuf};
use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

/// Returns the path to the `Windows Kits` directory. It's by default at
/// `C:\Program Files (x86)\Windows Kits\10`.
fn get_windows_kits_dir() -> PathBuf {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";
    let dir: String = hklm
        .open_subkey(key)
        .unwrap()
        .get_value("KitsRoot10")
        .unwrap();

    return dir.into();
}

/// Returns the path to the given includes. The path may look like this:
/// `C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\<name>`.
fn get_windows_kits_includes(name: &str) -> PathBuf {
    let include_dir = get_windows_kits_dir().join("include").read_dir().unwrap();

    let max_verdir = include_dir
        .filter_map(|dir| dir.ok())
        .map(|dir| dir.path())
        .filter(|dir| {
            dir.components()
                .last()
                .and_then(|c| c.as_os_str().to_str())
                .map(|c| c.starts_with("10.") && dir.join(name).is_dir())
                .unwrap_or(false)
        })
        .max()
        .ok_or(format!("Can not find a valid {} dir", name))
        .unwrap();

    return max_verdir.join(name);
}

pub fn get_header_path(name: &str) -> PathBuf {
    let includes_dir = get_windows_kits_includes("km");
    return includes_dir.join(name);
}

pub fn get_header(name: &str) -> String {
    let header = get_header_path(name);
    if header.exists() && header.is_file() {
        return fs::read_to_string(header).unwrap();
    }

    panic!("Cannot find a valid {:?} header", name);
}

pub fn get_test_header_path() -> PathBuf {
    return env::current_dir().unwrap().join("test.hpp");
}

pub fn get_test_header() -> String {
    let header = get_test_header_path();
    if header.exists() && header.is_file() {
        return fs::read_to_string(header).unwrap();
    }

    panic!("Cannot find the test header");
}
