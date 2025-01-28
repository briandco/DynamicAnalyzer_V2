use std::{fs, io, path::Path};
use std::io::Write;


const INSTRUMENTATION_BEGIN: &str = r#"MEMORYSTATUSEX statex_start;
statex_start.dwLength = sizeof(statex_start);
GlobalMemoryStatusEx(&statex_start);
int start_var;

///////////////// Wall Time ////////////////////////
// Start measuring time
LARGE_INTEGER frequency, start_wall, end_wall;
QueryPerformanceFrequency(&frequency);
QueryPerformanceCounter(&start_wall);

///////////////// CPU Time /////////////////////////
// Start measuring CPU time
FILETIME creation_time, exit_time, kernel_time_start, user_time_start;
GetProcessTimes(GetCurrentProcess(), &creation_time, &exit_time, &kernel_time_start, &user_time_start);"#;

const INSTRUMENTATION_END: &str = r#"int end_var;

MEMORYSTATUSEX statex_end;
statex_end.dwLength = sizeof(statex_end);
GlobalMemoryStatusEx(&statex_end);
std::cout << "\nStack used = " << ((reinterpret_cast<intptr_t>(&end_var)) - (reinterpret_cast<intptr_t>(&start_var)) - 4);
std::cout << "\nMemory in use = " << (statex_end.dwMemoryLoad - statex_start.dwMemoryLoad) << "%\n";

//////////////// Wall Time ///////////////
QueryPerformanceCounter(&end_wall);
double elapsed_wall = (double)(end_wall.QuadPart - start_wall.QuadPart) * 1000000.0 / frequency.QuadPart;

printf("Wall Time measured: %.3f microseconds.\n", elapsed_wall);

//////////////// CPU Time ///////////////
FILETIME kernel_time_end, user_time_end;
GetProcessTimes(GetCurrentProcess(), &creation_time, &exit_time, &kernel_time_end, &user_time_end);

ULONGLONG start_time = (static_cast<ULONGLONG>(kernel_time_start.dwLowDateTime) | 
                        (static_cast<ULONGLONG>(kernel_time_start.dwHighDateTime) << 32)) +
                       (static_cast<ULONGLONG>(user_time_start.dwLowDateTime) |
                        (static_cast<ULONGLONG>(user_time_start.dwHighDateTime) << 32));

ULONGLONG end_time = (static_cast<ULONGLONG>(kernel_time_end.dwLowDateTime) | 
                      (static_cast<ULONGLONG>(kernel_time_end.dwHighDateTime) << 32)) +
                     (static_cast<ULONGLONG>(user_time_end.dwLowDateTime) |
                      (static_cast<ULONGLONG>(user_time_end.dwHighDateTime) << 32));

double elapsed_cpu = (double)(end_time - start_time) / 10.0;

printf("CPU Time measured: %.3f microseconds.\n\n", elapsed_cpu);"#;

pub fn instrument_cpp_file( input_path:&Path, output_path:&Path ) -> io::Result<()>{
    
    let content = fs::read_to_string(input_path)?;
    let mut output = Vec::new();

    let mut inside_function = false;
    let mut waiting_for_braces = false;
    let mut brace_count = 0;
    let mut includes_inserted = false;
    let mut pending_instrumentation_end = false;

    const ADDITIONAL_INCLUDES: &str = r#"#include <windows.h>
#include <iostream>"#;

    for line in content.lines(){
        let trimmed_line = line.trim();

        if trimmed_line == "#include <iostream>" && !includes_inserted{
            output.push(line.to_string());
            output.push(ADDITIONAL_INCLUDES.to_string());
            includes_inserted = true;
            continue;
        }

        if trimmed_line.ends_with("{") && trimmed_line.contains(")") && !inside_function{
            inside_function = true;
            brace_count = 1;
            let parts:Vec<&str> = trimmed_line.splitn(2, "{").collect();
            output.push(parts[0].to_string() + "{");
            output.push(INSTRUMENTATION_BEGIN.to_string());
            if parts.len() > 1 {
                output.push(parts[1].to_string());
            }
            continue;
        }

        if trimmed_line.ends_with(")") && !inside_function {
            waiting_for_braces = true;
            output.push(line.to_string());
            continue;
        }

        if waiting_for_braces && trimmed_line == "{" {
            waiting_for_braces = false;
            brace_count = 1;
            inside_function = true;

            output.push(line.to_string());
            output.push(INSTRUMENTATION_BEGIN.to_string());
            continue;
        }

        if inside_function {
            if trimmed_line.starts_with("return") {
                output.push(INSTRUMENTATION_END.to_string());
                output.push(line.to_string());
                pending_instrumentation_end = false;
                continue;
            }

            if trimmed_line.contains("{") {
                brace_count += 1;
            }
            if trimmed_line.contains("}") {
                brace_count -= 1;

                if brace_count == 0 {
                    if pending_instrumentation_end {
                        output.push(INSTRUMENTATION_END.to_string());
                    }
                    inside_function = false;
                    pending_instrumentation_end = false;
                }
            } else {
                pending_instrumentation_end = true;
            }
        }
        output.push(line.to_string());

    } 

    let mut file = fs::File::create(output_path)?;
    for line in output {
        writeln!(file, "{}", line)?;
    }

    println!(
        "Instrumentation complete! Output written to '{}'",
        output_path.display()
    );

    Ok(())
}