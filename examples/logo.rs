use g13::*;

pub fn main() -> Result<(), G13Error>{
    let logo = include_bytes!("./logo.lpbm");

    let mut manager = G13Manager::new()?;
    let mut devices = manager.discover()?;

    let device = devices.first_mut().unwrap();

    device.clear_lcd()?;
    device.write_lcd(logo)?;

    Ok(())
}
