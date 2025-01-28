mod linux_da;
mod windows_da;

use std::env;
use std::io;

fn main() -> io::Result<()>{
    let os_type = env::consts::OS;

    match os_type{
        
        "linux" => {
            run_instrumentation(linux_da::instrument_cpp_file)?;
        }

        "windows" => {
            run_instrumentation(windows_da::instrument_cpp_file)?;
        }

        _ => {
            eprintln!("Unsupported OS: {}", os_type);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn run_instrumentation( platform_specific_fn: fn(&std::path::Path, &std::path::Path)-> io::Result<()>) -> io::Result<()>{
    let input_dir = std::path::PathBuf::from("input");
    let output_dir = std::path::PathBuf::from("output");

    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    for entry in std::fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("cpp") {
            let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
            let output_file_name = format!("{}_da.cpp", file_name);
            let output_path = output_dir.join(output_file_name);

            println!("Processing file: {:?}", path);

            platform_specific_fn(&path, &output_path)?;
        }
    }

    println!("All files processed successfully!");
    Ok(())
}











// use std::{fs, io, path::Path};

// #[cfg(target_os = "linux")]
// mod linux_da;

// #[cfg(target_os = "windows")]
// mod windows_da;


// #[cfg(target_os = "linux")]
// fn main() {
//     println!("Running on Linux!");

//     if let Err(e) = linux_da::run_instrumentation() {
//         eprintln!("Error: {}", e);
//     }
// }

// #[cfg(target_os = "windows")]
// fn main() {
//     println!("Running on Windows!");

//     if let Err(e) = windows_da::run_instrumentation() {
//         eprintln!("Error: {}", e);
//     }
// }

