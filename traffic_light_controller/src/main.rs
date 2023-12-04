mod traffic;

use crate::traffic::TrafficLight;

fn main() {
    let red_light: TrafficLight = TrafficLight::Red;
    let yellow_light: TrafficLight = TrafficLight::Yellow;
    let green_light: TrafficLight = TrafficLight::Green;

    red_light.display();
    yellow_light.display();
    green_light.display();
}

// TODO: Implement a real-time counter
