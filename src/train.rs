use crate::{
    WIDTH,
    HEIGHT,
    shape::Shape,
    layer::Layer,
};
use std::path::Path;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::error;


pub struct TrainingSet<'a>{
    pub label: &'a str,
    pub shape: Shape,
}
impl TrainingSet<'_>{
    pub fn new(label: &str, shape: Shape) -> TrainingSet{
        TrainingSet{
            label,
            shape,
       }
    }
    pub fn generate_training_sets(count: u32) -> Vec<TrainingSet<'static>>{
        let mut training_sets = Vec::new();
        for _ in 0..count{
            if let Some(shape) = Shape::random_shape(None) {
                match shape {
                    Shape::Rectangle{x:_,y:_,w:_,h:_} => {
                        let training_set = TrainingSet::new("Rectangle", shape);
                        training_sets.push(training_set);
                    }
                    Shape::Circle { xc:_, yc:_, r:_ } =>{
                        let training_set = TrainingSet::new("Circle", shape);
                        training_sets.push(training_set);
                    }
                }
            }
        }
        training_sets
    }
    pub fn read_training_sets(directory: String) -> Result<Vec<TrainingSet<'static>>, Box<dyn error::Error>>{
        let mut training_sets = Vec::new();
        let paths = fs::read_dir(directory).unwrap();

        for path in paths{
            if let Ok(path) = path {
                if let Ok(lines) = read_lines(path.file_name()){
                    for line in lines{
                        if let Ok(line) = line {
                            let content: Vec<&str> = line.split(" ").collect();
                            match content[0]{
                                "Rectangle" => {
                                    assert_eq!(5, content.len());
                                    let training_set = TrainingSet::new(
                                    "Rectangle",
                                        Shape::new_rectangle(
                                            content[1].parse::<u32>()?,
                                            content[2].parse::<u32>()?,
                                            content[3].parse::<u32>()?,
                                            content[4].parse::<u32>()?,
                                        ),
                                    );
                                    training_sets.push(training_set);
                                },
                                "Circle" => {
                                    assert_eq!(4, content.len());
                                    let training_set = TrainingSet::new(
                                    "Circle",
                                        Shape::new_circle(
                                            content[1].parse::<u32>()?,
                                            content[2].parse::<u32>()?,
                                            content[3].parse::<u32>()?,
                                        ),
                                    );
                                    training_sets.push(training_set);

                                },
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(training_sets)
    }

    pub fn write_training_sets(filename: String, training_sets: &Vec<TrainingSet>) -> Result<(), io::Error>{
        for (index, training_set) in training_sets.iter().enumerate() {
            let name = format!("{}{}", filename, index);
            let path = Path::new(&name);
            let mut file = File::create(&path)?;
            match training_set.label {
                "Rectangle" => {
                    let dimensions = training_set.shape.get_dimensions();
                    let buffer = String::from(
                        format!(
                            "Rectangle {} {} {} {}",
                            dimensions[0],
                            dimensions[1],
                            dimensions[2],
                            dimensions[3],
                        )
                    );
                    file.write(&buffer.as_bytes())?;
                },
                "Circle" => {
                    let dimensions = training_set.shape.get_dimensions();
                    let buffer = String::from(
                        format!(
                            "Circle {} {} {}",
                            dimensions[0],
                            dimensions[1],
                            dimensions[2],
                        )
                    );
                    file.write(&buffer.as_bytes())?;
                },
                _ => {
                    return Err(io::Error::new(io::ErrorKind::Other, "Failed to write to file"));
                }
            }
        }
        Ok(())
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}