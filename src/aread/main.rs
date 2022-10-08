use tokio::{
    fs::File,
    io::{
        AsyncBufReadExt,
        BufReader
    }
};

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let in_file = &args[1];

    let file = File::open(in_file).await.expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.expect("Failed to read file") {
        println!("{line}");
    }
}