#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, CreationContext, NativeOptions};
use egui::{Button, CentralPanel, Context, UserAttentionType};


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 240.0]).with_resizable(false),
        // viewport: egui::ViewportBuilder::default().with_fullscreen(true),

        ..Default::default()
    };

    // Our application state:
    let mut temp_type = String::from("Types of temp:");
    

    eframe::run_simple_native("Temperature Converter", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Simple temperature converter!");
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{}",&temp_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut temp_type, "Fahrenheit".to_owned(), "Fahrenheit");
                    ui.selectable_value(&mut temp_type, "Celcius".to_owned(), "Celcius");
                    ui.selectable_value(&mut temp_type, "Kelvin".to_owned(), "Kelvin");
                }
                );
            });

            ui.label(&temp_type);
        });
    })
}
