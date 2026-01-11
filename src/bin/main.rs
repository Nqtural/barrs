use tokio::time::{Duration, sleep};

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
    bar.start_modules();

    loop {
        println!(
            "{}",
            bar.construct().await,
        );
        sleep(Duration::from_secs(1)).await;
    }
}
