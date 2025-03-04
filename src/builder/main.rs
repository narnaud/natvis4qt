use std::collections::HashMap;
use std::fs;

use clap::Parser;

#[derive(Parser)]
#[command(version, long_about = None, disable_version_flag = true)]
struct Args {
    /// Directory with the natvis files
    dir: String,
    /// Output directory
    #[clap(short, long)]
    output: Option<String>,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = Args::parse();

    let qt_files = fun_name(args.dir);

    // Create output directory if it does not exist
    if let Some(output) = &args.output {
        fs::create_dir_all(output).expect("Could not create output directory");
    }
    let output = args.output.clone().unwrap_or_else(|| ".".to_string());

    // Create a natvis file for each version
    for (version, files) in qt_files {
        create_natvis_file(&output, version, VERSION, files);
    }
}

fn fun_name(dir: String) -> HashMap<char, Vec<String>> {
    // Get all the files in the directory
    let mut files = fs::read_dir(&dir)
        .expect("Could not read directory")
        .map(|entry| entry.expect("Could not read entry").path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    files.sort_by(|a, b| b.cmp(a));

    // separate in different vector based on the name qt*, * being a number
    let mut qt_files: HashMap<char, Vec<String>> = HashMap::new();
    for file in files {
        let file_name = file
            .file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could not convert file name to string")
            .to_string();
        let version = file_name
            .chars()
            .nth(2)
            .expect("Could not get version character");
        let complete_file_name = format!("{}\\{}", dir, file_name);
        qt_files
            .entry(version)
            .and_modify(|f| f.push(complete_file_name.clone()))
            .or_insert(vec![complete_file_name]);
    }
    qt_files
}

fn create_natvis_file(output: &String, qt_version: char, natvis_version: &str, files: Vec<String>) {
    let output = std::path::Path::new(output).join(format!("qt{}.natvis", qt_version));

    let file = std::fs::read_to_string(&files[0]).expect("Could not read file");

    let mut lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();

    for file_name in &files[1..] {
        lines.truncate(lines.len() - 2);
        let file = std::fs::read_to_string(file_name).expect("Could not read file");
        let new_lines: Vec<String> = file.lines().skip(3).map(|s| s.to_string()).collect();
        lines.extend(new_lines);
    }

    // Add the version as the second line
    lines.insert(1, format!("<!-- Qt Natvis version: {} -->", natvis_version));

    std::fs::write(output, lines.join("\n")).expect("Could not write output file");
}
