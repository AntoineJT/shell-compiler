use std::fs::File;
use std::process::{Command, exit};
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::env;

// TODO Instead of compiling in C, pack the original file in a binary one which will run it
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Error: No input file specified!
Format: {} <input_file> [output_no_ext]", &args[0]);
        exit(1);
    }

    let script_name = &args[1];
    let c_name = &if args.len() == 3 {
        args[2].clone()
    } else {
        let filename = Filename { fname: String::from(script_name) }.remove_extension();
        format!("{}.c", filename.to_string())
    };

    generate_c_code(script_name, c_name);
    compile_with_gcc(c_name);
}

fn generate_c_code(input: &str, output: &str) {
    let input = File::open(input)
        .expect(format!("No {} file found!", input).as_str());
    let reader = BufReader::new(input);

    let output = File::create(output)
        .expect(format!("Unable to create {} file!", output).as_str());
    let mut writer = BufWriter::new(output);

    writer.write_all("#include <stdio.h>
#include <stdlib.h>

int main(void) {
".as_bytes()).unwrap();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = &line.trim();
        // TODO Find a way to handle heredocs, meanwhile, at least print an error when found one
        if line.is_empty()  // empty lines
            || line.starts_with("#") // bash comments
            || line.starts_with("REM") // windows batch comments
        {
            continue;
        }
        writer.write_all(format!("\tsystem(\"{}\");\n", line).as_bytes()).unwrap();
    }

    writer.write_all("\treturn EXIT_SUCCESS;\n}\n".as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn compile_with_gcc(input: &str) {
    Command::new("gcc")
        .arg(input)
        .arg("-O3")
        .spawn()
        .expect("GCC execution failed");
}

struct Filename {
    fname: String
}

impl Filename {
    /*
    fn reverse_filename(&mut self) {
        let mut new_value = String::with_capacity(self.fname.len());
        for _ in self.fname.len() {
            new_value.push(self.fname.pop().unwrap())
        }
        self.fname = new_value
    }
     */

    /*
    fn remove_extension_mut(&mut self) {
        if !self.fname.contains('.') {
            return
        }
        loop {
            let c = self.fname.pop().unwrap();
            if c == '.' {
                break
            }
        }
    }
    */

    fn remove_extension(&self) -> Filename {
        let mut fname = self.fname.clone();
        if !self.fname.contains('.') {
            return Filename { fname }
        }
        loop {
            let c = fname.pop().unwrap();
            if c == '.' {
                break
            }
        }
        Filename { fname }
    }

    /*
    fn add_binary_extension_mut(&mut self) {
        if cfg!(windows) {
            self.fname.push_str(".exe");
            return
        }
        if cfg!(linux) {
            return
        }
        else
        {
            eprintln!("Platform not supported for now. \
            Generated file, if supported by the compiler, will not have any extension.")
        }
    }
    */

    fn add_binary_extension(&self) -> Filename {
        let mut fname = self.fname.clone();

        if cfg!(windows) {
            fname.push_str(".exe");
        }

        if !cfg!(windows) && !cfg!(linux) {
            eprintln!("Platform not supported for now. \
            Generated file, if supported by the compiler, will not have any extension.");
        }
        Filename { fname }
    }

    fn to_string(&self) -> &str {
        &self.fname
    }
}
