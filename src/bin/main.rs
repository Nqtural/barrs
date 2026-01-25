use std::sync::mpsc;
use tokio::io::AsyncWriteExt;

use barrs::Config;
use barrs::Bar;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "update" {
        let id = &args[2];
        let mut stream = match tokio::net::UnixStream::connect("/tmp/barrs.sock").await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("error: could not connect to socket: {e}");
                return;
            }
        };
        stream.write_all(format!("update {id}\n").as_bytes()).await.unwrap();
        return;
    }

    let config = match Config::parse() {
        Ok(config) => config,
        Err(e) => {
            println!("error: {e}");
            return;
        }
    };
    let (tx, rx) = mpsc::channel();
    let bar = Bar::new(&config, tx);
    bar.start_modules();
    bar.start_command_listener("/tmp/barrs.sock").await;

    loop {
        println!(
            "{}",
            bar.construct().await,
        );

        // wait for any module to send update signal
        rx.recv().unwrap();
    }
}
