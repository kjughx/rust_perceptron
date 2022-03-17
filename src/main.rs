use perceptron::{
    layer::{
        Color
    },
    model::Model,
    train::TrainingSet,
    TRAINING_CASE_COUNT,
};

fn main() -> Result<(), std::io::Error>{
    let training_sets = TrainingSet::generate_training_sets(TRAINING_CASE_COUNT);
    let mut model = Model::new();

    let train_accuracy = model.train(training_sets);
    println!("Model had a training accuracy of {}%", (train_accuracy*100.0) as i8);
    let eval_accuracy = model.evaluate();
    println!("Model had a evaluation accuracy of {}%", (eval_accuracy*100.0) as i8);

    model
        .layer
        .write_to_ppm("model.ppm".to_string(), Color{r:255/3, g:255/3, b:255/3})?;

    Ok(())
}
