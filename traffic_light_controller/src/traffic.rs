pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn display(&self) {
        match self {
            TrafficLight::Red => println!("The light will stay red for 30 seconds"),
            TrafficLight::Green => println!("The light will stay green for 20 seconds"),
            TrafficLight::Yellow => println!("The light will stay yellow for 5 seconds"),
        }
    }
}