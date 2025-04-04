use preferences::{AppInfo, Preferences, PreferencesMap};
use std::collections::HashSet;
use std::path::PathBuf;
use std::{env, fs};

/******************************************************************************
 * Preferences
 *****************************************************************************/
const APP_INFO: AppInfo = AppInfo {
    name: "natvis4qt",
    author: "narnaud",
};
const QT_ROOT_KEY: &str = "qt-root";
const INSTALL_KEYS_KEY: &str = "install-keys";
const AUTOUPDATE_KEY: &str = "autoupdate";

/// Get the preferences map
fn get_prefs() -> PreferencesMap<String> {
    PreferencesMap::load(&APP_INFO, env!("CARGO_PKG_NAME")).unwrap_or_default()
}
/// Save the preferences map
fn save_prefs(prefs: &PreferencesMap<String>) {
    let save_result = prefs.save(&APP_INFO, env!("CARGO_PKG_NAME"));
    assert!(save_result.is_ok());
}

/// Store the Qt installation root directory.
pub fn set_qt_root(qt_root: &str) {
    let mut prefs = get_prefs();
    prefs.insert(QT_ROOT_KEY.into(), qt_root.into());
    save_prefs(&prefs);
}

/// Get the Qt installation root directory.
pub fn get_qt_root() -> String {
    let prefs = get_prefs();
    prefs
        .get(QT_ROOT_KEY)
        .cloned()
        .unwrap_or("C:\\Qt".to_string())
}

/// Store the keys of the directories to install the natvis files.
///
/// This method is adding the new keys to the existing ones.
pub fn set_install_keys(keys: &[String]) {
    let mut prefs = get_prefs();

    // Merge the new keys with the old ones
    let old_keys = get_install_keys().unwrap_or_default();
    let mut new_keys = old_keys.iter().collect::<HashSet<_>>();
    new_keys.extend(keys.iter());

    let new_keys = new_keys.into_iter().cloned().collect::<Vec<String>>();
    prefs.insert(INSTALL_KEYS_KEY.into(), new_keys.join(","));
    save_prefs(&prefs);
}

/// Get the keys of the directories to install the natvis files.
pub fn get_install_keys() -> Option<Vec<String>> {
    let prefs = get_prefs();
    prefs
        .get(INSTALL_KEYS_KEY)
        .map(|s| s.split(',').map(|s| s.to_string()).collect())
}

/// Get the autoupdate preference.
pub fn get_autoupdate() -> bool {
    let prefs = get_prefs();
    prefs
        .get(AUTOUPDATE_KEY)
        .map(|s| s == "true")
        .unwrap_or(true)
}

/// Set the autoupdate preference.
pub fn set_autoupdate(autoupdate: bool) {
    let mut prefs = get_prefs();
    prefs.insert(AUTOUPDATE_KEY.into(), autoupdate.to_string());
    save_prefs(&prefs);
}

/******************************************************************************
 * Natvis file handling
 *****************************************************************************/

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
        ("vs2019", "Visual Studio 2019"),
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

/// Returns the VS Code Cpptools visualizer directory.
fn get_vscode_cpptools_dir() -> Option<NatvisInfo> {
    let mut extensions_dir = dirs::home_dir().expect("Could not find home directory");
    extensions_dir.push(".vscode\\extensions");

    for entry in std::fs::read_dir(extensions_dir)
        .ok()?
        .filter_map(|e| e.ok())
    {
        if entry
            .file_name()
            .to_str()
            .is_some_and(|n| n.starts_with("ms-vscode.cpptools"))
        {
            let mut visualizer_dir = entry.path();
            visualizer_dir.push("debugAdapters\\vsdbg\\bin\\Visualizers");

            if visualizer_dir.exists() {
                return Some(NatvisInfo {
                    key: "vscodeCpptools".to_owned(),
                    name: "VS Code C/C++ Extension".to_string(),
                    path: visualizer_dir,
                    version: vec![5, 6],
                });
            }
        }
    }

    None
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
                key: format!("{}-{}", &version, &entry),
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
pub fn get_natvis_info(qt_root: PathBuf) -> Vec<NatvisInfo> {
    let mut dirs = Vec::new();

    dirs.extend(get_msvc_dirs());
    dirs.extend(get_vscode_cpptools_dir());
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
    let default_root = PathBuf::from(get_qt_root());
    if default_root.exists() {
        Some(default_root)
    } else {
        None
    }
}
