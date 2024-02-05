use std::{error::Error, fmt::{Debug, Display}};


pub enum DbError {
    CreateError,
    ReadError,
    UpdateError,
    DeleteError,
}

impl Error for DbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
}

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}
impl Debug for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateError => write!(f, "CreateError"),
            Self::ReadError => write!(f, "ReadError"),
            Self::UpdateError => write!(f, "UpdateError"),
            Self::DeleteError => write!(f, "DeleteError"),
        }
    }
}