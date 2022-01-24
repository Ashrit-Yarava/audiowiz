use std::process::Command;
use glob::glob;
use std::path::Path;
use std::fs;
use rayon::prelude::*;
use ansi_term::Colour::Green;

fn run_command(input_file: &str, output_file: &str) {
   let _output = Command::new("sox")
       .args([input_file, output_file, "echo", "0.5", "0.5", "200", "0.3", "reverb", "delay", "0", "+0.1"])
       .output()
       .expect("Failed to run SoX, is it in the PATH?");
}

fn get_files() -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for entry in glob("*.mp3").expect("No files found...") {
        if let Ok(path) = entry {
            files.push(format!("{:?}", path.display()))
        }
    }
    return files;
}

fn loop_surround(input_file: &str) {
    let input = input_file.trim_matches('"'); 
    let output = format!("wizzed/{input}");
    run_command(input, &output);
}


fn main() {
    let _files = get_files();

    if !Path::new("wizzed").is_dir() {
        let _ = fs::create_dir("wizzed");
    }

    _files.into_par_iter().for_each(|file| {
        loop_surround(&file);
        println!(" * {}", 
                 Green.paint(file.as_str().replace("\\", "").trim_matches('"')));
    })

}
