use moshpit::{cli, util};

use asdf_pixel_sort::{
    PColor,
    Options,
    Direction,
    Mode,
    sort,
    sort_with_options,
};

use image::ImageError;

fn main() -> Result<(), ImageError> {
    // Parse arguments
    let cmd = cli::parse_args();

    // Load source image into u8 buffer
    let source = image::open(cmd.input).unwrap();
    let mut buffer = source.to_rgb8();

    if cmd.mode.is_none() {
        // No options provided. Sort with defaults.
        sort(&mut buffer);
    } else {
        // Options provided. Proceed with user-provided options.
        let mode: Mode;

        if cmd.threshold.is_none() {
            // If no threshold is provided, use default values from asdf_pixel_sort
            mode = match cmd.mode.unwrap() {
                cli::OptionsMode::BLACK => Mode::black(),
                cli::OptionsMode::BRIGHT => Mode::brightness(),
                cli::OptionsMode::WHITE => Mode::white(),
            };
        } else {
            // Otherwise, use specified threshold values. Clamp to 0xff for brightness,
            //  or parse red, blue, green, and alpha for white or black.
            let thresh = cmd.threshold.unwrap();
            mode = match cmd.mode.unwrap() {
                cli::OptionsMode::BLACK => {
                    let (r, g, b, a) = util::parse_color(thresh);
                    let color = PColor::new(r, g, b).with_alpha(a);
                    Mode::Black(color)
                },
                cli::OptionsMode::BRIGHT => {
                    Mode::Brightness(util::clamp_brightness(thresh))
                },
                cli::OptionsMode::WHITE => {
                    let (r, g, b, a) = util::parse_color(thresh);
                    let color = PColor::new(r, g, b).with_alpha(a);
                    Mode::White(color)
                },
            };
        }

        // Use user-specified direction
        let direction = match cmd.direction.unwrap() {
            cli::OptionsDirection::BOTH => Direction::Both,
            cli::OptionsDirection::COLUMN => Direction::Column,
            cli::OptionsDirection::ROW => Direction::Row,
        };

        // Create options struct from parsed options values
        let options = Options {
            mode,
            direction,
        };

        // Sort with options
        sort_with_options(&mut buffer, &options);
    }

    // Save modified buffer to output file
    buffer.save(cmd.output)?;

    // Return Ok
    Ok(())
}
