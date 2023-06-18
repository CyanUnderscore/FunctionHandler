#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::epaint::ahash::{HashMap, RandomState};
use eframe::epaint::{ColorImage, TextureId};
use eframe::{egui, Error};
use plotters::prelude::*;
use std::path::Path;
use std::fs::File;
use base64;
use std::io::Read;
use image::{GenericImageView, ImageBuffer, Rgba};
use egui::epaint::textures::TextureManager;
use egui::widgets::{TextEdit};
use egui_extras::RetainedImage;
pub type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<(), eframe::Error> {
    funcManager("".to_owned());
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Debug)]
enum PossibleTypes<'a> {
    int(i32),
    float(f64),
    bool(bool),
    str(&'a str)
}

struct MyApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    image : RetainedImage,
    function: String
}


impl Default for MyApp {
    fn default() -> Self {
        Self { allowed_to_close: false, 
            show_confirmation_dialog: false, 
            image: RetainedImage::from_color_image(
                "plot.png",
                get_value(draw_func(Path::new("/home/cyansky/Documents/rust/FunctionHandler/plot.png"))),
            ),
            function: String::from("2*x-1")
        }
    }
}

impl MyApp {
    fn as_tuples(&self) -> Vec<(Box<str>, PossibleTypes)> {
        vec![("allowed_to_close".to_owned().into_boxed_str(), PossibleTypes::bool(self.allowed_to_close)),
        ("show_confirmation_dialog".to_owned().into_boxed_str(), PossibleTypes::bool(self.show_confirmation_dialog)),]
    }
}

impl eframe::App for MyApp {

    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("lets try something");
            ui.heading("This is an image:");
            ui.horizontal(|ui| {});
            self.image.show(ui);
            egui::SidePanel::right("Section 1").show(ctx, |ui| {
                ui.horizontal(|ui|{
                    ui.label("y = ");
                    ui.add(TextEdit::singleline(&mut self.function).hint_text("2x-1"));
                });
                if ui.button("submit").clicked() {
                    println!("fn sub");
                    funcManager(self.function.clone());
                    draw_func(Path::new("/home/cyansky/Documents/rust/FunctionHandler/plot.png"));
                }
            });
            
        });

        /*egui::Window::new("Self status")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        for (title, info) in self.as_tuples() {
                            let read : String = format!(" {} : {:?}", title, info);
                            ui.label(read);
                        }
                });
            });*/

        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}

fn funcManager(function : String) -> Result<String, Box<dyn std::error::Error>> {

    let path = "plot.png";
    let root = BitMapBackend::new(path, (400, 375)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(20)
        .y_label_area_size(20)
        .build_cartesian_2d(0f32..20f32, -10f32..10f32)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .draw()?;

    chart.draw_series(LineSeries::new(
        do_the_math(function),
        &RED,
    ))?;

    Ok(path.to_owned())
}



fn draw_func(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

fn do_the_math(function : String) -> Vec<(f32, f32)> {
    let mut cur_str = "".to_owned();
    let mut val : Vec<f64> = vec![];
    let mut signs : Vec<char> = vec![];
    let mut x_places : Vec<i32> = vec![];
    let mut x = 0.0;
    let mut i = 0;
    for chr in function.chars(){
        println!("{}", chr);
        match chr as u32 {
            42..=47 => {
                val.push(get_value(cur_str.parse()));
                signs.push(chr);
                cur_str = "".to_owned();
            },
            48..=57 => {
                cur_str += chr.to_string().as_str(); 
                i+=1;}
            120 => {
                cur_str += &x.to_string().as_str();
                x_places.push(i);
                i+=1;
            }
            _ => println!("what the fuck is this shit ? : {}, char = {}", chr, chr as u32)
        } if chr == function.chars().last().unwrap(){
            val.push(get_value(cur_str.parse()));
        }
    }
    let mut res : Vec<(f32, f32)> = vec![];
    let backup_val = val.clone();
    let backup_sign = signs.clone();
    let mut equilibre:i32 = 1;
    let mut iter = 0;
    for i in 0..=20 {
        signs = backup_sign.clone();
        val = backup_val.clone();
        for iter in 0..x_places.len() {
            let x_index : usize = x_places[iter] as usize;
            val[x_index] = i as f64;
        }
        println!("{:?}, {:?}, {:?}", &val, &signs, x_places);
        equilibre = 1;
        while iter < signs.len() {
            println!("{}, {}", signs[iter], signs[iter] == '*' || signs[iter] == '/');
            if signs[iter] == '*' || signs[iter] == '/' {
                if signs[iter] == '*' {
                    val[iter + equilibre as usize -1] *= val[iter + equilibre as usize];
                    val.remove(iter+equilibre as usize);
                    equilibre-=1;
                    
                } else if signs[iter] == '/' {
                    val[iter + equilibre as usize -1] /= val[iter + equilibre as usize];
                        val.remove(iter+equilibre as usize);
                        equilibre-=1;
                }
                println!("{:?}, {:?}, {:?} */", &val, &signs, x_places);
            } 
            iter +=1;
        }
        iter = 0;
        while iter < signs.len() {
            if signs[iter] == '+' || signs[iter] == '-' {
                if signs[iter] == '+' {
                    val[0] += val[1];
                    val.remove(1);
                    equilibre-=1;
                } else if signs[iter] == '-' {
                    val[0] -= val[1];
                    val.remove(1);
                    equilibre-=1;
                }
                println!("{:?}, {:?}, {:?} +-", &val, &signs, x_places);
            }
            iter+=1;
            
        }
        iter = 0;
        println!("result pour x = {} : {:?}",i, val);
        if val.len() > 0 {
            let tuple = (i as f32, val[0] as f32);
            res.push(tuple);
        } else {
            let tuple = (i as f32, 0.0 as f32);
            res.push(tuple);
        }
    }
    println!("{:?}", res);
    return res;
}

fn get_value<T, E>(res : Result<T, E>) -> T {
    match res {
        Ok(r) => return r,
        Err(e) => panic!("error")
    }
}
