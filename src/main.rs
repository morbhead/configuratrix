mod args;
mod sudo;
mod internal;

use clap::Parser;

use args::Args;
use internal::types::*;

fn main() {
    let args = Args::parse();

    let config_path = args.config_path;
    let config_type = args.config_type;
    let options = args.options;

    println!("{}", config_path);
    println!("{}", config_type);

    if config_type == "sudo" {
        sudo::construct(config_path, options);
    }

}