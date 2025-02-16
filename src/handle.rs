// 客户端发起请求(查询文件是否存在,并将请求转换成json格式),服务端处理请求,返回结果
// 用户可以通过http请求对文件进行增删改查操作
// 用户可以上传文件,下载文件,删除文件,查询文件是否存在
// 用户可以查询文件的大小,文件的创建时间,文件的修改时间
// 用户可以查询文件的md5值,文件的sha1值,文件的sha256值
// 用户可以打开文件,读取文件,写入文件,关闭文件
// 编辑器可以打开文件,读取文件,写入文件,关闭文件

use crate::command::{ Command,get, create, delete, edit};
use axum::extract::Path;

#[axum::debug_handler]

pub async fn handler(Path((cmd, _path)): Path<(Command, String)>) {
    match cmd {
        Command::Get { path: get_path } => {
            let _ = get(get_path).await;
        }
        Command::Create { path: create_path, content } => {
            let _ = create(create_path, content).await;
        }
        Command::Delete { path: delete_path } => {
            let _ = delete(delete_path).await;
        }
        Command::Edit { path: edit_path, start, end, content } => {
            let _ = edit(edit_path, start, end, content).await;
        }
    }
}

