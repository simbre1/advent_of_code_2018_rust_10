extern crate image;

use image::{ImageBuffer, Rgb};
use std::cmp;
use std::fs;

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32
}

struct Velocity {
    x: i32,
    y: i32
}

fn main() {
    let contents = fs::read_to_string("D:\\dev\\advent_of_code_2018\\rust-10\\input.txt")
        .expect("peut");

    let pvs: Vec<(Position, Velocity)> = contents.lines()
        .map(|line| parse(line))
        .collect();

    let w: u32 = 10;
    let h: u32 = 10;

    let mut image = ImageBuffer::<Rgb<u8>,Vec<u8>>::new(w, h);

    // set a central pixel to white
    image.get_pixel_mut(5, 5).data = [255, 255, 255];

    // write it out to a file
    image.save("D:\\dev\\advent_of_code_2018\\rust-10\\images\\output.png").unwrap();
}

//position=<-39892,  -9859> velocity=< 4,  1>
fn parse(str: &str) -> (Position, Velocity) {
    let a = str.find("<").unwrap();
    let b = a + str[a..].find(",").unwrap();
    let c = b + str[b..].find(">").unwrap();
    let d = c + str[c..].find("<").unwrap();
    let e = d + str[d..].find(",").unwrap();
    let f = e + str[e..].find(">").unwrap();

    (
        Position {
            x: str[a+1..b].trim().parse::<i32>().unwrap(),
            y: str[b+1..c].trim().parse::<i32>().unwrap(),
        },
        Velocity {
            x: str[d+1..e].trim().parse::<i32>().unwrap(),
            y: str[e+1..f].trim().parse::<i32>().unwrap(),
        }
    )
}



