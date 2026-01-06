use std::thread;
use std::time::Duration;

use barrs::Config;
use barrs::Bar;

fn main() {
    let config = match Config::parse() {
        Ok(config) => config,
        Err(e) => {
            println!("error: {e}");
            return;
        }
    };
    let mut bar = Bar::new(&config);

    loop {
        bar.update();
        println!(
            "{}",
            bar.construct(),
        );
        thread::sleep(Duration::from_secs(1));
    }
}
