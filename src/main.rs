use rayon::prelude::*;
use std::fs;
use std::path::Path;

mod convert;
mod parser;

fn main() {

    let args = parser::parse();

    // Create the output directory if it does not exist.
    if !Path::new(&args[1]).is_dir() {
        let _ = fs::create_dir_all(&args[1]);
    }

    let _files = convert::get_files(&args[0]);
    _files.into_par_iter().for_each(|file| {
        convert::command_loop(&file, &args[1]);
    })

}
