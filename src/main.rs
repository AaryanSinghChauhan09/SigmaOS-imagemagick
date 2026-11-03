use std::env;
use log::{info, warn, error, debug};
use anyhow::Result;
use clap::Parser;
use imagemagick::imagemagick;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_hw_accel: bool,
}

fn main() -> Result<()> { -> Result<()> {
    let cli = Cli::parse();
    let config = imagemagick::Config { 
        hardware_acceleration: !cli.no_hw_accel, 
        ..Default::default() 
    };
    let mut component = imagemagick::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-imagemagick ready");
    Ok(())
}

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}
