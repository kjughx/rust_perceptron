use rand::{
    Rng,
    random,
};
use crate::{
    WIDTH,
    HEIGHT
};
#[derive(Clone, Copy)]
pub enum Shape{
    Rectangle {
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    },
    Circle {
        xc: u32,
        yc: u32,
        r:  u32,
    }
}

impl Shape {
    pub fn new_rectangle(x: u32, y: u32, w: u32, h: u32) -> Shape{
        Shape::Rectangle { x, y, w, h }
    }
    pub fn new_circle(xc: u32, yc: u32, r: u32) -> Shape{
        Shape::Circle{ xc, yc, r }
    }
    pub fn get_dimensions(&self) -> Vec<&u32>{
        match self{
            Shape::Rectangle{x, y, w, h} => {
                return Vec::from([x, y, w, h]);
            },
            Shape::Circle{xc, yc, r} => {
                return Vec::from([xc, yc, r]);
            }
        }
    }
    pub fn random_shape(shape: Option<&str>) -> Option<Shape>{
        match shape {
            Some("Rectangle") => {
                let rectangle = Shape::new_rectangle(
                    rand::thread_rng().gen_range(0..WIDTH),
                    rand::thread_rng().gen_range(0..HEIGHT),
                    rand::thread_rng().gen_range(0..WIDTH/2),
                    rand::thread_rng().gen_range(0..HEIGHT/2),
                );
                Some(rectangle)
            },
            Some("Circle") => {
                let circle = Shape::new_circle(
                    rand::thread_rng().gen_range(0..WIDTH),
                    rand::thread_rng().gen_range(0..HEIGHT),
                    rand::thread_rng().gen_range(0..std::cmp::min(WIDTH/2, HEIGHT/2)),
                );
                Some(circle)
            },
            Some(&_) => {None},
            None => {
                let shape = random::<u8>() % 2;
                match shape {
                    0 => {
                        let rectangle = Shape::new_rectangle(
                            rand::thread_rng().gen_range(0..WIDTH),
                            rand::thread_rng().gen_range(0..HEIGHT),
                            rand::thread_rng().gen_range(0..WIDTH/2),
                            rand::thread_rng().gen_range(0..HEIGHT/2),
                        );
                        Some(rectangle)
                    }
                    1 => {
                        let circle = Shape::new_circle(
                            rand::thread_rng().gen_range(0..WIDTH),
                            rand::thread_rng().gen_range(0..HEIGHT),
                            rand::thread_rng().gen_range(0..std::cmp::min(WIDTH/2, HEIGHT/2)),
                        );
                        Some(circle)
                    }
                    _ => {None}
                }
            }
        }
    }
}

#[inline]
pub fn clamp<T: std::cmp::PartialOrd>(val: T, low: T, high: T) -> T{
    if val < low {return low;}
    if val > high {return high;}
    val
}

#[inline]
pub fn in_bounds<T: std::cmp::PartialOrd>(val: T, low: T, high: T) -> bool {
    if val >= low && val <= high {return true;}
    false
}
