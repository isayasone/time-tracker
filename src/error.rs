#[derive(Debug, thiserror::Error)]
#[error("An application error  has  occurred")]
pub struct AppError;

// A suggestion to help the user fix the error
pub struct Suggestion(pub &'static str);

pub struct ErrorCode(u16);
