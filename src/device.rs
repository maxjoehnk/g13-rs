use crate::Error;
use libusb::{DeviceHandle, Direction, RequestType, Recipient};
use libusb_sys::{LIBUSB_REQUEST_TYPE_CLASS, LIBUSB_RECIPIENT_INTERFACE, LIBUSB_ENDPOINT_OUT};
use std::time::Duration;
use crate::consts::{G13_LCD_ENDPOINT, G13_LCD_BUF_SIZE, G13_LCD_BUFFER_SIZE};

pub struct G13<'a> {
    handle: DeviceHandle<'a>
}

impl<'a> G13<'a> {
    pub(crate) fn new(device: libusb::Device) -> Result<G13, Error> {
        let mut handle = device.open()?;
        if handle.kernel_driver_active(0)? {
            handle.detach_kernel_driver(0)?;
        }
        handle.claim_interface(0)?;

        handle.write_control(0, 9, 1, 0, &vec![], Duration::from_millis(1000))?;
        let mut device = G13 {
            handle
        };

        Ok(device)
    }

    pub fn clear_lcd(&mut self) -> Result<(), Error> {
        let mut buf = [0; G13_LCD_BUFFER_SIZE];
        self.write_lcd(&mut buf)
    }

    pub fn write_lcd(&mut self, buffer: &[u8]) -> Result<(), Error> {
        assert_eq!(buffer.len(), G13_LCD_BUFFER_SIZE);
        let mut buf = vec![0; 32];
        buf.extend_from_slice(buffer);
        buf[0] = 0x03;
        self.handle.write_interrupt(LIBUSB_ENDPOINT_OUT | G13_LCD_ENDPOINT, &buf, Duration::from_millis(1000))?;
        Ok(())
    }

    pub fn set_key_color(&mut self, color: (u8, u8, u8)) -> Result<(), Error> {
        let data = vec![5, color.0, color.1, color.2, 0];
        let result = self.handle.write_control(LIBUSB_REQUEST_TYPE_CLASS | LIBUSB_RECIPIENT_INTERFACE, 9, 0x307, 0, &data, Duration::from_millis(1000))?;
        assert_eq!(result, 5);
        Ok(())
    }
}

impl<'a> std::fmt::Debug for G13<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "G13 {{}}")
    }
}