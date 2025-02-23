use axum::extract::Path;
use tokio::fs::{self, try_exists};
use crate::error::AppError;

pub async fn get_file(Path(path): Path<String>) -> Result<String, AppError> {
        if !try_exists(path.clone()).await? {
            return Err(AppError::InvalidParameters(format!("File does not exist: {}", path)));
        }else {
            let content = fs::read_to_string(path).await?;
            println!("File content: {}", content);
            Ok(content)
}
}
pub async fn create_file(Path(path): Path<String>, content: String) -> Result<String, AppError> {
    if try_exists(path.clone()).await? {
        return Err(AppError::InvalidParameters(format!("File already exists: {}", path)));
    }else {
        fs::write(path, content).await?;  
        Ok("File created successfully".to_string())
    }
}

use axum::extract::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateCommand {
    pub start: usize,
    pub end: usize,
    pub content: String,
}
#[axum::debug_handler]
pub async fn update_file(Path(path): Path<String>, Json(cmd): Json<UpdateCommand>) -> Result<String, AppError> {
   if !try_exists(path.clone()).await? {
        return Err(AppError::InvalidParameters(format!("File does not exist: {}", path)));
   }else {
       let content = fs::read_to_string(&path).await?;
       if cmd.start > content.len() || cmd.end > content.len() || cmd.start > cmd.end {
           return Err(AppError::IndexError("Out of range".into()));
       }
       
       let mut re_content = content;
       re_content.replace_range(cmd.start..cmd.end, &cmd.content);
       fs::write(&path, re_content).await?;
       Ok("File updated successfully".to_string())
   }
} 

pub async fn delete_file(Path(path): Path<String>) -> Result<String, AppError> {
    if !try_exists(path.clone()).await? {
      return Err(AppError::InvalidParameters(format!("File does not exist: {}", path)));
    }else {
        fs::remove_file(path).await?;
        Ok("File deleted successfully".to_string())
    }
}