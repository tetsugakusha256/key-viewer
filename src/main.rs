// use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut input = String::new();
    // open a file for writing (creates or truncates the file)
    let mut file = match File::create("output.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating file: {}", err);
            return;
        }
    };

    loop {
        input.clear(); // Clear the previous input
        stdin
            .lock()
            .read_to_string(&mut input)
            .expect("Failed to read from stdin");
        if input.is_empty() {
            break; // Exit the loop if no more input is provided
        }

        if let Err(err) = writeln!(file, "{}", input) {
            eprintln!("Error writing to file: {}", err);
            return;
        }
        // println!("input: {input}");
        let mut stdout = stdout.lock();
        stdout
            .write_all(input.as_bytes())
            .expect("Failed to write to stdout");
        stdout.flush().expect("Failed to flush stdout");
    }

    // Get the command-line arguments
    // let args: Vec<String> = env::args().collect();


    // Write the command-line arguments to the file
    // for arg in args.iter() {
    //     if let Err(err) = writeln!(file, "{}", arg) {
    //         eprintln!("Error writing to file: {}", err);
    //         return;
    //     }
    // }
}
