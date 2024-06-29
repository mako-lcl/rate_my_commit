pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    NoGPTRating,
}

// region: --- Froms
impl From<reqwest::Error> for Error {
    fn from(val: reqwest::Error) -> Self {
        Self::Reqwest(val)
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::Serde(val)
    }
}
// endregion: --- Froms

// region:    --- Error Boiler
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boiler
