use crate::EResult;
use std::path::Path;

const KMEMLEAK_PATH: &str = "/sys/kernel/debug/kmemleak";

pub struct Session(());

#[derive(Debug)]
pub enum Error {
    NotSupported,
    Other(anyhow::Error),
}

impl Session {
    pub fn start() -> Result<Self, Error> {
        let path = Path::new(KMEMLEAK_PATH);

        if !path.exists() {
            return Err(Error::NotSupported);
        }

        std::fs::write(path, b"clear\n").map_err(|e| Error::Other(e.into()))?;

        Ok(Session(()))
    }

    pub fn finish(self) -> EResult<Option<String>> {
        let path = Path::new(KMEMLEAK_PATH);

        std::fs::write(path, b"scan\n")?;

        // Read the results
        let contents = std::fs::read_to_string(path)?;
        match contents.trim().is_empty() {
            true => Ok(None),
            false => Ok(Some(contents)),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotSupported => write!(f, "kmemleak not supported or not accessible"),
            Error::Other(e) => write!(f, "kmemleak error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NotSupported => None,
            Error::Other(e) => Some(e.as_ref()),
        }
    }
}
