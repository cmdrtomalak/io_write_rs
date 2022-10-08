use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_file(filename: &str) -> std::io::Result<String> {
    std::fs::read_to_string(filename)
}

fn write_string(filename: &str, sout: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    write!(file, "{}", sout)?;
    write!(file, "\n")?;
    Ok(())
}

fn read_write(in_file: &str, out_file: &str, rm_text: &str, add_text: &str) -> std::io::Result<()> {
    let text = read_file(in_file).unwrap();
    println!("Text read:\n");
    println!("{text}");
    let text = text.replace(rm_text, add_text);
    let _ = write_string(out_file, &text);
    println!("\nWritten to {out_file}:\n");
    println!("{text}");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let in_file = &args[1];
    let out_file = &args[2];
    let rm_text = &args[3];
    let add_text = &args[4];
    println!("{in_file}");
    let _ = read_write(in_file, out_file, rm_text, add_text);
    Ok(())
}
