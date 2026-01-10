use std::thread;
use std::time::Duration;

use barrs::Config;
use barrs::Bar;

#[tokio::main]
async fn main() {
    let config = match Config::parse() {
        Ok(config) => config,
        Err(e) => {
            println!("error: {e}");
            return;
        }
    };
    let bar = Bar::new(&config);
    bar.start_modules().await;

    loop {
        println!(
            "{}",
            bar.construct().await,
        );
        thread::sleep(Duration::from_secs(1));
    }
}
