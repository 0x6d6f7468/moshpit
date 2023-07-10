# moshpit

## Description
Simple CLI wrapper around ASDF pixel sorting crate.

ASDF pixel sorting is an image manipulation algorithm designed by Kim Asendorf. Check out their repository here: https://github.com/kimasendorf/ASDFPixelSort

The original tool is written in Processing, which is a very powerful platform, but I was looking for a command line utility that could accomplish the same task.

The code probably isn't the best/cleanest as I'm still learning Rust, but it seems to work pretty well.

## Usage
```
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
/////////////////////////////////////////////////////////////////////////////

Usage: moshpit [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version

Image Input/Output:
  -i, --input <INPUT>    Input image filename
  -o, --output <OUTPUT>  Output image filename

ASDF Options:
  -m, --mode <MODE>            Sorting mode [possible values: black, bright, white]
  -t, --threshold <THRESHOLD>  Sorting threshold.
                                        0x00 - 0xff for brightness,
                                        0x00 - 0xffffffff (rgba space) for white/black.
                               Default values used if none provided:
                                        Black:  0x0bdc00ff
                                        Bright: 0x3c
                                        White:  0x39a2c0ff
  -d, --direction <DIRECTION>  Sorting direction [possible values: both, column, row]
```

## Example(s)

