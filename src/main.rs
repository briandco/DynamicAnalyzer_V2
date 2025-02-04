mod linux_da;
mod windows_da;

use std::env;
use std::fs;
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
            let output_file_name = format!("{}.cpp", file_name);
            let output_path = output_dir.join(output_file_name);

            println!("Processing file: {:?}", path);

            platform_specific_fn(&path, &output_path)?;
        }else {
            let output_path = output_dir.join(path.file_name().unwrap());
            fs::copy(&path, &output_path)?;
            println!("Copied file: {:?} to {:?}", path, output_path);
        }
    }

    println!("All files processed successfully!");
    Ok(())
}