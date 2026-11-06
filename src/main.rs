use ::imagemagick::{imagemagick, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = imagemagick::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.run().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-imagemagick operational");
    Ok(())
}
