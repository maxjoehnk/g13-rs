pub const G13_INDEX: u16 = 0;
pub const G13_KEY_ENDPOINT: u8 = 1;
pub const G13_LCD_ENDPOINT: u8 = 2;
pub const G13_VENDOR_ID: u16 = 0x046d;
pub const G13_PRODUCT_ID: u16 = 0xc21c;
pub const G13_REPORT_SIZE: usize = 8;
pub const G13_LCD_COLUMNS: usize = 160;
pub const G13_LCD_ROWS: usize = 48;
pub const G13_LCD_BYTES_PER_ROW: usize = G13_LCD_COLUMNS/8;
pub const G13_LCD_BUFFER_SIZE: usize = G13_LCD_ROWS * G13_LCD_BYTES_PER_ROW;

pub const G13_SET_KEY_COLOR: u16 = 0x307;
pub const G13_SET_MODE_LEDS: u16 = 0x305;
