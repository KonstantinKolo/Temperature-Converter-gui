#[derive(Debug)]
pub enum Temperaturetype {
    Fahrenheit { name: String, value: i32},
    Celcius { name: String, value: i32},
    Kelvin {name: String, value: i32},
}

impl Temperaturetype {
    // Create a method to construct Fahrenheit, Celcius and Kelvin
    pub fn fahrenheit_default(value: i32) -> Self {
        Temperaturetype::Fahrenheit {
            name: String::from("Fahrenheit"),
            value,
        }
    }
    pub fn celcius_default(value: i32) -> Self {
        Temperaturetype::Celcius{
            name: String::from("Celcius"),
            value,
        }
    }
    pub fn kelvin_default(value: i32) -> Self {
        Temperaturetype::Kelvin {
            name: String::from("Kelvin"),
            value
        }
    }
}

impl Temperaturetype {
    pub fn convert_to_another_type(&self) -> (f32, f32) {
        match self {
            Temperaturetype::Fahrenheit { name: _, value }=> {
                let celc: f32 = ((value - 32) * 5 / 9) as f32;
                let kelv: f32 = celc + 273.15;
                let tup: (f32,f32) = (celc, kelv);
                tup
            }
            Temperaturetype::Celcius { name: _, value } => {
                let fahr: f32 = ((9 / 5 * value) + 32) as f32;
                let kelv: f32 = *value as f32 + 273.15;
                let tup: (f32,f32) = (fahr, kelv);
                tup
            }
            Temperaturetype::Kelvin { name: _, value} => {
                let celc: f32 = *value as f32 - 273.15;
                let fahr: f32 = (*value as f32 - 273.15) * 1.8 + 32.0;
                let tup: (f32,f32) = (celc, fahr);
                tup
            }
        }
    }
}
