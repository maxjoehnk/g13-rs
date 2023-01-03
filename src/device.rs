use std::time::Duration;

use rusb::{Context, Device, DeviceHandle, Direction, Recipient, request_type, RequestType};
use rusb::constants::*;

use crate::consts::*;
use crate::error::G13Error;
use crate::flags::{Keys, ModeLeds};

pub struct G13 {
    handle: DeviceHandle<Context>,
}

impl G13 {
    pub(crate) fn new(device: Device<Context>) -> Result<G13, G13Error> {
        let mut handle = device.open()?;
        if handle.kernel_driver_active(0)? {
            handle.detach_kernel_driver(0)?;
        }
        handle.claim_interface(0)?;

        handle.write_control(0, 9, 1, 0, &[], Duration::from_millis(1000))?;
        let device = G13 {
            handle
        };

        Ok(device)
    }

    pub fn clear_lcd(&mut self) -> Result<(), G13Error> {
        let buf = [0; G13_LCD_BUFFER_SIZE];
        self.write_lcd(&buf)
    }

    pub fn write_lcd(&mut self, buffer: &[u8]) -> Result<(), G13Error> {
        if buffer.len() != G13_LCD_BUFFER_SIZE {
            return Err(G13Error::InvalidLcdBufferSize(buffer.len(), G13_LCD_BUFFER_SIZE));
        }

        let mut buf = vec![0; 32];
        buf.extend_from_slice(buffer);
        buf[0] = 0x03;
        self.handle.write_interrupt(LIBUSB_ENDPOINT_OUT | G13_LCD_ENDPOINT, &buf, Duration::from_millis(1000))?;

        Ok(())
    }

    pub fn set_key_color(&mut self, color: (u8, u8, u8)) -> Result<(), G13Error> {
        let data = vec![5, color.0, color.1, color.2, 0];
        let result = self.handle.write_control(request_type(Direction::Out, RequestType::Class, Recipient::Interface), 9, G13_SET_KEY_COLOR, G13_INDEX, &data, Duration::from_millis(1000))?;

        if result != 5 {
            return Err(G13Error::ProblemSendingData(result));
        }

        Ok(())
    }

    pub fn set_mode_leds(&mut self, leds: ModeLeds) -> Result<(), G13Error> {
        let data = vec![5, leds.bits(), 0, 0, 0];
        let result = self.handle.write_control(request_type(Direction::Out, RequestType::Class, Recipient::Interface), 9, G13_SET_MODE_LEDS, G13_INDEX, &data, Duration::from_millis(1000))?;

        if result != 5 {
            return Err(G13Error::ProblemSendingData(result));
        }

        Ok(())
    }

    pub fn read(&self, timeout: Duration) -> Result<Response, G13Error> {
        let mut data = [0; G13_REPORT_SIZE];
        self.handle.read_interrupt(LIBUSB_ENDPOINT_IN | G13_KEY_ENDPOINT, &mut data, timeout)?;

        let mut value: u64 = data[7] as u64;
        value <<= 8;
        value += data[6] as u64;
        value <<= 8;
        value += data[5] as u64;
        value <<= 8;
        value += data[4] as u64;
        value <<= 8;
        value += data[3] as u64;

        log::trace!("{value:#010b}");

        let keys = Keys::from_bits_truncate(value);

        let x = data[1] as f32 / u8::MAX as f32;
        let y = data[2] as f32 / u8::MAX as f32;

        Ok(Response {
            keys,
            joystick: (x, y),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Response {
    pub keys: Keys,
    pub joystick: (f32, f32),
}

impl std::fmt::Debug for G13 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "G13 {{}}")
    }
}
