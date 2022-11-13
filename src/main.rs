use eframe::{egui::*};

#[derive(Default)]
pub struct EguiSample {}

impl EguiSample {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {Self::default()}
}

impl eframe::App for EguiSample {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}       
    fn update(&mut self, _ctx: &Context, _frame: &mut eframe::Frame) {}
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("egui_sample", options, Box::new(|cc| Box::new(EguiSample::new(cc))));
}