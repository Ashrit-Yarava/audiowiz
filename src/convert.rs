use std::path::Path;
use std::process::Command;

use glob::glob;

use ansi_term::Colour::Green;
use ansi_term::Colour::Red;

fn run_command(input_file: &str, output_file: &str) {
    // Run the SoX command.
    let _output = Command::new("sox")
        .args([
            input_file,
            output_file,
            "echo",
            "0.5",
            "0.5",
            "200",
            "0.3",
            "reverb",
            "delay",
            "0",
            "+0.1",
        ])
        .output()
        .expect("Failed to run SoX, is it in the PATH?");
}

pub fn get_files(directory: &str) -> Vec<String> {
    // OUtputs an array with files that are in the specified directory that are .mp3 files.
    let mut files: Vec<String> = Vec::new();
    let glob_pattern = format!("{directory}/*.mp3");
    for entry in glob(&glob_pattern).expect("No files found...") {
        if let Ok(path) = entry {
            files.push(format!("{}", path.display()))
        }
    }
    return files;
}

pub fn command_loop(input_file: &str, directory: &str) {
    let result = Path::new(input_file).file_name();
    if let Some(path) = result {
        let output_file = format!("{}{}", directory, path.to_string_lossy());
        if Path::new(&output_file).exists() {
            println!(" * {}", Red.paint(output_file)); // Red Color for not converted.
            return;
        }
        run_command(input_file, &output_file);
        println!(" * {}", Green.paint(output_file)); // Green Color for converted.
    } else {
        return;
    }
}
