use std::error::Error;
use std::{fmt, result};

pub type Result<T = ()> = result::Result<T, PlatformError>;

#[derive(Debug)]
pub enum PlatformError {
    SdlGenericError(String),
    SdlWindowBuildError(sdl2::video::WindowBuildError),

    EcsMaxEntitiesError(),
}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlatformError::SdlGenericError(_) => {
                write!(f, "SDL Error! A generic SDL error occurred")
            }
            PlatformError::SdlWindowBuildError(ref cause) => {
                write!(f, "SDL Window Build Error: {}", cause)
            }
            PlatformError::EcsMaxEntitiesError() => {
                write!(f, "Max of {} entities reached", std::u16::MAX)
            }
        }
    }
}

impl Error for PlatformError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            PlatformError::SdlGenericError(_) => None,
            PlatformError::SdlWindowBuildError(ref cause) => Some(cause),
            PlatformError::EcsMaxEntitiesError() => None,
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
