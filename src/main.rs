use clap::{Parser, Subcommand, ValueHint};

#[derive(Parser, Debug)]
pub struct Opts {
    #[command(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    #[clap(visible_aliases = &["--max-int", "maxi"])]
    MaxInt {
        #[clap(default_value = "uint256")]
        value: String,
    },
}

fn main() {
    let opts = Opts::parse();
    match opts.sub {
        Subcommands::MaxInt { value } => {
            println!("{}", value)
        }
    }

    let debug_enabled = std::env::var("FOUNDRY_DEBUG").is_ok();
    println!("{:?}", debug_enabled);
}
