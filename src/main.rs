use rand::Rng;
use std::io;
// Define a struct for a person
struct Person {
    name: String,
    has_activity: bool,
}

impl Person {
    // new
    fn new(name: &str, has_activity: bool) -> Self {
        Person {
            name: name.to_string(),
            has_activity,
        }
    }
    // get name
    fn get_name(&self) -> &str {
        &self.name
    }

    // get status
    fn check_availability(&self) -> bool {
        self.has_activity
    }
}

fn main() {
    let mut rng = rand::rng();
    let yrq_activity = rng.random_bool(20.0 / 100.0);

    let pl = Person::new("pl", true);
    let yrq_dsb = Person::new("yrq", yrq_activity);

    if pl.check_availability() && yrq_dsb.check_availability() {
        println!("We can have an activity together!");
    } else {
        println!("Fuck {}", yrq_dsb.get_name(),);
    }
    let mut input = String::new();
    // for my windows friend
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}
