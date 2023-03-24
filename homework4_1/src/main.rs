#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}
trait LightTime{
    fn time(&self) -> u8 ;
}

impl LightTime for TrafficLight {
    fn time(&self) -> u8{
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Green => 25,
            TrafficLight::Yellow => 5,
        }

    }
}

fn main() {
    let yellow = TrafficLight::Yellow;
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    println!("yellow time is {:?}",yellow.time());
    println!("red time is {:?}",red.time());
    println!("green time is {:?}",green.time());
}