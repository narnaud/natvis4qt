use clap::Parser;

#[derive(Parser)]
#[command(version, long_about = None)]
struct Args {
    /// List of files to use
    names: Vec<String>,
    /// Output file
    #[clap(short, long)]
    output: Option<String>,
}

fn main () {
    let args = Args::parse();

    if args.names.len() < 2 {
        println!("Nothing to do");
        std::process::exit(1);
    }

    // Open the first file and read the content except the last 2 lines
    let file = std::fs::read_to_string(&args.names[0]).expect("Could not read file");
    let mut lines: Vec<String> = file.lines().take(file.lines().count() - 2).map(|s| s.to_string()).collect();

    for file_name in &args.names[1..] {
        let file = std::fs::read_to_string(file_name).expect("Could not read file");
        let new_lines: Vec<String> = file.lines().skip(3).map(|s| s.to_string()).collect();
        lines.extend(new_lines);
        std::fs::remove_file(file_name).expect("Could not remove file");
    }

    // Write into the first file
    std::fs::write(&args.names[0], lines.join("\n")).expect("Could not write output file");
}
