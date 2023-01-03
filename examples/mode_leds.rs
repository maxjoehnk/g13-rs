use std::thread;
use std::time::Duration;
use g13::*;

const DURATION: Duration = Duration::from_secs(1);

pub fn main() -> Result<(), G13Error>{
    let mut manager = G13Manager::new()?;
    let mut devices = manager.discover()?;

    let device = devices.first_mut().unwrap();

    device.set_mode_leds(ModeLeds::empty())?;
    thread::sleep(DURATION);

    device.set_mode_leds(ModeLeds::M1)?;
    thread::sleep(DURATION);

    device.set_mode_leds(ModeLeds::M2)?;
    thread::sleep(DURATION);

    device.set_mode_leds(ModeLeds::M3)?;
    thread::sleep(DURATION);

    device.set_mode_leds(ModeLeds::MR)?;
    thread::sleep(DURATION);

    device.set_mode_leds(ModeLeds::all())?;
    thread::sleep(DURATION);

    device.set_mode_leds(ModeLeds::empty())?;

    Ok(())
}
