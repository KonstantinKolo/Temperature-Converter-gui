use std::collections::HashMap;

#[derive(Debug)]
pub enum Temperaturetype {
    Fahrenheit { name: String, value: f32},
    Celcius { name: String, value: f32},
    Kelvin {name: String, value: f32},
}

impl Temperaturetype {
    // Create a method to construct Fahrenheit, Celcius and Kelvin
    pub fn fahrenheit_default(value: f32) -> Self {
        Temperaturetype::Fahrenheit {
            name: String::from("Fahrenheit"),
            value,
        }
    }
    pub fn celcius_default(value: f32) -> Self {
        Temperaturetype::Celcius{
            name: String::from("Celcius"),
            value,
        }
    }
    pub fn kelvin_default(value: f32) -> Self {
        Temperaturetype::Kelvin {
            name: String::from("Kelvin"),
            value
        }
    }
}

impl Temperaturetype {
    pub fn convert_to_another_type(&self) -> HashMap<String, f32> {
        match self {
            Temperaturetype::Fahrenheit { name: _, value }=> {
                let celc: f32 = (value - 32.0) * 5.0 / 9.0;
                let kelv: f32 = celc + 273.15;
                
                let mut output = HashMap::new();
                output.insert(String::from("Celcius"), celc);
                output.insert(String::from("Fahrenheit"), *value);
                output.insert(String::from("Kelvin"), kelv);

                output
            }
            Temperaturetype::Celcius { name: _, value } => {
                let fahr: f32 = (9.0 / 5.0 * value) + 32.0;
                let kelv: f32 = value + 273.15;
                
                let mut output = HashMap::new();
                output.insert(String::from("Celcius"), *value);
                output.insert(String::from("Fahrenheit"), fahr);
                output.insert(String::from("Kelvin"), kelv);

                output
            }
            Temperaturetype::Kelvin { name: _, value} => {
                let celc: f32 = value - 273.15;
                let fahr: f32 = (value - 273.15) * 1.8 + 32.0;
                
                let mut output = HashMap::new();
                output.insert(String::from("Celcius"), celc);
                output.insert(String::from("Fahrenheit"), fahr);
                output.insert(String::from("Kelvin"), *value);

                output
            }
        }
    }
}
