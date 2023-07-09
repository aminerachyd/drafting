use std::env::args;

use drafting::*;

fn main() {
    let mut args = args();
    let subcommand = args.nth(1);
    run(subcommand);
}
