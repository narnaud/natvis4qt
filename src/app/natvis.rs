use std::path::PathBuf;
use std::{env, fs};

/// Struct to store natvis information for installation.
pub struct NatvisInfo {
    /// Key to identify the directory
    pub key: String,
    /// Name to display to the user
    pub name: String,
    /// Path to the directory
    pub path: PathBuf,
    /// Version(s) to install
    pub version: Vec<u8>,
}

/// Returns all the MS Visual Studio user directories.
fn get_msvc_dirs() -> Vec<NatvisInfo> {
    let mut dirs = Vec::new();

    let document_dir = dirs::document_dir().expect("Could not find document directory");

    let vs_versions = [
        ("2019", "Visual Studio 2019"),
        ("vs2022", "Visual Studio 2022"),
    ];
    for (key, name) in vs_versions.iter() {
        let vs_dir = document_dir.join(name);
        if vs_dir.exists() {
            dirs.push(NatvisInfo {
                key: key.to_string(),
                name: name.to_string(),
                path: vs_dir.join("Visualizers"),
                version: vec![5, 6],
            });
        }
    }

    dirs
}

/// Returns all the Qt installation.
fn get_qt_dirs(qt_root: PathBuf) -> Vec<NatvisInfo> {
    if !qt_root.exists() {
        return Vec::new();
    }

    let mut dirs = Vec::new();

    // Get all children of C:\Qt, and check the one looking like *.*.* (version)
    let qt_versions = qt_root
        .read_dir()
        .expect("Could not read Qt directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|entry| entry.contains('.'))
        .map(|entry| entry.to_string());

    for version in qt_versions {
        // If it has a child directory starting with msvc, add it to the list
        let qt_version_dir = qt_root.join(&version);

        let qt_version_major = version
            .split('.')
            .next()
            .expect("Could not extract Qt major version")
            .parse::<u8>()
            .expect("Could not parse Qt major version");

        let qt_msvc_dirs = qt_version_dir
            .read_dir()
            .expect("Could not read Qt version directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|entry| entry.starts_with("msvc"))
            .map(|entry| NatvisInfo {
                key: format!("{}{}", &version, &entry),
                name: {
                    let mut qt_version = "Qt ".to_string();
                    qt_version.push_str(&version);
                    qt_version
                },
                path: qt_version_dir.join(&entry).join("natvis"),
                version: vec![qt_version_major],
            });

        dirs.extend(qt_msvc_dirs);
    }

    dirs
}

/// Returns all the possible natvis info for installation.
pub fn get_natvis_installs(qt_root: PathBuf) -> Vec<NatvisInfo> {
    let mut dirs = Vec::new();

    dirs.extend(get_msvc_dirs());
    dirs.extend(get_qt_dirs(qt_root));

    dirs
}

/// Copy natvis file to the given path.
pub fn copy_natvis_file(info: &NatvisInfo) -> Result<(), std::io::Error> {
    for version in info.version.iter() {
        let natvis_file_name = format!("qt{}.natvis", version);
        let dst = &info.path;

        // Create directory unconditionally
        if fs::create_dir_all(dst).is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Could not create {}", dst.display()),
            ));
        }

        // Get executable directory
        let src = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("natvis")
            .join(&natvis_file_name);

        if fs::copy(src, dst.join(&natvis_file_name)).is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Could not copy {} to {}", natvis_file_name, dst.display()),
            ));
        }
    }
    Ok(())
}

/// Get default qt root directory, if it exists.
pub fn get_default_qt_root() -> Option<PathBuf> {
    let default_root = PathBuf::from("C:\\Qt");
    if default_root.exists() {
        Some(default_root)
    } else {
        None
    }
}
