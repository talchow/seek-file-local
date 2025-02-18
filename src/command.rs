use serde::Deserialize;


#[derive(Deserialize)]
pub enum Command {
    Get {
        path: String,
    },
    Create {
        path: String,
        content: String,
    },
    Delete {
        path: String,
    },
    Edit {
        path: String,
        start: usize,
        end: usize,
        content: String,
    },
}


