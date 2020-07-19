extern crate image;

use std::env;
use image::{ImageBuffer, DynamicImage, imageops::FilterType::Nearest};

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (x_offset, y_offset) = if args[1].eq("y") {
        (1, 0)
    } else {
        (0, 1)
    };
    let scale = args[2].clone().parse::<u32>().expect("NaN");
    let output = args[3].clone();

    let mut min_x:i64 = i64::MAX;
    let mut min_y:i64 = i64::MAX;
    let mut max_x:i64 = i64::MIN;
    let mut max_y:i64 = i64::MIN;

    let mut points: Vec<Point> = Vec::new();

    for i in (4..args.len()-1).step_by(2) {
        let x: i64 = args[i+x_offset].parse::<i64>().expect("NaN");
        let y: i64 = args[i+y_offset].parse::<i64>().expect("NaN");
        points.push(Point{x: x, y: y});

        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }

    if output.eq("ascii") {
        for y in min_y..=max_y {
            let mut line = String::new();
            for x in min_x..=max_x {
                let mut c = '.';
                for point in &points {
                    if point.x==x && point.y==y {
                        c = '#';
                    }
                }
                for _x in 0..scale {
                    line.push(c);
                }
            }
            for _y in 0..scale {
                println!("{}", line);
            }
        }
    } else {
        let imgx = (1 + max_x - min_x) as u32;
        let imgy = (1 + max_y - min_y) as u32;

        let mut imgbuf = ImageBuffer::new(imgx, imgy);

        for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = image::Rgb([0 as u8, 0, 0]);
        }

        for point in &points {
            let pixel = imgbuf.get_pixel_mut((point.x-min_x) as u32, (point.y-min_y) as u32);
            *pixel = image::Rgb([255 as u8, 255, 255]);
        }

        let img = DynamicImage::ImageRgb8(imgbuf);
        let img = img.resize(imgx*scale, imgy*scale, Nearest);
        img.save(output).unwrap();
        
        //imgbuf.save(output).unwrap();
    }
}