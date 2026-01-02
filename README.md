# Barrs

_**NOTE:**
Barrs is in very early stages of development, and features outlined in
this document may or may not be implemented yet._

Barrs is a statusbar generator designed to be used together with bars
such as [lemonbar](https://github.com/LemonBoy/bar).

## Installation

```sh
git clone https://github.com/Nqtural/barrs
cd barrs
cargo build --release
sudo cp target/release/main /usr/bin/barrs
```

## Usage

### Configuration

Before running Barrs, you should select which bar you intend to use in
the config file. This ensures that Barrs formats the output correctly
for that bar’s syntax.

The configuration file is located at
`$XDG_CONFIG_HOME/barrs/config.toml` or, if `$XDG_CONFIG_HOME` is not
set, `~/.config/barrs/config.toml`.

In the config, set the frontend, for example:
```toml
frontend = "lemonbar"
```
Barrs will then generate output formatted for the selected bar.

### Running

Running the program depends on what bar is used. All the supported bars
have an example below:

**lemonbar:**
`barrs | lemonbar -p -g x24 -f "SauceCodePro NF:size=10" -F "#ccccee" -B "#0d0d10"`
