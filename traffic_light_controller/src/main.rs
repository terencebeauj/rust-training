enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn display(&self) {
        match self {
            TrafficLight::Red => println!("The light will stay red for 30 seconds"),
            TrafficLight::Green => println!("The light will stay green for 20 seconds"),
            TrafficLight::Yellow => println!("The light will stay yellow for 5 seconds"),
        }
    }
}

fn main() {
    let red_light: TrafficLight = TrafficLight::Red;
    let yellow_light: TrafficLight = TrafficLight::Yellow;
    let green_light: TrafficLight = TrafficLight::Green;

    red_light.display();
    yellow_light.display();
    green_light.display();
}

// TODO: Implement a real-time counter
