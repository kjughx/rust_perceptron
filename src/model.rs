use crate::{
    layer::Layer,
    train::TrainingSet,
    shape::Shape,
    BIAS, WIDTH, HEIGHT,
    EVALUATE_CASE_COUNT, TRAINING_CASE_COUNT,
    LEARNING_RATE,
};
pub struct Model {
    pub layer: Layer,
}
fn cost(x: f64) -> &'static str{
    if x < BIAS{
        "Rectangle"
    }else{
        "Circle"
    }
}
impl Model {
    pub fn new() -> Model{
        Model{
            layer: Layer::new(),
        }
    }
    pub fn correct(&mut self, training_set: &TrainingSet, prediction: f64){
        let mut image = Layer::new();
        image.fill(training_set.shape);
        match training_set.label{
            "Rectangle" => {
                for x in 0..WIDTH{
                    for y in 0..HEIGHT{
                        if let Some(pixel_value) = image.get_point(x, y){
                        self.layer
                            .set_point(
                                x, 
                                y, 
                                pixel_value + (0.0 - LEARNING_RATE)*prediction*pixel_value
                            );
                        }
                    }
                }
            },
            "Circle" => {
            for x in 0..WIDTH{
                for y in 0..HEIGHT{
                    if let Some(pixel_value) = image.get_point(x, y){
                        self.layer
                            .set_point(
                                x,
                                y,
                                 pixel_value + (1.0 - LEARNING_RATE)*pixel_value
                            );
                    }
                } 
            }
            }
            _ => {}
        }
    }
    pub fn predict<'a>(&self, inputs: &Layer) -> f64{
        let mut output = 0f64;
        assert_eq!(self.layer.width, inputs.width);
        assert_eq!(self.layer.height, inputs.height);

        for x in 0..WIDTH{
            for y in 0..HEIGHT{
                if let Some(value) = self.layer.get_point(x,y){
                    output += value;
                }
            }
        }
        output
    }
    pub fn train(&mut self, training_sets: Vec<TrainingSet>) -> f64{
        println!("Training model...");
        let mut accuracy = 0.0;
        let mut image = Layer::new();
        for training_set in training_sets{
            image.fill(training_set.shape);
            let prediction = self.predict(&image);
            if cost(prediction) == training_set.label{
                accuracy += 1.0;
            } else{
                self.correct(&training_set, prediction);
            } 
        }
        println!("Finished training!");
        accuracy / (TRAINING_CASE_COUNT as f64)
    }

    pub fn evaluate(&self) -> f64{
        let mut accuracy = 0.0;
        let mut layer = Layer::new();
        for _ in 0..EVALUATE_CASE_COUNT{
            if let Some(shape) = Shape::random_shape(None){
                match shape{
                    Shape::Rectangle{x:_,y:_,w:_,h:_} => {
                        layer.fill(shape);
                        let prediction = self.predict(&layer);
                        if cost(prediction) == "Rectangle" {
                            accuracy += 1.0;
                        }
                    },
                    Shape::Circle{xc:_,yc:_,r:_} => {
                        layer.fill(shape);
                        let prediction = self.predict(&layer);
                        if cost(prediction) == "Circle" {
                            accuracy += 1.0;
                        }
                    }
                }
            }
        }
        accuracy / (EVALUATE_CASE_COUNT as f64)
    }
}