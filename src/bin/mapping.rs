use std::error::Error;

use tokio::fs::File;

struct Config {

}

impl Config {

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("Mapping.toml").await?;
//    println!("fuck you: {}");
    Ok(())
}