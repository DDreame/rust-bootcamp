use anyhow::Result;
use std::fs::File;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Player{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    number: u8  // u8, u16, u32, u64, u128,
}

fn main() -> Result<()> {
    let file = File::open("assets/juventus.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize(){
        let player: Player = result?;
        println!("Name: {:?},  Position: {:?},  No.: {:?}",
        player.name, player.position, player.number);
    }
   
    Ok(())
    
}
