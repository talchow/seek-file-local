use serde::Deserialize;

#[derive(Deserialize)]
pub enum Command {
    Get{path:String},
    Create{path:String, content:String},
    Delete{path:String},
    Edit{path:String, start:usize , end:usize , content:String },
}


use std::fs::{ read_to_string, remove_file, write };

use anyhow::Error;

pub async fn get(path:String) -> Result<(), Error> {
    let content: String = read_to_string(path)?;
    println!("{}", content);
    Ok(())  
}

pub async fn create(path:String, content:String) -> Result<(), Error> {
    write(path, content)?;
    println!("File created");
    Ok(())
}

pub async fn delete(path:String) -> Result<(), Error> {
    remove_file(path)?;
    println!("File deleted");
    Ok(())
}

pub async fn edit(path:String, start:usize , end:usize , content:String) -> Result<(), Error> {
    let mut file_content: String = read_to_string(&path)?;
    file_content.replace_range(start..end, &content);
    write(path.clone(), file_content)?;
    println!("File edited");
    Ok(())
}