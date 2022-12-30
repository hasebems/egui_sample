use eframe::{egui::*};
use eframe::egui;
use std::time::{Duration, Instant};

pub struct EguiSample {
    input_locate: usize,
    input_text: String,
    start_time: Instant,
}

impl EguiSample {
    const SPACE_LEFT: f32 = 30.0;
    const SPACE_RIGHT: f32 = 570.0;
    const LEFT_MERGIN: f32 = 5.0;
    const LETTER_WIDTH: f32 = 10.0;

    const SPACE_UPPER: f32 = 20.0;
    const SPACE_LOWER: f32 = 50.0;
    const UPPER_MERGIN: f32 = 2.0;
    const LOWER_MERGIN: f32 = 3.0;
    const CURSOR_MERGIN: f32 = 6.0;
    const CURSOR_THICKNESS: f32 = 4.0;

    const PROMPT_LETTERS: &str = "hasebe>";
    const PROMPT_LEN: usize = Self::PROMPT_LETTERS.len();
    const CURSOR_MAX_LOCATE: usize = 50;

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "monofont".to_owned(),
            FontData::from_static(include_bytes!("../assets/courier.ttc")),
        );
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "monofont".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        Self {
            input_locate: 0,
            input_text: String::new(),
            start_time: Instant::now(),
        }
    }
    fn command_key(&mut self, key: &Key) {
        if key == &Key::Enter {
            // send text somewhere
            self.input_text = "".to_string();
            self.input_locate = 0;
            println!("Key>>{:?}",key);
        }
        else if key == &Key::Backspace {
            if self.input_locate > 0 {
                self.input_locate -= 1;
                self.input_text.remove(self.input_locate);
            }
            println!("Key>>{:?}",key);
        }
        else if key == &Key::ArrowLeft {
            if self.input_locate > 0 {self.input_locate -= 1;}
            println!("Key>>{:?}",key);
        }
        else if key == &Key::ArrowRight {
            self.input_locate += 1;
            let maxlen = self.input_text.chars().count();
            if self.input_locate > maxlen { self.input_locate = maxlen;}
            println!("Key>>{:?}",key);
        }
    }
    fn input_letter(&mut self, letters: Vec<&String>) {
        if self.input_locate <= Self::CURSOR_MAX_LOCATE {
            println!("Letters:{:?}",letters);
            letters.iter().for_each(|ltr| {self.input_text.push_str(ltr);self.input_locate+=1;});    
        }
    }
    fn update_input_text(&mut self, ui: &mut egui::Ui) {
        // Paint input Letter Space
        ui.painter().rect_filled(
            Rect::from_min_max(pos2(Self::SPACE_LEFT,Self::SPACE_UPPER),
                               pos2(Self::SPACE_RIGHT,Self::SPACE_LOWER)),
            2.0,                              //  curve
            Color32::from_rgb(48, 48, 48)     //  color
        );
        // Paint cursor
        let cursor = self.input_locate + Self::PROMPT_LEN;
        let elapsed_time = self.start_time.elapsed().as_millis();
        if elapsed_time%500 > 200 {
            ui.painter().rect_filled(
                Rect { min: Pos2 {x:Self::SPACE_LEFT + Self::LEFT_MERGIN 
                                    + 5.0 + 9.5*(cursor as f32),
                                y:Self::SPACE_LOWER - Self::CURSOR_MERGIN},
                       max: Pos2 {x:Self::SPACE_LEFT + Self::LEFT_MERGIN 
                                    + 3.0 + 9.5*((cursor+1) as f32),
                                y:Self::SPACE_LOWER - Self::CURSOR_MERGIN + Self::CURSOR_THICKNESS},},
                0.0,                              //  curve
                Color32::from_rgb(160, 160, 160)  //  color
            );
        }
        // Draw Letters
        ui.put( // Prompt
            Rect { min: Pos2 {x:Self::SPACE_LEFT + Self::LEFT_MERGIN,
                              y:Self::SPACE_UPPER + Self::UPPER_MERGIN},
                   max: Pos2 {x:Self::SPACE_LEFT + Self::LEFT_MERGIN 
                                + Self::LETTER_WIDTH*(Self::PROMPT_LEN as f32),
                              y:Self::SPACE_LOWER - Self::LOWER_MERGIN},},
            Label::new(RichText::new(Self::PROMPT_LETTERS)
                .size(16.0).color(Color32::from_rgb(0,200,200)).family(FontFamily::Monospace))
        );
        let txtcnt = self.input_text.chars().count() + Self::PROMPT_LEN;
        ui.put( // User Input
            Rect { min: Pos2 {x:Self::SPACE_LEFT + Self::LEFT_MERGIN 
                                + Self::LETTER_WIDTH*(Self::PROMPT_LEN as f32)
                                + 3.25 - 0.25*(txtcnt as f32), // 謎の調整
                              y:Self::SPACE_UPPER + Self::UPPER_MERGIN},
                   max: Pos2 {x:Self::SPACE_LEFT + Self::LEFT_MERGIN 
                                + Self::LETTER_WIDTH*(txtcnt as f32)
                                + 3.25 - 0.25*(txtcnt as f32), // 謎の調整
                              y:Self::SPACE_LOWER - Self::LOWER_MERGIN},},
            Label::new(RichText::new(&self.input_text)
                .size(16.0).color(Color32::WHITE).family(FontFamily::Monospace))
        );
    }
}

impl eframe::App for EguiSample {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}       
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // repaint 100msec interval
        ctx.request_repaint_after(Duration::from_millis(100));

        //  Get Keyboard Event from Egui::Context
        let evts = ctx.input().events.clone();
        let mut letters: Vec<&String> = vec![];
        for ev in evts.iter() {
            match ev {
                Event::Text(ltr) => letters.push(ltr),
                Event::Key {key,pressed, modifiers:_} => {
                    if pressed == &true { self.command_key(key);}
                },
                _ => {},
            }
        }
        if letters.len() >= 1 {self.input_letter(letters);}

        // Configuration for CentralPanel
        let my_frame = egui::containers::Frame {
            inner_margin: egui::style::Margin { left: 0., right: 0., top: 0., bottom: 0. },
            outer_margin: egui::style::Margin { left: 0., right: 0., top: 0., bottom: 0. },
            rounding: egui::Rounding { nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 },
            shadow: eframe::epaint::Shadow { extrusion: 0.0, color: Color32::BLACK },
            fill: Color32::BLACK,
            stroke: egui::Stroke::new(0.0, Color32::BLACK),
        };

        CentralPanel::default().frame(my_frame).show(ctx, |ui| {
            //  input text
            self.update_input_text(ui);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some((600.0, 80.0).into()),
        resizable: false,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native("egui_sample", options, Box::new(|cc| Box::new(EguiSample::new(cc))));
}