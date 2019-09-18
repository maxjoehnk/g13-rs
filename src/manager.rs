use libusb::{Context, DeviceHandle};

use crate::{Error, G13};
use crate::consts::{G13_PRODUCT_ID, G13_VENDOR_ID};

pub struct G13Manager {
    context: Context
}

impl G13Manager {
    pub fn new() -> Result<G13Manager, Error> {
        let context = Context::new()?;

        Ok(G13Manager {
            context
        })
    }

    pub fn discover(&mut self) -> Result<Vec<G13>, Error> {
        self.context.devices()?.iter()
            .filter(|device| {
                if let Ok(descriptor) = device.device_descriptor() {
                    descriptor.vendor_id() == G13_VENDOR_ID && descriptor.product_id() == G13_PRODUCT_ID
                } else {
                    false
                }
            })
            .map(G13::new)
            .collect()
    }
}