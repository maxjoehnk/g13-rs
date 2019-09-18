use g13::*;

pub fn main() -> Result<(), Error>{
    let logo = include_bytes!("./logo.lpbm");

    let mut manager = G13Manager::new()?;
    let mut devices = manager.discover()?;

    let device = devices.first_mut().unwrap();

    device.set_key_color((255, 255, 255))?;

    device.clear_lcd()?;
    device.write_lcd(logo)?;

    Ok(())
}