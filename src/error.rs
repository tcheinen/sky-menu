use core::fmt;
use std::error::Error;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum LauncherError {
    ResolveIconThemeError,
}

impl fmt::Display for LauncherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LauncherError::ResolveIconThemeError => "Unable to resolve icon theme",
            }
        )
    }
}

impl Error for LauncherError {}
