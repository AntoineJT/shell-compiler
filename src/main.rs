use std::fs::File;
use std::process::{exit, Command, Output};
use std::io::{BufReader, BufRead, BufWriter, Write};

fn main() {
    let input = File::open("input.sh").expect("No input.sh file found!");
    let reader = BufReader::new(input);

    let output = File::create("output.c").expect("Unable to create output.c file!");
    let mut writer = BufWriter::new(output);

    writer.write_all("#include <stdio.h>\n\
        #include <stdlib.h>\n
        \n\
        int main(void) {\n".as_bytes()).unwrap();
    for line in reader.lines() {
        writer.write_all(format!("\tsystem(\"{}\");\n", line.unwrap()).as_bytes()).unwrap();
    }

    writer.write_all("\treturn EXIT_SUCCESS;\n}\n".as_bytes()).unwrap();
    /*
    let output = Command::new("gcc")
        .args(&[
            "output.c",
            "-O3",
            "-Wall"])
        .output();
    match output {
        Ok(o) => println!("Output: {}\nScript successfully compiled!", get_command_output(o)),
        Err(e) => {
            println!("Failed to compile! Check if gcc is installed on your system!\nError: {}", e.to_string());
            exit(1);
        },
    }
     */
}

/*
fn get_command_output(out: Output) -> String {
    let stream = if out.status.success() {
        out.stdout
    } else {
        out.stderr
    };
    String::from_utf8(stream).unwrap()
}
*/
