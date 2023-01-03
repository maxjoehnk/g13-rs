use std::time::Duration;
use g13::*;

pub fn main() -> Result<(), G13Error>{
    let mut manager = G13Manager::new()?;
    let mut devices = manager.discover()?;

    let device = devices.first_mut().unwrap();

    loop {
        let keys = device.read(Duration::from_secs(10));
        println!("{keys:?}");
    }
}
