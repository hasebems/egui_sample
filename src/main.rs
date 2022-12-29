use eframe::{egui::*};

#[derive(Default)]
pub struct EguiSample {}

impl EguiSample {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "profont".to_owned(),
            FontData::from_static(include_bytes!("../assets/newyork.ttf")),
        );
        fonts.font_data.insert(
            "monofont".to_owned(),
            FontData::from_static(include_bytes!("../assets/courier.ttc")),
        );

        fonts
            .families
            .entry(FontFamily::Proportional)    //  search value of this key
            .or_default()                       //  if not found
            .insert(0, "profont".to_owned());
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "monofont".to_owned());

        cc.egui_ctx.set_fonts(fonts);
        Self::default()
    }
}

impl eframe::App for EguiSample {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}       
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {});
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some((400.0, 400.0).into()),
        resizable: false,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native("egui_sample", options, Box::new(|cc| Box::new(EguiSample::new(cc))));
}