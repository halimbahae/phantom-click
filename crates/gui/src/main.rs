use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use eframe::egui;
use phantom_click::{clicker, cursor};

struct PhantomGui {
    active: Arc<AtomicBool>,
    cps: Arc<AtomicU64>,
    cursor_x: f64,
    cursor_y: f64,
    last_click: String,
}

impl PhantomGui {
    fn new() -> Self {
        let active = Arc::new(AtomicBool::new(false));
        let cps = Arc::new(AtomicU64::new(10));
        clicker::spawn_clicker(Arc::clone(&active), Arc::clone(&cps));
        PhantomGui {
            active,
            cps,
            cursor_x: 0.0,
            cursor_y: 0.0,
            last_click: String::new(),
        }
    }
}

impl eframe::App for PhantomGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (x, y) = cursor::position();
        self.cursor_x = x;
        self.cursor_y = y;

        ctx.request_repaint_after(Duration::from_millis(50));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Phantom Click");

            ui.horizontal(|ui| {
                ui.label("Status:");
                if self.active.load(Ordering::Relaxed) {
                    ui.colored_label(egui::Color32::GREEN, "● CLICKING");
                } else {
                    ui.colored_label(egui::Color32::GRAY, "○ STOPPED");
                }
            });

            ui.add_space(10.0);

            let mut cps_val = self.cps.load(Ordering::Relaxed) as f32;
            ui.label("CPS:");
            if ui
                .add(egui::Slider::new(&mut cps_val, 1.0..=200.0).text("clicks/sec"))
                .changed()
            {
                self.cps.store(cps_val as u64, Ordering::SeqCst);
            }

            ui.horizontal(|ui| {
                if ui
                    .button(if self.active.load(Ordering::Relaxed) {
                        "■ STOP"
                    } else {
                        "▶ START"
                    })
                    .clicked()
                {
                    let active = self.active.load(Ordering::Relaxed);
                    self.active.store(!active, Ordering::SeqCst);
                }
                if ui.button("Click once").clicked() {
                    match clicker::click_once() {
                        Ok(()) => self.last_click = "✓ Clicked".to_string(),
                        Err(e) => self.last_click = format!("✗ {}", e),
                    }
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Cursor:");
                ui.monospace(format!("({:.0}, {:.0})", self.cursor_x, self.cursor_y));
            });

            if !self.last_click.is_empty() {
                ui.label(&self.last_click);
            }

            ui.add_space(20.0);

            ui.separator();
            ui.small("Global hotkeys: C=toggle  Z=+1  S=-1  Q/Esc=quit");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 280.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Phantom Click",
        options,
        Box::new(|_cc| Ok(Box::new(PhantomGui::new()))),
    )
}
