use anyhow::Result;
use std::fs::File;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Player{
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub number: u8  // u8, u16, u32, u64, u128,
}

fn main() -> Result<()> {
    let file = File::open("assets/juventus.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize(){
        let player: Player = result?;
        // println!("Name: {:?},  Position: {:?},  No.: {:?}",
        // player.name, player.position, player.number);
        println!("{:?}", player.to_json()?)
    }
   
    Ok(())
    
}


impl Player {
    // &self, &mut self, self
    pub fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(json: &str) -> Result<Player>{
        let player: Player = serde_json::from_str(json)?;
        Ok(player)
    }
}