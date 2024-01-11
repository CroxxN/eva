#[derive(Debug)]
pub enum ParseError {
    BadMagic,
    UnsupportedClass,
    UnsupportedEndianess,
    UnsupportedVersion,
    Utf8Error(core::str::Utf8Error),
    TryFromSliceError(core::array::TryFromSliceError),
    TryFromIntError(core::num::TryFromIntError),
    IOError(std::io::Error),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::BadMagic => {
                write!(f, "\x1b[1;31mError: Bad magic\x1b[0m")
            }
            Self::UnsupportedClass => {
                write!(f, "\x1b[1;31mError: Unsupported ELF class\x1b[0m")
            }
            Self::UnsupportedEndianess => {
                write!(f, "\x1b[1;31mError: Unsupported ELF Endianess\x1b[0m")
            }
            Self::UnsupportedVersion => {
                write!(f, "\x1b[1;31mError: Unsupported ELF format version\x1b[0m")
            }
            Self::Utf8Error(e) => {
                write!(f, "\x1b[1;31mError: Parsing utf8: {e}\x1b[0m")
            }
            Self::TryFromSliceError(e) => {
                write!(f, "\x1b[1;31mError: Parsing slice: {e}\x1b[0m")
            }
            Self::TryFromIntError(e) => {
                write!(f, "\x1b[1;31mError: Parsing integer: {e}\x1b[0m")
            }
            Self::IOError(e) => {
                write!(f, "\x1b[1;31mError: IO error: {e}\x1b[0m")
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ParseError::Utf8Error(ref e) => Some(e),
            ParseError::TryFromSliceError(ref e) => Some(e),
            ParseError::TryFromIntError(ref e) => Some(e),
            ParseError::IOError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> ParseError {
        ParseError::IOError(err)
    }
}
