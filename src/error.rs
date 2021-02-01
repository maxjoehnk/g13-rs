use thiserror::Error;

#[derive(Debug, Error)]
pub enum G13Error {
    #[error("Usb error: {0}")]
    Libusb(#[from] rusb::Error)
}
