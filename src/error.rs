#[derive(Debug)]
pub enum Error {
    Libusb(libusb::Error)
}

impl From<libusb::Error> for Error {
    fn from(err: libusb::Error) -> Self {
        Error::Libusb(err)
    }
}