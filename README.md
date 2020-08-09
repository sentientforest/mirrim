## mirrim

_Mirror Image Command Line Interface Tool_

mirrim: Mirr[or] Im[age]

Given an image, reverse it, and output the opposing mirror versions as a single image file.

Work in progress.

## Examples

mirrim 0.1.0
Given a path to an image, generates its mirror and combine the two into one output image

USAGE:
    mirrim <output-path> --input <input-path> --mirror <mirror>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


OPTIONS:
    -i, --input <input-path>    
            input path to an image

    -m, --mirror <mirror>       
            direction to mirror. pass a value of:

            l or left: mirror to the left. mirror image appears on the left in the output, original on right.

            r or right: (default). mirror to the right. mirror image appears on the right in the output, original on the
            left.

ARGS:
    <output-path>    
            output path to write the generated image


## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
