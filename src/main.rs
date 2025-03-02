use console::style;
use std::path::PathBuf;

/// Returns all the MS Visual Studio user directories.
fn get_msvc_dirs() -> Vec<(String, String, PathBuf)> {
    let mut dirs = Vec::new();

    let document_dir = dirs::document_dir().expect("Could not find document directory");

    // Test if Document\Visual Studio 2019 exists
    let vs2019_dir = document_dir.join("Visual Studio 2019");
    if vs2019_dir.exists() {
        dirs.push((
            "vs2019".to_string(),
            "Visual Studio 2019".to_string(),
            vs2019_dir.join("Visualizers"),
        ));
    }

    // Test if Document\Visual Studio 2022 exists
    let vs2022_dir = document_dir.join("Visual Studio 2022");
    if vs2022_dir.exists() {
        dirs.push((
            "vs2022".to_string(),
            "Visual Studio 2022".to_string(),
            vs2022_dir.join("Visualizers"),
        ));
    }

    dirs
}

/// Returns all the Qt installation.
fn get_qt_dirs() -> Vec<(String, String, PathBuf)> {
    let mut dirs = Vec::new();

    let qt_dir = PathBuf::from("C:\\Qt");
    if !qt_dir.exists() {
        return dirs;
    }

    // Get all children of C:\Qt, and check the one looking like *.*.* (version)
    let qt_versions = qt_dir
        .read_dir()
        .expect("Could not read Qt directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|entry| entry.contains('.'))
        .map(|entry| entry.to_string());

    for version in qt_versions {
        // If it has a child directory starting with msvc, add it to the list
        // C:\Qt\5.15.2\msvc2019_64\natvis
        let qt_version_dir = qt_dir.join(&version);

        let qt_msvc_dirs = qt_version_dir
            .read_dir()
            .expect("Could not read Qt version directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|entry| entry.starts_with("msvc"))
            .map(|entry| entry.to_string())
            .map(|entry| {
                (
                    version.clone(),
                    {
                        let mut qt_version = "Qt ".to_string();
                        qt_version.push_str(&version);
                        qt_version
                    },
                    qt_version_dir.join(entry).join("natvis"),
                )
            });

        dirs.extend(qt_msvc_dirs);
    }

    dirs
}

/// Returns all the directories the natvis could be installed in.
fn get_possible_install_dirs() -> Vec<(String, String, PathBuf)> {
    let mut dirs = Vec::new();

    dirs.extend(get_msvc_dirs());
    dirs.extend(get_qt_dirs());

    dirs
}

fn main() {
    cliclack::intro(style("Natvis installation for Qt").on_green().black()).unwrap();

    let dirs = get_possible_install_dirs();
    let keys = dirs
        .iter()
        .map(|(key, _, _)| key.as_str())
        .collect::<Vec<_>>();
    let dirs_for_multiselect = dirs
        .iter()
        .map(|(key, name, path)| (key.as_str(), name.as_str(), path.to_str().unwrap()))
        .collect::<Vec<_>>();

    let install_dirs = cliclack::multiselect("Select known directories to install natvis files")
        .initial_values(keys)
        .items(&dirs_for_multiselect)
        .interact();

    match install_dirs {
        Ok(_) => {
            cliclack::outro(style("Success!").green().bold()).unwrap();
        }
        Err(_) => {
            cliclack::outro(format!(
                "Problems? {}\n",
                style("https://github.com/narnaud/natvis4qt/issues")
                    .cyan()
                    .underlined()
            ))
            .unwrap();
        }
    }
}
