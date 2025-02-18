// 客户端发起请求(查询文件是否存在,并将请求转换成json格式),服务端处理请求,返回结果
// 用户可以通过http请求对文件进行增删改查操作
// 用户可以上传文件,下载文件,删除文件,查询文件是否存在
// 用户可以查询文件的大小,文件的创建时间,文件的修改时间
// 用户可以查询文件的md5值,文件的sha1值,文件的sha256值
// 用户可以打开文件,读取文件,写入文件,关闭文件
// 编辑器可以打开文件,读取文件,写入文件,关闭文件

use crate::command::Command;
use crate::error::AppError;
use axum::extract::Path;
use axum::response::IntoResponse;
use tokio::fs::{read_to_string, remove_file, write};

#[axum::debug_handler]

pub async fn handler(Path((cmd, _ )): Path<(Command, String)>) ->Result<impl IntoResponse, AppError> {
    match cmd {
        Command::Get { path } => {
             let content = read_to_string(path).await?;
             println!("{}", content);
             Ok("Get operation succeed")
        }
        Command::Create { path , content } => {
             write(path, content).await?;
             println!("Create operation succeed");
             Ok("Create operation succeed")
        }
        Command::Delete { path } => {
              remove_file(path).await?;
              println!("Delete operation succeed");
              Ok("Delete operation succeed")
        }
        Command::Edit { path , start, end, content } => {
              let mut file_content = read_to_string(path.clone()).await?;
              file_content.replace_range(start..end, &content);
                write(path, file_content).await?;
              Ok("Edit operation succeed")
        }
    }
}

