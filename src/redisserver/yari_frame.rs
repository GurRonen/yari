use bytes::Bytes;

#[derive(Clone, Debug)]
pub enum YariFrame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<YariFrame>)
}

pub enum FrameError {
    Incomplete,
    Other(crate::Error)
}

impl YariFrame {
    fn new() {
        
    }
}


impl From<String> for FrameError {
    fn from(src: String) -> FrameError {
        FrameError::Other(src.into())
    }
}

impl From<&str> for FrameError {
    fn from(src: &str) -> FrameError {
        src.to_string.into()
    }
}

impl From<FromUtf8Error> for FrameError {
    fn from(_src: FromUtf8Error) -> FrameError {
        "protocol error; Invalid utf8 frame format".into()
    }
}

impl From<TryFromIntError> for FrameError {
    fn from(_src: TryFromIntError) -> FrameError {
        "protocol error; invalid int frame format".into()
    }
}

impl std::error::Error for FrameError {}

impl fmt::Display for FrameError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Incomplete => "Stream ended early".fmt(fmt),
            Error::Other(err) => err.fmt(fmt),
        }
    }
}