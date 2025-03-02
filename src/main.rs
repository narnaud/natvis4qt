use console::style;
use std::path::PathBuf;

/// Struct to store natvis information for installation.
struct NatvisInfo {
    /// Key to identify the directory
    key: String,
    /// Name to display to the user
    name: String,
    /// Path to the directory
    path: PathBuf,
}

fn main() {
    cliclack::intro(style("Natvis installation for Qt").on_green().black()).unwrap();

    let qt_root = ui_get_qt_root();
    if qt_root.is_err() {
        ui_outro_error();
        return;
    }

    let natvis_installs = get_natvis_installs(qt_root.unwrap());

    let install_keys = ui_get_install_keys(&natvis_installs);
    if install_keys.is_err() {
        ui_outro_error();
        return;
    }

    cliclack::outro(style("Success!").green().bold()).unwrap();
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

        let qt_msvc_dirs = qt_version_dir
            .read_dir()
            .expect("Could not read Qt version directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|entry| entry.starts_with("msvc"))
            .map(|entry| entry.to_string())
            .map(|entry| NatvisInfo {
                key: version.clone(),
                name: {
                    let mut qt_version = "Qt ".to_string();
                    qt_version.push_str(&version);
                    qt_version
                },
                path: qt_version_dir.join(entry).join("natvis"),
            });

        dirs.extend(qt_msvc_dirs);
    }

    dirs
}

/// Returns all the possible natvis info for installation.
fn get_natvis_installs(qt_root: PathBuf) -> Vec<NatvisInfo> {
    let mut dirs = Vec::new();

    dirs.extend(get_msvc_dirs());
    dirs.extend(get_qt_dirs(qt_root));

    dirs
}

/// UI: ask the user for the Qt installation root.
fn ui_get_qt_root() -> Result<PathBuf, std::io::Error> {
    let default_root = PathBuf::from("C:\\Qt");
    if default_root.exists() {
        cliclack::log::info(format!(
            "Default Qt installation root found in {}",
            style(default_root.to_str().unwrap()).cyan()
        ))?;
        return Ok(default_root);
    }

    cliclack::input("Qt installation root?")
        .placeholder("C:\\Qt")
        .validate(|input: &String| {
            if PathBuf::from(input).exists() {
                Ok(())
            } else {
                Err("Please enter a valid path")
            }
        })
        .interact()
}

/// UI: ask the user to select the directories to install the natvis files.
/// Returns the selected directories keys.
fn ui_get_install_keys(dirs: &[NatvisInfo]) -> Result<Vec<&str>, std::io::Error> {
    let keys = dirs
        .iter()
        .map(|info| info.key.as_str())
        .collect::<Vec<_>>();
    let dirs_for_multiselect = dirs
        .iter()
        .map(|info| {
            (
                info.key.as_str(),
                info.name.as_str(),
                info.path.to_str().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    cliclack::multiselect("Select known directories to install natvis files")
        .initial_values(keys)
        .items(&dirs_for_multiselect)
        .interact()
}

/// UI: show error outro message.
fn ui_outro_error() {
    cliclack::outro(format!(
        "Problems? {}\n",
        style("https://github.com/narnaud/natvis4qt/issues")
            .cyan()
            .underlined()
    ))
    .unwrap();
    std::process::exit(0)
}
