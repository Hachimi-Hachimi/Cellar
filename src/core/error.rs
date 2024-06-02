use std::fmt;

#[derive(Debug)]
pub enum Error {
    HookingError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HookingError(e) => {
                write!(f, "Hooking failed: {}", e)
            }
        }
    }
}