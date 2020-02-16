use learner::*;

pub mod learner {
    tonic::include_proto!("learner");
}

fn main() {
    let feature = Feature {};
    let label = Label {};
    let item = Item {
        id: "id".to_string(),
        feature: Some(feature),
        label: Some(label)
    };

    println!("hello rust");
}
