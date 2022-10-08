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

fn main() -> std::io::Result<()> {
    let text = read_file("hello.txt").unwrap();
    let _ = write_string("write2.txt", &text);
    Ok(())
}
