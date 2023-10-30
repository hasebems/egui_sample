use eframe::egui::*;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng, rngs};

pub struct NoteObj {
    para1: f32,
    para2: f32,
    para3: f32,
    time: i32,
}

impl NoteObj {
    const DISAPPEAR_RATE: f32 = 300.0;
    const RIPPLE_SIZE: i32 = 32;
    const BRIGHTNESS: f32 = 255.0;  // Max 255
    const RIPPLE_SIZE_F: f32 = (NoteObj::RIPPLE_SIZE-1) as f32;
    fn disp(&self, crnt_time: i32, ui: &mut Ui) -> bool {
        let cnt = (crnt_time - self.time)*4;
        if cnt as f32 > NoteObj::DISAPPEAR_RATE {return false;}
        for i in 0..NoteObj::RIPPLE_SIZE {
            let phase = std::f32::consts::PI*(i as f32)/16.0;  // 波の密度
            let gray = NoteObj::BRIGHTNESS*(1.0-phase.sin().abs());          // 波パターンの関数(sinの絶対値)
            let gray_scl = (gray*
                (self.para3/100.0)*
                ((NoteObj::RIPPLE_SIZE_F-(i as f32))/NoteObj::RIPPLE_SIZE_F)*     // 厚さと濃淡
                ((NoteObj::DISAPPEAR_RATE-(cnt as f32))/NoteObj::DISAPPEAR_RATE)  // 消えゆく速さ
            ) as u8;  // 白/Alpha値への変換
            if i < cnt {
                ui.painter().circle_stroke(
                    Pos2 {x:self.para1, y:self.para2},  // location
                    (cnt-i) as f32,                                   // radius
                    Stroke {width:1.0, color:Color32::from_white_alpha(gray_scl)}
                );
            }
        }
        true
    }
}

pub struct EguiSample {
    cnt: i32,
    instant: Instant,
    size: Pos2,
    rndm: rngs::ThreadRng,
    nobj: Vec<NoteObj>,
}

impl EguiSample {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            cnt: 0,
            instant: Instant::now(),
            size: Pos2 {x:400.0, y:400.0},
            rndm: thread_rng(),
            nobj: vec![NoteObj {para1:200.0, para2:200.0, para3:127.0, time:0}],
        }
    }
}

impl eframe::App for EguiSample {  
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(25));
        if self.instant.elapsed() >= Duration::from_millis(50) {
            self.cnt += 1;
            self.instant = Instant::now();
            if self.cnt%20 == 0 {   // 1sec
                let orgx = self.size.x;
                let orgy = self.size.y;
                self.size.x = frame.info().window_info.size.x;
                self.size.y = frame.info().window_info.size.y;
                if orgx != self.size.x || orgy != self.size.y {
                    println!("x:{},y:{}", self.size.x, self.size.y);
                }
                //  create new object
                let rndx: f32 = self.rndm.gen();
                let rndy: f32 = self.rndm.gen();
                let mut rnd_strength: f32 = self.rndm.gen();
                rnd_strength = rnd_strength*99.0 + 1.0;
                self.nobj.push(
                    NoteObj {para1:self.size.x*rndx, para2:self.size.y*rndy, para3:rnd_strength, time:self.cnt}
                );
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            let nlen = self.nobj.len();
            let mut rls = vec![true; nlen];
            for (i, obj) in self.nobj.iter_mut().enumerate() {
                if obj.disp(self.cnt, ui) == false {
                    rls[i] = false;
                }
            }
            for i in 0..nlen {  // 一度に一つ消去
                if !rls[i] {self.nobj.remove(i); break;}
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some((800.0, 800.0).into()),
        //resizable: false,
        ..eframe::NativeOptions::default()
    };
    let _ = eframe::run_native(
        "egui_sample",
        native_options,
        Box::new(|cc| Box::new(EguiSample::new(cc)))
    );
}