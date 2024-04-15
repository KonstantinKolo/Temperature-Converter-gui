#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod vars;
use vars::Temperaturetype;

use eframe::egui;


fn main() -> Result<(), eframe::Error> {
    //set the window size
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 240.0]).with_resizable(false),
        ..Default::default()
    };

    //application states:
    let mut temp_type = String::from("Types of temp:");
    let mut input_value: f32 = 0.0;
    let mut calculate: bool = false;
    

    eframe::run_simple_native("Temperature Converter", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Simple temperature converter!");
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Select one!")
                .width(110.0)
                .selected_text(format!("{}",&temp_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut temp_type, "Fahrenheit".to_owned(), "Fahrenheit");
                    ui.selectable_value(&mut temp_type, "Celcius".to_owned(), "Celcius");
                    ui.selectable_value(&mut temp_type, "Kelvin".to_owned(), "Kelvin");
                }); 
            });

            ui.add(egui::Slider::new(&mut input_value, -200.0..=400.0).text("degrees"));

            ui.horizontal(|ui| {
                if ui.button("Convert").clicked() {
                    calculate = true;
                }
            });
            
            if calculate == true{
                    let temperature = match temp_type.trim() {
                        "Fahrenheit" => Temperaturetype::fahrenheit_default(input_value),
                        "Celcius" => Temperaturetype::celcius_default(input_value),
                        "Kelvin" => Temperaturetype::kelvin_default(input_value),
                        _ => Temperaturetype::celcius_default(input_value),
                    };

                    let temp_values = temperature.convert_to_another_type();

                    let celcius = String::from("Celcius: ") + temp_values.get("Celcius").copied().unwrap_or(0.0).to_string().as_str() + "°";
                    let fahrenheit = String::from("Fahrenheit: ") + temp_values.get("Fahrenheit").copied().unwrap_or(0.0).to_string().as_str() + "°";
                    let kelvin = String::from("Kelvin: ") + temp_values.get("Kelvin").copied().unwrap_or(0.0).to_string().as_str() + "°";
                    
                    ui.colored_label(egui::Color32::RED, celcius);
                    ui.colored_label(egui::Color32::RED, fahrenheit);
                    ui.colored_label(egui::Color32::RED, kelvin);
            }
        });
    })
}