use clap::{Arg, ArgAction, Command};

/// Simple CLI tool to generate folder structure in ASCII for markdown files.
fn main() {
    // CLI interface
    let matches = Command::new("tree_gen")
        .version("0.1.0")
        .about("tree-gen is a CLI tool to generate folder structure in ASCII for markdown files.")
        .author("Kei-K23")
        .arg(
            Arg::new("path")
                .help("Path of the directory to display")
                .required(true),
        )
        .arg(
            Arg::new("depth")
                .help("Maximum depth of the tree")
                .short('d')
                .long("depth")
                .default_value("10"),
        )
        .arg(
            Arg::new("ignore_hidden")
                .help("Ignore hidden files and folders")
                .short('i')
                .long("ignore-hidden")
                .required(false)
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let url = matches.get_one::<String>("path").unwrap();
    let depth_str = matches.get_one::<String>("depth").unwrap();
    let depth_int = depth_str.parse::<i32>().unwrap();
    let ignore = matches.get_one::<bool>("ignore_hidden").unwrap();

    println!("{url} {ignore} {depth_int}")
}
