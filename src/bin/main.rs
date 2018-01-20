extern crate raster;
extern crate image;

use image::{RgbImage, Rgb};
use raster::*;

fn main() {
    let mut img = RgbImage::new(256, 256);

    draw_line(&mut img, 10, 10, 246, 128, Rgb{data:[0,128,255]});
    draw_line(&mut img, 10, 10, 13, 128, Rgb{data:[0,128,255]});
    draw_hline(&mut img, 10, 30, 10, Rgb{data:[255,0,0]});
    draw_vline(&mut img, 40, 45, 100, Rgb{data:[0,255,0]});
    draw_circle(&mut img, 128, 30, 15, Rgb{data:[255,128,255]});
    draw_fill_circle(&mut img, 200, 100, 30, Rgb{data:[255,128,255]});

 	let ngon = generate_ngon(100,100,2,30);
 	draw_ngon(&mut img, &ngon, Rgb{data:[255,128,255]});

    img.save("output.png").unwrap();
}