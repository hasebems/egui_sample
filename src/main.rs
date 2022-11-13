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
        CentralPanel::default().show(ctx, |ui| {
            ui.painter().vline(
                200.0,                                        // x
                std::ops::RangeInclusive::new(100.0, 300.0),  // y
                Stroke {width:3.0, color:Color32::RED},       // width, color
            );

            ui.painter().rect_filled(
                Rect { min: Pos2 {x:50.0, y:50.0}, 
                       max: Pos2 {x:150.0,y:150.0},},   // location
                8.0,                                    // curve
                Color32::from_rgb(199,21,133),          // color
            );

            ui.painter().circle_filled(
                Pos2 {x:300.0, y:300.0},  // location
                50.0,                     // radius
                Color32::GREEN,           // color
            );

            ui.painter().text(
                Pos2 {x:100.0, y:300.0},
                Align2::CENTER_CENTER,
                "Hello,",
                FontId::new(36.0, FontFamily::Proportional),
                Color32::WHITE
            );
            ui.painter().text(
                Pos2 {x:300.0, y:100.0},
                Align2::CENTER_CENTER,
                "World!",
                FontId::new(24.0, FontFamily::Monospace),
                Color32::BLUE
            );
        });
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