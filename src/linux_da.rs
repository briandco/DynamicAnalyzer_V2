use std::fs;
use std::io::{self, Write};
use std::path::Path;

const INSTRUMENTATION_BEGIN: &str = r#"struct mallinfo mi_start =  mallinfo();
int start_var;

///////////////// Wall Time ////////////////////////
// Start measuring time
struct timespec begin_wall, end_wall; 
clock_gettime(CLOCK_REALTIME, &begin_wall);

///////////////// CPU Time /////////////////////////
// Start measuring time
struct timespec begin_CPU, end_CPU; 
clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &begin_CPU);"#;

const INSTRUMENTATION_END: &str = r#"int end_var;

struct mallinfo mi_end = mallinfo();
cout << "\nStack used = " << ((reinterpret_cast<intptr_t>(&end_var))-(reinterpret_cast<intptr_t>(&start_var)) - 4);
cout << "\nHeap used = " << (mi_end.uordblks - mi_start.uordblks) << " bytes \n";

//////////////// Wall Time ///////////////
clock_gettime(CLOCK_REALTIME, &end_wall);
long seconds = end_wall.tv_sec - begin_wall.tv_sec;
long nanoseconds = end_wall.tv_nsec - begin_wall.tv_nsec;

// Convert the elapsed time to microseconds
double elapsed = (seconds * 1e6) + (nanoseconds * 1e-3);

printf("Wall Time measured: %.3f microseconds.\n", elapsed);

//////////////// CPU Time ///////////////
clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &end_CPU);
long seconds_CPU = end_CPU.tv_sec - begin_CPU.tv_sec;
long nanoseconds_CPU = end_CPU.tv_nsec - begin_CPU.tv_nsec;

// Convert the elapsed time to microseconds
double elapsed_CPU = (seconds_CPU * 1e6) + (nanoseconds_CPU * 1e-3);

printf("CPU Time measured: %.3f microseconds.\n\n", elapsed_CPU);"#;

/// Instruments a `.cpp` file by adding code at the beginning and end of functions
pub(crate) fn instrument_cpp_file(input_path: &Path, output_path: &Path) -> io::Result<()> {
    let content = fs::read_to_string(input_path)?;
    let mut output = Vec::new();

    let mut inside_function = false;
    let mut waiting_for_braces = false;
    let mut brace_count = 0;
    let mut includes_inserted = false; // Tracks if additional includes were inserted
    let mut pending_instrumentation_end = false;

    const ADDITIONAL_INCLUDES: &str = r#"#include <malloc.h>
#include <chrono>
#include <time.h>
#include <ctime>"#;

    for line in content.lines() {
        let trimmed_line = line.trim();

        // Detect #include <iostream> and insert additional includes
        if trimmed_line == "#include <iostream>" && !includes_inserted {
            output.push(line.to_string()); // Push #include <iostream>
            output.push(ADDITIONAL_INCLUDES.to_string()); // Insert additional includes
            includes_inserted = true;
            continue;
        }

        // Scenario 1: Function header and { on the same line
        if trimmed_line.ends_with("{") && trimmed_line.contains(")") && !inside_function {
            inside_function = true;
            brace_count = 1;

            let parts: Vec<&str> = trimmed_line.splitn(2, "{").collect();
            output.push(parts[0].to_string() + "{");
            output.push(INSTRUMENTATION_BEGIN.to_string());

            if parts.len() > 1 {
                output.push(parts[1].to_string());
            }
            continue;
        }

        // Scenario 2: Function header ends with ')' but { is on the next line
        if trimmed_line.ends_with(")") && !inside_function {
            waiting_for_braces = true;
            output.push(line.to_string());
            continue;
        }

        // Wait for '{' on the next line
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
            // Track nested braces to determine function end
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
        // Add the current line to output
        output.push(line.to_string());
    }

    // Write the instrumented code to the new file
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
