extern crate image;

use std::f32::consts::PI;
use image::{GenericImage};

pub struct Point {
	x:i32,
	y:i32
}

pub fn generate_ngon(x0:i32, y0:i32, n:u32, radius:i32) -> Vec<Point> {
	let mut v:Vec<Point> = Vec::new();
	for i in 0..n {
		let p = Point{
			x:x0 + ((radius as f32)*((2.0*PI*i as f32)/(n as f32)).cos()) as i32,
			y:y0 + ((radius as f32)*((2.0*PI*i as f32)/(n as f32)).sin()) as i32,
		};
		v.push(p);
	}
	v
}

pub fn draw_ngon<T: GenericImage>(img: &mut T, v:&Vec<Point>, pixel: T::Pixel) {
	for i in 0..v.len()-1 {
		draw_line(img, v[i].x, v[i].y, v[i+1].x, v[i+1].y, pixel);
	}
	draw_line(img, v[v.len()-1].x, v[v.len()-1].y, v[0].x, v[0].y, pixel);

}

pub fn draw_hline<T: GenericImage>(img: &mut T, x0: i32, x1: i32, y: i32, pixel: T::Pixel) {
  for x in x0..x1 {
    img.put_pixel(x as u32, y as u32, pixel);
  }
}

pub fn draw_vline<T: GenericImage>(img: &mut T, x: i32, y0: i32, y1: i32, pixel: T::Pixel) {
  for y in y0..y1 {
    img.put_pixel(x as u32, y as u32, pixel);
  }
}


pub fn draw_line<T: GenericImage>(img: &mut T, x0: i32, y0: i32, x1: i32, y1: i32, pixel: T::Pixel) {
  let mut x0 = x0;
  let mut y0 = y0;

  let dx = (x1-x0).abs();
  let dy = (y1-y0).abs();
  let slope_x = if x0 < x1 { 1 } else { -1 };
  let slope_y = if y0 < y1 { 1 } else { -1 };
  let mut err = (if dx>dy { dx } else { -dy })/2;
  let mut err2;

  loop {
    img.put_pixel(x0 as u32, y0 as u32, pixel);
    if x0==x1 && y0==y1 { break; }
    // adjust error and new position
    err2 = 2*err;
    if err2 >-dx {
        err -= dy;
        x0 += slope_x;
    }
    if err2 < dy {
        err += dx;
        y0 += slope_y;
    }
  }
}

// https://rosettacode.org/wiki/Bitmap/Midpoint_circle_algorithm#C
pub fn draw_circle<T: GenericImage>(img: &mut T, x0: i32, y0: i32, radius: i32, pixel: T::Pixel) {
    let mut f = 1 - radius;
    let mut ddf_x = 0;
    let mut ddf_y = -2 * radius;
    let mut x = 0;
    let mut y = radius;
 
    img.put_pixel(x0 as u32, (y0 + radius) as u32, pixel);
    img.put_pixel(x0 as u32, (y0 - radius) as u32, pixel);
    img.put_pixel((x0 + radius) as u32, y0 as u32, pixel);
    img.put_pixel((x0 - radius) as u32, y0 as u32, pixel);

    while x < y {
        if f >= 0 {
            y -= 1;
            ddf_y += 2;
            f += ddf_y;
        }
        x += 1;
        ddf_x += 2;
        f += ddf_x + 1;

        img.put_pixel((x0 + x) as u32, (y0 + y) as u32, pixel);
        img.put_pixel((x0 - x) as u32, (y0 + y) as u32, pixel);
        img.put_pixel((x0 + x) as u32, (y0 - y) as u32, pixel);
        img.put_pixel((x0 - x) as u32, (y0 - y) as u32, pixel);
        img.put_pixel((x0 + y) as u32, (y0 + x) as u32, pixel);
        img.put_pixel((x0 - y) as u32, (y0 + x) as u32, pixel);
        img.put_pixel((x0 + y) as u32, (y0 - x) as u32, pixel);
        img.put_pixel((x0 - y) as u32, (y0 - x) as u32, pixel);
    }
}


pub fn draw_fill_circle<T: GenericImage>(img: &mut T, x0: i32, y0: i32, radius: i32, pixel: T::Pixel) {
    let mut f = 1 - radius;
    let mut ddf_x = 0;
    let mut ddf_y = -2 * radius;
    let mut x = 0;
    let mut y = radius;
 
    draw_hline(img, (x0 - radius) as i32, (x0 + radius) as i32, y0 as i32, pixel);

    while x < y {
        if f >= 0 {
            y -= 1;
            ddf_y += 2;
            f += ddf_y;
        }
        x += 1;
        ddf_x += 2;
        f += ddf_x + 1;

        draw_hline(img, (x0 - x) as i32, (x0 + x) as i32, (y0 + y) as i32, pixel);
        draw_hline(img, (x0 - x) as i32, (x0 + x) as i32, (y0 - y) as i32, pixel);
        draw_hline(img, (x0 - y) as i32, (x0 + y) as i32, (y0 + x) as i32, pixel);
        draw_hline(img, (x0 - y) as i32, (x0 + y) as i32, (y0 - x) as i32, pixel);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
