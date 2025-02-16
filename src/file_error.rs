enum FileError {
    IoError(String),
    InvalidPath,
    ParseError,
    LockConflict,
}