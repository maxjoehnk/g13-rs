use thiserror::Error;

#[derive(Debug, Error)]
pub enum G13Error {
    #[error("HID error: {0}")]
    Transport(#[from] hidapi::HidError),
    #[error("Invalid key response")]
    InvalidKeyResponse,
    #[error("Problem sending data. Result: {0}")]
    ProblemSendingData(usize),
    #[error("Invalid lcd buffer size: Got {0}, expected {1}")]
    InvalidLcdBufferSize(usize, usize)
}
