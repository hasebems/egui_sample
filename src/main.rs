use eframe::egui::*;
use std::time::{Duration, Instant};

pub struct NoteObj {
    para1: f32,
    para2: f32,
    para3: f32,
    time: i32,
}

impl NoteObj {
    fn disp(&self, crnt_time: i32, ui: &mut Ui) {
        let cnt = crnt_time - self.time;
        for i in 0..256 {
            let position = 127-(cnt*4)%128;
            let phase = std::f32::consts::PI*((i+position) as f32)/32.0;  // 波紋の密度
            let gray = 255.0 - (phase.sin()*255.0).abs();               // 濃淡の関数(sinの絶対値)
            let gray_scl = (gray*(255.0-(i as f32))/511.0) as u8;        // 白/Alpha値への変換
            ui.painter().circle_stroke(
                Pos2 {x:self.para1, y:self.para2},  // location
                i as f32,                                   // radius
                Stroke {width:1.0, color:Color32::from_white_alpha(gray_scl)}
            );
        }
    }
}

pub struct EguiSample {
    cnt: i32,
    instant: Instant,
    pos: Pos2,
    nobj: Vec<NoteObj>,
}

impl EguiSample {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            cnt: 0,
            instant: Instant::now(),
            pos: Pos2 {x:400.0, y:400.0},
            nobj: vec![NoteObj {para1:200.0,para2:200.0,para3:0.0,time:0}],
        }
    }
}

impl eframe::App for EguiSample {
    //fn save(&mut self, _storage: &mut dyn eframe::Storage) {}       
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(25));
        if self.instant.elapsed() >= Duration::from_millis(50) {
            self.cnt += 1;
            self.instant = Instant::now();
        }

        CentralPanel::default().show(ctx, |ui| {
            for obj in self.nobj.iter() {
                obj.disp(self.cnt, ui);
            }

            if self.cnt%10 == 0 {
                //frame.set_window_size(Vec2{x:200.0,y:400.0});
                self.pos.x = frame.info().window_info.size.x;
                self.pos.y = frame.info().window_info.size.y;
                println!("x:{},y:{}", self.pos.x, self.pos.y);
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some((400.0, 400.0).into()),
        //resizable: false,
        ..eframe::NativeOptions::default()
    };
    let _ = eframe::run_native(
        "egui_sample",
        native_options,
        Box::new(|cc| Box::new(EguiSample::new(cc)))
    );
}