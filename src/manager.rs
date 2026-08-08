use hidapi::HidApi;

use crate::{G13Error, G13};
use crate::consts::{G13_PRODUCT_ID, G13_VENDOR_ID};

pub struct G13Manager {
    hid_api: HidApi
}

impl G13Manager {
    pub fn new() -> Result<G13Manager, G13Error> {
        Ok(G13Manager {
            hid_api: HidApi::new()?
        })
    }

    pub fn discover(&mut self) -> Result<Vec<G13>, G13Error> {
        self.hid_api.device_list()
            .filter(|device| device.vendor_id() == G13_VENDOR_ID && device.product_id() == G13_PRODUCT_ID)
            .map(|device| G13::new(&self.hid_api, device))
            .collect()
    }
}
