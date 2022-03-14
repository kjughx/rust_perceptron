#![allow(dead_code)]
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::{
    shape::{
        Shape,
        clamp,
        in_bounds
    },
    train::TrainingSet,
    COLOR_DEPTH,
    BIAS,
    WIDTH,
    HEIGHT,
    EVALUATE_CASE_COUNT,
};
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Layer {
    width: u32,
    height: u32,
    data: Vec<f64>,
}
type Model = Layer;

impl Layer{
    pub fn new(width: u32, height: u32) -> Layer{
        let size = height * width;
        let buffer = vec![0f64; size as usize];
        Layer{
            height,
            width,
            data: buffer
        }
    }
    pub fn buffer_size(&self) -> u32 {
        self.height * self.width
    }
    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (y * self.width) + x;
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }
    fn set_point(&mut self, x: u32, y: u32, value: f64) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset] = value;
                true
            },
            None => false
        }
    }
    pub fn fill(&mut self, shape: Shape){
        match shape {
            Shape::Rectangle{x,y,w,h} => {
                let x0 = x;
                let y0 = y;
                let x1 = x + w;
                let y1 = y + h;
                for y in 0..self.height{
                    for x in 0..self.width{
                        if in_bounds(x, x0, x1) && in_bounds(y, y0, y1){
                            self.set_point(x, y, 1f64);
                        } else{
                            self.set_point(x, y, 0f64);
                        }
                    }
                }
            },
            Shape::Circle{xc, yc, r} => {
                for y in 0..HEIGHT{
                    for x in 0..WIDTH{
                        let x0 = clamp((xc as i32) - (r as i32), 0, (WIDTH-1) as i32) as u32;
                        let y0 = clamp((yc as i32) - (r as i32), 0, (HEIGHT-1) as i32) as u32;
                        let x1 = clamp((xc as i32) + (r as i32), 0, (WIDTH-1) as i32) as u32;
                        let y1 = clamp((yc as i32) + (r as i32), 0, (HEIGHT-1) as i32) as u32;
                        let dx = ((x as i32) - (xc as i32)).abs() as u32;
                        let dy = ((y as i32) - (yc as i32)).abs() as u32;
                        if in_bounds(x, x0, x1) && in_bounds(y, y0, y1)
                            && dx*dx + dy*dy <= r*r{
                            self.set_point(x, y, 1f64);
                        }
                        else{
                            self.set_point(x, y, 0f64);
                        }
                    }
                }

            },
        }
    }
    pub fn correct(&mut self, training_set: &TrainingSet){
        assert_eq!(self.buffer_size(), training_set.image.buffer_size());
        match training_set.label {
            "Rectangle" => {
                for i in 0..self.buffer_size() as usize{
                    self.data[i] -= training_set.image.data[i];
                }
            },
            "Circle" => {
                for i in 0..self.buffer_size() as usize{
                    self.data[i] += training_set.image.data[i];
                }
            },
            _ => {},

        }
    }
    pub fn predict<'a>(&self, inputs: &Layer) -> &'a str{
        let mut output = 0f64;
        assert_eq!(self.width, inputs.width);
        assert_eq!(self.height, inputs.height);

        for i in 0..self.buffer_size() as usize{
            output += inputs.data[i] * self.data[i];
        }

        if output < BIAS{
            return "Rectangle"
        }
        else {
            return "Circle"
        }
    }
    pub fn train(&mut self, training_sets: Vec<TrainingSet>){
        println!("Training model...");
        for training_set in training_sets{
            let prediction = self.predict(&training_set.image);
            if prediction != training_set.label{
                self.correct(&training_set);
            }
        }
        println!("Finished training!");
    }
    pub fn evaluate(&self) -> f64{
        let mut accuracy = 0.0;
        let mut layer = Layer::new(WIDTH, HEIGHT);
        for _ in 0..EVALUATE_CASE_COUNT{
            if let Some(shape) = Shape::random_shape(None){
                match shape{
                    Shape::Rectangle{x:_,y:_,w:_,h:_} => {
                        layer.fill(shape);
                        let prediction = self.predict(&layer);
                        if prediction == "Rectangle" {
                            accuracy += 1.0;
                        }
                    },
                    Shape::Circle{xc:_,yc:_,r:_} => {
                        layer.fill(shape);
                        let prediction = self.predict(&layer);
                        if prediction == "Circle" {
                            accuracy += 1.0;
                        }
                    }
                }
            }
        }
        accuracy / (EVALUATE_CASE_COUNT as f64)
    }
    pub fn write_to_ppm(&self, filename: String, color: Color) -> Result<(), std::io::Error> {
        let path = Path::new(&filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} {}\n", self.width, self.height, COLOR_DEPTH);
        let mut data = Vec::new();
        for i in 0..self.buffer_size() as usize{
            data.push(((color.r as f64) * self.data[i]) as u8);
            data.push(((color.g as f64) * self.data[i]) as u8);
            data.push(((color.b as f64) * self.data[i]) as u8);
        }
        file.write(header.as_bytes())?;
        file.write(&data)?;
        println!("Drew the model as PPM in {}", filename);
        Ok(())
    }
}
