#![allow(unused_variables, dead_code, unused_imports)]
use perceptron::{
    shape::Shape,
    layer::{
        Layer,
        Color
    },
    train::TrainingSet,
    TRAINING_CASE_COUNT,
    WIDTH,
    HEIGHT,
};

use rand::random;

fn main() -> Result<(), std::io::Error>{
    let mut model = Layer::new(WIDTH, HEIGHT);
    let training_sets = TrainingSet::generate_training_sets(TRAINING_CASE_COUNT);

    model.train(training_sets);
    let accuracy = model.evaluate();
    println!("Model has an accuracy of {}%", (accuracy*100.0) as i8);

    model.write_to_ppm("model.ppm".to_string(), Color{r:255/3, g:255/3, b:255/3})?;

    Ok(())
}
