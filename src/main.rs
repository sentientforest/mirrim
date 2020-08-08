extern crate image;

use std::{
    io::{self, Write},
};

use image::{GenericImageView, imageops};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Given a path to an image, generates its mirror and combine the two into one output image
struct Cli {
    #[structopt(short = "i", long = "input")]
    /// input path to an image
    input_path: String,

    #[structopt(short = "m", long = "mirror")]
    /// direction to mirror. pass a value of:
    ///
    ///  l or left: mirror to the left. mirror image appears on the left in the output, original on right.
    ///
    ///  r or right: (default). mirror to the right. mirror image appears on the right in the output, original on the left.

    mirror: String,

    /// output path to write the generated image
    output_path: String
}

fn main() {
    let args = Cli::from_args();
    let mut stdout = io::stdout();
    let mirror_right = match args.mirror.as_str() {
        "l" | "left" => false,
        _ => true,
    };
    println!("got mirror_right value: {}", mirror_right);
    let img = image::open(&args.input_path).unwrap();
    let mir = imageops::flip_horizontal(&img);
    let (width, height) = img.dimensions();
    let dwidth = 2 * width as u32;
    let mut combined_img = image::ImageBuffer::new(dwidth, height);
    for (x, y, pixel) in combined_img.enumerate_pixels_mut() {
        // todo: pass in param to use original as either left, right, top or bottom
        // if top or bottom, vertical flip vs horizontal
        // left-side pixels
        if x < width {
            *pixel = if mirror_right {img.get_pixel(x, y)} else {*mir.get_pixel(x, y)};
        } else {
            // right side pixls
            let nx = x - width;
            *pixel = if mirror_right {*mir.get_pixel(nx, y)} else {img.get_pixel(nx, y)};
        }
    }

    writeln!(stdout, "{}, {}", "got path: ", &args.input_path).unwrap();
    writeln!(stdout, "combined output dimensions {:?}", combined_img.dimensions()).unwrap();
    combined_img.save(&args.output_path).unwrap();
}
