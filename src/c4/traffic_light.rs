trait LightTime {
    fn light_seconds(&self) -> i64;
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl LightTime for TrafficLight {
    fn light_seconds(&self) -> i64 {
        match self {
            TrafficLight::Red => { 10 }
            TrafficLight::Yellow => { 20 }
            TrafficLight::Green => { 60 }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LightTime, TrafficLight};

    #[test]
    fn test_light_seconds() {
        let cases = [
            (TrafficLight::Red, 10),
            (TrafficLight::Yellow, 20),
            (TrafficLight::Green, 60),
        ];
        for (input, res) in cases {
            assert_eq!(input.light_seconds(), res);
        }
    }
}