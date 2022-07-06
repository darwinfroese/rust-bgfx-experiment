use std::error::Error;
use std::{fmt, result};

pub type Result<T = ()> = result::Result<T, PlatformError>;

#[derive(Debug)]
pub enum PlatformError {
    SdlGenericError(String),
    SdlWindowBuildError(sdl2::video::WindowBuildError),
}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlatformError::SdlGenericError(_) => write!(f, "SDL Error"),
            PlatformError::SdlWindowBuildError(ref cause) => {
                write!(f, "SDL Window Build Error: {}", cause)
            }
        }
    }
}

impl Error for PlatformError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            PlatformError::SdlGenericError(_) => None,
            PlatformError::SdlWindowBuildError(ref cause) => Some(cause),
        }
    }
}

impl From<String> for PlatformError {
    fn from(cause: String) -> PlatformError {
        PlatformError::SdlGenericError(cause)
    }
}

impl From<sdl2::video::WindowBuildError> for PlatformError {
    fn from(cause: sdl2::video::WindowBuildError) -> PlatformError {
        PlatformError::SdlWindowBuildError(cause)
    }
}
