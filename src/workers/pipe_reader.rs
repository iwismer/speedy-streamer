use crate::models::Message;
use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::sync::mpsc::Sender;

/// Receives reads from the reader, then forwards them to the client pool.
#[derive(Debug)]
pub struct PipeReader {
    chip_read_bus: Sender<Message>,
}

impl PipeReader {
    pub fn new(chip_read_bus: Sender<Message>) -> Self {
        PipeReader { chip_read_bus }
    }

    /// Start listening for reads.
    ///
    /// This function should never return.
    pub async fn begin(&mut self) {
        #[allow(unused_assignments)]
        let mut f = File::open("/tmp/reader").await;
        // Wait until it can open the named pipe
        loop {
            f = match File::open("/tmp/reader").await {
                Ok(file) => Ok(file),
                Err(_) => continue,
            };
            break;
        }
        let bufreader = BufReader::new(f.unwrap());
        let mut lines = bufreader.lines();
        while let Ok(line) = lines.next_line().await {
            if line.is_none() {
                continue;
            }
            let read = line.unwrap() + "\n";
            self.chip_read_bus
                .send(Message::CHIP_READ(read.to_owned()))
                .await
                .unwrap_or_else(|_| {
                    println!(
                        "\r\x1b[2KError sending read to thread. Maybe no readers are connected?"
                    );
                });
        }
    }
}
