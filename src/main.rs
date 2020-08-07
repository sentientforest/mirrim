extern crate image;

use std::{
    io::{self, Write},
};

use image::{GenericImageView, imageops};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    path: String
}

fn main() {
    let args = Cli::from_args();
    let mut stdout = io::stdout();
    let img = image::open(&args.path).unwrap();
    let mir = imageops::flip_horizontal(&img);
    let (width, height) = img.dimensions();
    let dwidth = 2 * width as u32;
    let mut combined_img = image::ImageBuffer::new(dwidth, height);

    for (x, y, pixel) in combined_img.enumerate_pixels_mut() {
        // todo: pass in param to use original as either left, right, top or bottom
        // if top or bottom, vertical flip vs horizontal
        if x < width {
            *pixel = *mir.get_pixel(x, y);
        } else {
            let nx = x - width;
            *pixel = img.get_pixel(nx, y);
        }
    }

    writeln!(stdout, "{}, {}", "got path: ", &args.path).unwrap();
    writeln!(stdout, "combined output dimensions {:?}", combined_img.dimensions()).unwrap();
    // todo: pass in parameter to specify output path / name 
    combined_img.save("fixtures/test.png").unwrap();
}
