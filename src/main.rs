use anyhow::Result;
use clap::Parser;
use imagemagick::imagemagick;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_hw_accel: bool,
}

fn main() -> Result<()> {
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
