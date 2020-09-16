use std::fs::File;
use std::process::{exit, Command, Output};
use std::io::{BufReader, BufRead, BufWriter, Write};

fn main() {
    let input = File::open("input.sh").expect("No input.sh file found!");
    let reader = BufReader::new(input);

    let output = File::create("output.c").expect("Unable to create output.c file!");
    let mut writer = BufWriter::new(output);

    writer.write_all("#include <stdio.h>
#include <stdlib.h>

int main(void) {
".as_bytes()).unwrap();
    for line in reader.lines() {
        writer.write_all(format!("\tsystem(\"{}\");\n", line.unwrap()).as_bytes()).unwrap();
    }

    writer.write_all("\treturn EXIT_SUCCESS;\n}\n".as_bytes()).unwrap();

    Command::new("gcc")
        .args(&[
            "output.c",
            "-O3",
            "-Wall"])
        .spawn()
        .expect("GCC execution failed");
}
