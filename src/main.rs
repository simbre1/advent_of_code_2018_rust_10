extern crate image;

use image::{ImageBuffer, Rgb};
use std::fs;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32
}

type Velocity = Point;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Point {
    fn add(&self, other_x: i32, other_y: i32) -> Point {
        Point {
            x: self.x + other_x,
            y: self.y + other_y
        }
    }
}

struct PV {
    position: Point,
    velocity: Velocity
}

impl PV {
    fn translate(&self, x: i32, y: i32) -> PV {
        PV {
            position: self.position.add(x, y),
            velocity: self.velocity.clone()
        }
    }

    fn get_position_at(&self, seconds: i32) -> Point {
        Point {
            x: self.position.x + (self.velocity.x * seconds),
            y: self.position.y + (self.velocity.y * seconds)
        }
    }
}

impl fmt::Display for PV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos: {}, vel {}", self.position, self.velocity)
    }
}

fn main() {
    let contents = fs::read_to_string("D:\\dev\\advent_of_code_2018\\rust-10\\input.txt")
        .expect("peut");

    let raw_pvs: Vec<PV> = contents.lines()
        .map(|line| parse(line))
        .collect();

    let min_x = raw_pvs.iter().map(|pv| pv.position.x).min().unwrap();
    let min_y = raw_pvs.iter().map(|pv| pv.position.y).min().unwrap();

    let pvs: Vec<PV> = raw_pvs.iter()
        .map(|pv| pv.translate(-min_x, -min_y))
        .collect();

    let mut i: i32 = 0;
    let mut prev_size: u64 = std::u64::MAX;
    loop {
        let points = create_points_at_second(&pvs, i);
        let max_x = points.iter().map(|p| p.x).max().unwrap() as u64;
        let max_y = points.iter().map(|p| p.y).max().unwrap() as u64;
        let size = max_x * max_y;

        println!(
            "second {}, size: {}, dimensions: {}x{}",
            i,
            size,
            max_x,
            max_y);

        if size > prev_size {
            create_image_at_second(&pvs, i-1);
            break;
        } else {
            prev_size = size;
            i+=1;
        }
    }
}

fn create_points_at_second(pvs: &[PV], second: i32) -> Vec<Point> {
    let points: Vec<Point> = pvs.iter()
        .map(|pv| pv.get_position_at(second))
        .collect();

    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();

    //translate to origin
    let points: Vec<Point> = points.iter()
        .map(|p| p.add(-min_x, -min_y))
        .collect();

    points
}

fn create_image_at_second(pvs: &[PV], second: i32) {
    let points = create_points_at_second(pvs, second);
    let path = format!("D:\\dev\\advent_of_code_2018\\rust-10\\images\\output_{}.png", second);
    create_image(&path, &points);
}

fn create_image(path: &String, points: &[Point]) {
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let scale;
    if std::cmp::max(max_x, max_y) > 500 {
        scale = 500 as f32 / std::cmp::max(max_x, max_y) as f32;
    } else {
        scale = 1.0;
    }
    let w = (max_x as f32 * scale).ceil() as u32 + 2;
    let h = (max_y as f32 * scale).ceil() as u32 + 2;

    let mut image = ImageBuffer::<Rgb<u8>,Vec<u8>>::new(w, h);
    points.iter().for_each(
        |p| image.get_pixel_mut(
            (p.x as f32 * scale).floor() as u32 + 1,
            (p.y as f32 * scale).floor() as u32 + 1)
            .data = [255, 255, 255]);

    image.save(path).unwrap();
}

//position=<-39892,  -9859> velocity=< 4,  1>
fn parse(str: &str) -> PV {
    let a = str.find("<").unwrap();
    let b = a + str[a..].find(",").unwrap();
    let c = b + str[b..].find(">").unwrap();
    let d = c + str[c..].find("<").unwrap();
    let e = d + str[d..].find(",").unwrap();
    let f = e + str[e..].find(">").unwrap();

    PV {
        position: Point {
            x: str[a + 1..b].trim().parse::<i32>().unwrap(),
            y: str[b + 1..c].trim().parse::<i32>().unwrap(),
        },
        velocity: Velocity {
            x: str[d + 1..e].trim().parse::<i32>().unwrap(),
            y: str[e + 1..f].trim().parse::<i32>().unwrap(),
        }
    }
}
