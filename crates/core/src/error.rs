use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid data: {0}")]
    Invalid(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("toolchain unavailable: {0}")]
    Toolchain(String),
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl serde::Serialize for Error {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_serialize_as_messages() {
        let e = Error::NotFound("two-sum".into());
        assert_eq!(serde_json::to_string(&e).unwrap(), "\"not found: two-sum\"");
        let io: Error = std::io::Error::other("boom").into();
        assert!(io.to_string().contains("boom"));
        let json: Error = serde_json::from_str::<u8>("x").unwrap_err().into();
        assert!(json.to_string().starts_with("json error"));
        let db: Error = rusqlite::Error::InvalidQuery.into();
        assert!(db.to_string().starts_with("database error"));
        assert!(Error::Toolchain("python3".into())
            .to_string()
            .contains("python3"));
    }
}
