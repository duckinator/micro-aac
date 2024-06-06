use micro_aac::Interface;
use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("MicroAAC", native_options, Box::new(|cc| Box::new(MicroAACApp::new(cc))));

    /*
    for layout in LAYOUTS {
        if layout == iface.get_layout() {
            println!("*");
        } else {
            println!(" ");
        }
        println!("Up:    {:?}", layout[0]);
        println!("Left:  {:?}", layout[1]);
        println!("Right: {:?}", layout[2]);
        println!("Down:  {:?}", layout[3]);
        println!();
    }
    */

    /*for layout in LAYOUTS {
        for group in layout {
            for item in group {
                print!("{} ", item);
            }
            println!("");
        }
        println!("");
    }
    println!("Hello, world!");*/
}

#[derive(Default)]
struct MicroAACApp<'a> {
    iface: Interface<'a>,
}

impl MicroAACApp<'_> {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MicroAACApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("main").show(ui, |ui| {
                ui.label(self.iface.text());
                ui.end_row();

                ui.label("");
                if ui.add(egui::Button::new("^")).clicked() {
                }
                ui.label("");
                ui.end_row();

                if ui.add(egui::Button::new("<")).clicked() {
                }
                ui.label("");
                if ui.add(egui::Button::new(">")).clicked() {
                }
                ui.end_row();

                ui.label("");
                if ui.add(egui::Button::new("v")).clicked() {
                }
                ui.label("");
                ui.end_row();
            });
        });
    }
}
