use std::thread;
use std::time::Duration;
use g13::*;

const DURATION: Duration = Duration::from_secs(1);

pub fn main() -> Result<(), G13Error>{
    let mut manager = G13Manager::new()?;
    let mut devices = manager.discover()?;

    let device = devices.first_mut().unwrap();

    device.set_key_color((255, 255, 255))?;
    thread::sleep(DURATION);

    device.set_key_color((255, 0, 0))?;
    thread::sleep(DURATION);

    device.set_key_color((0, 255, 0))?;
    thread::sleep(DURATION);

    device.set_key_color((0, 0, 255))?;
    thread::sleep(DURATION);

    device.set_key_color((0, 0, 0))?;
    thread::sleep(DURATION);

    device.set_key_color((255, 255, 255))?;

    Ok(())
}
