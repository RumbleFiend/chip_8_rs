#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epaint::Vec2};
use chip8_core::*;
use std::env;
use std::fs::File;
use std::io::Read;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

struct Emulator {
    chip8 :Emu,
    rom: Option<File>,
}

impl Default for Emulator{
    fn default() -> Self {
        Self { chip8: Emu::new(), rom: None }
    }
}

impl eframe::App for Emulator{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.chip8.tick();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CHIP_8 Emulator");
            ui.add_sized([40.0, 20.0],egui::Label::new("CHIP-8 Emulator"));
            /*ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));*/
        });
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let mut r = File::open(&args[1]).expect("Unable to open file"); // rom
    let mut buffer = Vec::new();                                         
    r.read_to_end(&mut buffer).unwrap();

    let mut emu = Emulator {rom : Some(r),..Emulator::default()};
    emu.chip8.load(&buffer);

    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32));

    eframe::run_native(
        "Chip-8 Emulator",
        options,
        Box::new(|_cc| Box::new(emu)),
    );
}