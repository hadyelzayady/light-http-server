use std::{fmt, str::FromStr};

#[derive(Debug)]
pub struct HttpVersion(&'static str);
pub struct InvalidHttpVersion {
    _priv: (),
}

impl HttpVersion {
    pub const HTTP1_1: HttpVersion = HttpVersion("HTTP/1.1");
    pub const HTTP2: HttpVersion = HttpVersion("HTTP/2");
}

impl ToString for HttpVersion {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for HttpVersion {
    fn from_str(s: &str) -> Result<Self, InvalidHttpVersion> {
        match s {
            "HTTP/1.1" => Ok(HttpVersion::HTTP1_1),
            "HTTP/2" => Ok(HttpVersion::HTTP2),
            _ => Err(InvalidHttpVersion::new()),
        }
    }

    type Err = InvalidHttpVersion;
}

impl InvalidHttpVersion {
    fn new() -> InvalidHttpVersion {
        InvalidHttpVersion { _priv: () }
    }
}
impl fmt::Debug for InvalidHttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InvalidHttpVersion").finish()
    }
}
//impl Error for InvalidHttpVersion {}
