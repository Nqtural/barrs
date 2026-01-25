use std::sync::mpsc;
use tokio::io::AsyncWriteExt;

use barrs::Config;
use barrs::Bar;

fn print_usage(executable_name: &str) {
    eprintln!("error: usage:\n{executable_name} [update <id>]");
}

#[tokio::main]
async fn main() {
    // handle update command
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1 {
        if args[1] == "update" {
            if args.len() == 3 {
                let mut stream = match tokio::net::UnixStream::connect("/tmp/barrs.sock").await {
                    Ok(stream) => stream,
                    Err(e) => {
                        eprintln!("error: could not connect to socket: {e}");
                        return;
                    }
                };
                stream.write_all(format!("update {}\n", &args[2]).as_bytes()).await.unwrap();
            } else {
                print_usage(&args[0]);
            }
        } else {
            print_usage(&args[0]);
        }
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
