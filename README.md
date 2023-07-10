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

### Example 1: Default Arguments, Saving as Different Image Type

#### Original Photo:
![image](https://github.com/0x6d6f7468/moshpit/assets/25512187/d8779d4c-f50b-4082-849f-e2d66c8fe617)

Image Credit: "Blue-headed hummingbird" by Charles J. Sharp is licensed under CC BY 3.0

Image Source: [https://commons.wikimedia.org/w/index.php?curid=12374013]()

#### Command:
```bash
./moshpit -i hummingbird_original.jpg -o hummingbird_sorted.png
```

#### Result:
![image](https://github.com/0x6d6f7468/moshpit/assets/25512187/ae65c6a7-4774-4d1f-a043-be5d091b091d)

### Example 2: Custom Direction, Mode, and Threshold

#### Original Photo:
![image](https://github.com/0x6d6f7468/moshpit/assets/25512187/e0ca4995-9600-4f9c-baa8-59c127ca9377)

Image Credit: "Madagascar sunset moth, aka Round-winged Emerald butterfly." by Swallowtail Garden Seeds is marked with Public Domain Mark 1.0

Image Source: [https://www.flickr.com/photos/97123293@N07/19626923458]()

#### Command:
```bash
./moshpit -i moth_original.jpg -o moth_original.jpg -d both -m black -t 0x0bdcffff
```

#### Result:
![image](https://github.com/0x6d6f7468/moshpit/assets/25512187/59d9416c-88e2-4c82-b49a-bd5c022bbe8d)
