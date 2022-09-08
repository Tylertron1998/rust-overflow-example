#![forbid(unsafe_code)]

use std::ops::{Add, Index};
use eframe::{egui, Frame};
use egui_extras::RetainedImage;
use crate::egui::{Color32, ColorImage, Context, Vec2};




fn main() {
    let mut options = eframe::NativeOptions::default();

    options.initial_window_size = Option::Some(Vec2::new(1000.0, 1000.0));

    eframe::run_native("DotBOX",
    options,
    Box::new(|_|Box::new(DotboxApp::default())))
}

struct DotboxApp {
    base_image: RetainedImage,
    buffer: [u8; 1000 * 1000 * 4]
}

impl Default for DotboxApp {
    fn default() -> Self {

        let buffer = [0; 1000 * 1000 * 4];

        return Self {
            buffer,
            base_image: RetainedImage::from_image_bytes("image",  &buffer).unwrap()
        };
    }
}

impl eframe::App for DotboxApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            // for x in 200..800 {
            //     for y in 200..800 {
            //         let colour = Color32::WHITE;
            //         let index = x * y;
            //         self.buffer[index] = colour.r();
            //         self.buffer[index + 1] = colour.g();
            //         self.buffer[index + 2] = colour.b();
            //         self.buffer[index + 3] = colour.a();
            //     }
            // }

            //self.base_image.show(ui);
        });
    }
}