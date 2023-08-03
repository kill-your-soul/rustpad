#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    ffi::OsStr,
    io::{Read, Write},
    path::PathBuf,
};

use eframe::egui;
use egui::{Align, Layout, ScrollArea, Vec2};

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

struct MyEguiApp {
    text: String,
    // picked_path: String,
    file_path: PathBuf,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        MyEguiApp {
            text: String::new(),
            // picked_path: String::new(),
            file_path: PathBuf::new(),
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let color = egui::Color32::from_rgba_premultiplied(0, 0, 0, 0);
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.hovered.bg_stroke.color = color;
        visuals.widgets.active.bg_stroke.color = color;
        visuals.widgets.inactive.bg_stroke.color = color;

        // visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 255 / 2);
        _cc.egui_ctx.set_visuals(visuals);
        Self::default()
    }

    fn open_dropped_files(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let _text = ctx.input(|i| {
                for file in &i.raw.dropped_files {
                    if let Some(path) = &file.path {
                        match self.read_from_file(path.to_str().unwrap(), frame) {
                            Ok(res) => self.text = res,
                            Err(_) => println!("Don't text file"),
                        };
                        self.file_path = path.to_path_buf();
                        println!("{:#?}", self.file_path);
                    }
                }
                // println!("{}", text);
            });
        }
    }

    fn handle_save_file(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input(|i| (i.key_pressed(egui::Key::S) && i.modifiers.ctrl && i.modifiers.shift)) {
            self.save_file_as();
            let title = format!("{} - {}", "Rustpad", self.file_path.display());
            frame.set_window_title(&title);
        } else if ctx.input(|i| (i.key_pressed(egui::Key::S) && i.modifiers.ctrl)) {
            save_text_to_file(&self.text, self.file_path.to_str().unwrap());
        }
    }

    fn save_file_as(&mut self) {
        match rfd::FileDialog::new()
            .set_file_name(
                self.file_path
                    .file_name()
                    .unwrap_or(&OsStr::new("new.txt"))
                    .to_str()
                    .unwrap(),
            )
            .set_directory(&self.file_path)
            .save_file()
        {
            None => return,
            Some(res) => {
                save_text_to_file(&self.text, &res.display().to_string());
                println!("{:#?}", res);
                self.file_path = res;
            }
        }
        // println!("{:#?}", res);
    }

    fn handle_open_file(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input(|i| (i.key_pressed(egui::Key::O) && i.modifiers.ctrl)) {
            self.open_file(frame);
        }
    }

    fn open_file(&mut self, frame: &mut eframe::Frame) {
        let path = std::env::current_dir().unwrap();
        match rfd::FileDialog::new().set_directory(&path).pick_file() {
            Some(res) => {
                self.text = self.read_from_file(res.to_str().unwrap(), frame).unwrap();
                self.file_path = res;
            }
            None => return,
        }
        // println!("{}", res.to_string_lossy());
    }

    fn read_from_file(
        &mut self,
        filename: &str,
        frame: &mut eframe::Frame,
    ) -> Result<String, std::io::Error> {
        // frame.
        let mut file = std::fs::File::open(filename)?;
        let mut contents = String::new();
        let err = file.read_to_string(&mut contents);
        match err {
            Ok(_) => {
                let title = format!("{} - {}", "Rustpad", filename);
                frame.set_window_title(&title);
                Ok(contents)
            }
            Err(err) => Err(err),
        }
    }
    fn notepad_ui(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(
            Layout::left_to_right(Align::Min).with_cross_align(Align::Max),
            |ui| {
                ui.style_mut().visuals.extreme_bg_color =
                    egui::Color32::from_rgba_premultiplied(0, 0, 0, 255 / 6);
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.text)
                        .margin(Vec2 { x: 0.5, y: 0.5 })
                        .desired_width(f32::INFINITY),
                )
            },
        );
    }

    fn handle_quit(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input(|i| (i.key_pressed(egui::Key::Q) && i.modifiers.ctrl)) {
            frame.close();
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // self.handle_save_as_file(ctx);
        self.handle_save_file(ctx, frame);
        self.handle_quit(ctx, frame);
        self.handle_open_file(ctx, frame);
        self.open_dropped_files(ctx, frame);
        // egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //     ui.label("Test");
        // });
        egui::CentralPanel::default()
            .frame(eframe::egui::Frame::default())
            .show(ctx, |ui: &mut egui::Ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.open_file(frame);
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        save_text_to_file(&self.text, self.file_path.to_str().unwrap());
                        ui.close_menu();
                    }
                    if ui.button("Save as").clicked() {
                        self.save_file_as();
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        ui.close_menu();
                        frame.close();
                        return;
                    }
                });
                // ui.reset_style();
                ScrollArea::vertical().show(ui, |ui| self.notepad_ui(ui));
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
