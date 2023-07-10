use std::fmt;

pub enum Status {
    Ok = 200,
    NotFound = 404,
    ServerError = 500,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", unsafe { *(self as *const Self as *const u8) })
    }
}

pub fn generate_header(status: Status) -> String {
    format!("HTTP/1.1 {} {}", status, status.to_string())
}
