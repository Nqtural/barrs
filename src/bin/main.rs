use anyhow::Result;
use std::thread;
use std::time::Duration;

use lemonbar_rs::Config;
use lemonbar_rs::Bar;

fn main() -> Result<()> {
    let config = Config::parse()?;
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
