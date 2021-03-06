pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules"); 
            }
        }
    }
}

use a::series::of;
use a::series::of::nested_modules;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

//use TrafficLight::{Red, Yellow};
use TrafficLight::*;


fn main() {
    a::series::of::nested_modules();

    of::nested_modules();

    nested_modules();

    let _red = Red;
    let _yellow = Yellow;
    let _green = TrafficLight::Green;
    let _green2= Green;
}
