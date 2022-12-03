#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chip8_core::*;
use eframe::epaint::{vec2, Mesh, Mesh16, Rect};
use eframe::{egui, epaint::Vec2};
use egui::*;
use plot::{Corner, Legend, Line, Plot,Text, PlotImage, PlotPoint, PlotPoints, CoordinatesFormatter};
use std::env;
use std::fs::File;
use std::io::Read;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 10;

struct Emulator {
    chip8: Emu,
    rom: Option<File>,
    pause: bool,
}

impl Default for Emulator {
    fn default() -> Self {
        Self {
            chip8: Emu::new(),
            rom: None,
            pause: false,
        }
    }
}

impl eframe::App for Emulator {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::BLACK
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { pause, .. } = self;

        for _ in 0..TICKS_PER_FRAME {
            if !*pause{
            self.chip8.tick();
            }
        }
        self.chip8.tick_timers();

        let screen_buf = self.chip8.get_display();

        egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.heading("CHIP-8 EMULATOR:");
            ui.vertical(|ui|{
                ui.checkbox(pause,"Pause");
            });
            //ui.heading(format!("SP = {}",self.chip8.get_sp()));
            draw_screen(screen_buf,ui);
        });
        
        ctx.request_repaint();
    }
}

fn draw_screen(buf: &[bool], ui: &mut egui::Ui) -> Response{

    let plot = Plot::new("TEST Plot")
        .width(WINDOW_WIDTH as f32)
        .height(WINDOW_HEIGHT as f32)
        .allow_drag(false)
        .allow_zoom(false)
        .allow_boxed_zoom(false)
        .show_x(false)
        .show_y(false)
        .show_axes([false,false])
        .allow_scroll(true)
        .view_aspect(1.0)
        .data_aspect(-1.0)
        .min_size(Vec2 { x: WINDOW_WIDTH as f32, y: WINDOW_HEIGHT as f32 })
        .coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default());

    plot.show(ui, |plot_ui| {
        for (i, pixel) in buf.iter().enumerate() {
            if *pixel {
                let x = (i % SCREEN_WIDTH) as f64;
                let y = (i / SCREEN_WIDTH) as f64;
                
                // Draw a rectangle at (x,y), scaled up by SCALE value
                let pos = PlotPoint {
                    x: x * (SCALE as f64),
                    y: y * (SCALE as f64),
                };
                plot_ui.text(Text::new(pos, "⏹").color(egui::Rgba::WHITE));
            }
        }
    }).response
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

    let mut emu = Emulator {
        rom: Some(r),
        ..Emulator::default()
    };
    emu.chip8.load(&buffer);

    let options = eframe::NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        //decorated: false,
        // To have rounded corners we need transparency:
        transparent: true,
        min_window_size: Some(egui::vec2(1370.0,720.0)),
        ..Default::default()
    };

    eframe::run_native("Chip-8 Emulator",
     options,
     Box::new(|_cc| Box::new(emu)));

}


