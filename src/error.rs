#[cfg(feature = "parse")]
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("parse error: {0}")]
	ParseError(String),
}
