#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::io::{Read, Write};

use eframe::egui;
use egui::{Align, Layout, Vec2};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(
        "Rustpad",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
}

// #[derive(Default)]
struct MyEguiApp {
    text: String,
    // picked_path: String,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        MyEguiApp {
            text: String::new(),
            // picked_path: String::new(),
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // let mut visuals = egui::Visuals::dark();
        // visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 255 / 2);
        // _cc.egui_ctx.set_visuals(visuals);
        Self::default()
    }

    fn open_dropped_files(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let _text = ctx.input(|i| {
                for file in &i.raw.dropped_files {
                    if let Some(path) = &file.path {
                        self.text = self.read_from_file(path.to_str().unwrap(), frame).unwrap();
                    }
                }
                // println!("{}", text);
            });
        }
    }

    fn handle_save_file(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| (i.key_pressed(egui::Key::S) && i.modifiers.ctrl)) {
            self.save_file();
        }
    }

    fn save_file(&mut self) {
        let path = std::env::current_dir().unwrap();
        let res = rfd::FileDialog::new()
            .set_file_name("test.txt")
            .set_directory(&path)
            .save_file()
            .unwrap();
        println!("{:#?}", res);
        save_text_to_file(&self.text, &res.display().to_string());
    }

    fn handle_open_file(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input(|i| (i.key_pressed(egui::Key::O) && i.modifiers.ctrl)) {
            self.open_file(frame);
        }
    }

    fn open_file(&mut self, frame: &mut eframe::Frame) {
        let path = std::env::current_dir().unwrap();
        let res = rfd::FileDialog::new()
            .set_directory(&path)
            .pick_file()
            .unwrap();
        println!("{}", res.to_string_lossy());
        self.text = self.read_from_file(res.to_str().unwrap(), frame).unwrap();
    }

    fn read_from_file(
        &mut self,
        filename: &str,
        frame: &mut eframe::Frame,
    ) -> Result<String, std::io::Error> {
        let title = format!("{} - {}", "Rustpad", filename);
        frame.set_window_title(&title);
        let mut file = std::fs::File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    // fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
    //     let key = ctx.input(|i| i.keys_down);

    // }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_save_file(ctx);
        self.handle_open_file(ctx, frame);
        self.open_dropped_files(ctx, frame);
        egui::CentralPanel::default()
            .frame(eframe::egui::Frame::default())
            .show(ctx, |ui: &mut egui::Ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // println!("Open file");
                        self.open_file(frame);
                    }
                    if ui.button("Save").clicked() {
                        // println!("Save file")
                        self.save_file();
                    }
                });
                ui.reset_style();
                ui.with_layout(
                    Layout::left_to_right(Align::Max).with_cross_align(Align::Min),
                    |ui| {
                        ui.style_mut().visuals.extreme_bg_color =
                            egui::Color32::from_rgba_premultiplied(0, 0, 0, 255 / 6);
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(&mut self.text)
                                .margin(Vec2 { x: 0.5, y: 0.5 }),
                        );
                    },
                );
            });
    }
}

fn save_text_to_file(text: &str, filename: &str) {
    if let Ok(mut file) = std::fs::File::create(filename) {
        if let Err(err) = file.write_all(text.as_bytes()) {
            eprintln!("Failed to write to file: {}", err);
        }
    } else {
        eprintln!("Failed to create file");
    }
}
