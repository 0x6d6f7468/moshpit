use clap::{Parser, ArgGroup, ValueEnum};
use clap_num::maybe_hex;

#[derive(ValueEnum, Parser, Clone, Debug)]
pub enum OptionsMode {
    BLACK,
    BRIGHT,
    WHITE,
}

#[derive(ValueEnum, Parser, Clone, Debug)]
pub enum OptionsDirection {
    BOTH,
    COLUMN,
    ROW,
}

#[derive(Parser, Debug)]
#[command(author = "moth",
          version = "1.0.0",
          about = r"
/////////////////////////////////////////////////////////////////////////////
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\█████\\\\\\\\\\\\\\\\\███\\\█████\\\\\\
/////////////////////////////////////░░███/////////////////░░░///░░███///////
\\\\\█████████████\\\\██████\\\█████\\░███████\\\████████\\████\\███████\\\\\
////░░███░░███░░███//███░░███/███░░///░███░░███/░░███░░███░░███/░░░███░//////
\\\\\░███\░███\░███\░███\░███░░█████\\░███\░███\\░███\░███\░███\\\░███\\\\\\\
/////░███/░███/░███/░███/░███/░░░░███/░███/░███//░███/░███/░███///░███/███///
\\\\\█████░███\█████░░██████\\██████\\████\█████\░███████\\█████\\░░█████\\\\
////░░░░░/░░░/░░░░░//░░░░░░//░░░░░░//░░░░/░░░░░//░███░░░//░░░░░////░░░░░/////
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\░███\\\\\\\\\\\\\\\\\\\\\\\\
/////////////////////////////////////////////////█████///////////////////////
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\░░░░░\\\\\\\\\\\\\\\\\\\\\\\\
/////////////////////////////////////////////////////////////////////////////",
          long_about = None, arg_required_else_help(true))]
#[command(group(
        ArgGroup::new("sortopts")
        .required(false)
        .multiple(true)
        .args(["mode", "direction"])
        .requires_all(["mode", "direction"])
        ))]
pub struct Args {
    // Input Options
    #[arg(short, long,
          help_heading = "Image Input/Output",
          long_help = "Input image filename",
          required = true)]
    pub input: String,

    #[arg(short, long,
          help_heading = "Image Input/Output",
          long_help = "Output image filename",
          required = true)]
    pub output: String,

    #[arg(short, long,
          help_heading = "ASDF Options",
          long_help = "Sorting mode")]
    pub mode: Option<OptionsMode>,

    #[arg(short, long,
          help_heading = "ASDF Options",
          long_help = "Sorting threshold.\n\t\t0x00 - 0xff for brightness,\n\t\t0x00 - 0xffffffff (rgba space) for white/black.\nDefault values used if none provided:\n\t\tBlack:\t0x0bdc00ff\n\t\tBright:\t0x3c\n\t\tWhite:\t0x39a2c0ff",
          value_parser=maybe_hex::<u32>)]
    pub threshold: Option<u32>,
    
    #[arg(short, long,
          help_heading = "ASDF Options",
          long_help = "Sorting direction")]
    pub direction: Option<OptionsDirection>,
}

/// parse_args accepts no arguments and returns parsed results of the Args struct
pub fn parse_args() -> Args {
    Args::parse()
}
