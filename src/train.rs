use crate::{
    WIDTH,
    HEIGHT,
    shape::Shape,
    layer::Layer,
};

pub struct TrainingSet<'a>{
    pub label: &'a str,
    pub image: Layer,
}
impl TrainingSet<'_>{
    pub fn new(label: &str, shape: Shape) -> TrainingSet{
       let mut image = Layer::new(WIDTH, HEIGHT);
       image.fill(shape);
       TrainingSet{
           label,
           image,
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
}
