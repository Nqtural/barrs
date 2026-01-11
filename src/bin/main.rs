use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};

use barrs::Config;
use barrs::Bar;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "update" {
        let id = &args[2];
        let mut stream = match tokio::net::UnixStream::connect("/tmp/barrs.sock").await {
            Ok(stream) => stream,
            Err(_) => {
                eprintln!("error: could not connect to socket\nIs barrs running?");
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
    let bar = Bar::new(&config);
    bar.start_modules();
    bar.start_command_listener("/tmp/barrs.sock").await;

    loop {
        println!(
            "{}",
            bar.construct().await,
        );
        sleep(Duration::from_secs(1)).await;
    }
}
