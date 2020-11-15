use chrono::Utc;

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Headers {
    headers: Vec<Header>
}

impl Headers {
    pub fn new() -> Self {
        let default_headers = generate_default_headers();
        Headers{ headers: default_headers }
    }
}

impl Display for Headers {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        for header in &self.headers {
            write!(fmt, "{}\n", header)?;
        }
        Ok(())
    }
}

/// Responsible for generating default headers
fn generate_default_headers() -> Vec<Header> {
    vec![generate_date_header()]
}

fn generate_date_header() -> Header {
    let now = Utc::now();
    Header::new("Date", now.to_rfc2822())
}


#[derive(Debug)]
struct Header {
    key: &'static str,
    value: String
}

impl Header {
    fn new(key: &'static str, value: String) -> Self {
        Header{ key, value }
    }
}

impl Display for Header {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(fmt, "{}: {}", self.key, self.value)
    }
}