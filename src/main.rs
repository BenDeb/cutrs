use clap::Parser;
use cutrs::{run, Cli, Config};
fn main() {
    let args = Cli::parse();

    let conf = Config::build(args);
    run(conf);
}
