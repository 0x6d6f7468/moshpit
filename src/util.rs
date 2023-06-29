pub fn parse_color(threshold_color: u32) -> (u8, u8, u8, u8) {
    let mut color = threshold_color;
    let (red, green, blue, alpha): (u8, u8, u8, u8);

    if color <= 0xffffff {
        alpha = 0xff;
    } else {
        alpha = (color & 0xff) as u8;
        color >>= 8;
    }
    
    blue = (color & 0xff) as u8;
    color >>= 8;

    green = (color & 0xff) as u8;
    color >>= 8;

    red = (color & 0xff) as u8;

    (red, green, blue, alpha)
}

pub fn clamp_brightness(threshold_brightness: u32) -> u8 {
    let bright: u8;

    if threshold_brightness > 0xff {
        bright = 0xff;
    } else {
        bright = threshold_brightness as u8;
    }

    bright
}
