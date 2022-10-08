use tokio::{
    fs::File,
    io::{self, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let in_file = &args[1];
    let data = b"Something to jot down";

    let mut pos = 0;
    let mut buffer = File::create(in_file).await.expect("Failed to create file");

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]).await?;
        pos += bytes_written;
    }

    Ok(())
}
