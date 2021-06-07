use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("Io error: {0}")]
	Io(#[from] std::io::Error),

	#[error("Restart from jsonrpsee")]
	RestartFromJsonrpsee,
}
