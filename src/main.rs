#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, Error};
use plotters::prelude::*;
use core::panic;
use std::path::Path;
use egui::widgets::{TextEdit, DragValue};
use egui_extras::RetainedImage;
pub type Result<T, E = Error> = std::result::Result<T, E>;
use FunctionHandler::expr;
fn main() -> Result<(), eframe::Error> {
    funcManager("".to_owned(), (0, 20));
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
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
    function: String,
    low : i32,
    high : i32
}


impl Default for MyApp {
    fn default() -> Self {
        Self { allowed_to_close: false, 
            show_confirmation_dialog: false, 
            image: RetainedImage::from_color_image(
                "plot.png",
                draw_func(Path::new("/home/cyansky/Documents/rust/FunctionHandler/plot.png")).unwrap(),
            ),
            function: String::from("2*x-1"),
            low: 0,
            high : 20,
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
            ui.heading("cyan function calculator");
            ui.heading("graphic representation:");
            self.image.show(ui);
            egui::SidePanel::right("Section 1").show(ctx, |ui| {
                ui.horizontal(|ui|{
                    ui.label("y = ");
                    ui.add(TextEdit::singleline(&mut self.function).hint_text("2x-1"));
                });
                ui.horizontal(|ui|{
                    ui.label("min x:");
                    ui.add(DragValue::new(&mut self.low).speed(1));
                    ui.label("max x:");
                    ui.add(DragValue::new(&mut self.high).speed(1));

                });
                if ui.button("submit").clicked() {
                    println!("fn sub");
                    funcManager(self.function.clone(), (self.low, self.high));
                    draw_func(Path::new("/home/cyansky/Documents/rust/FunctionHandler/plot.png"));
                    self.image = RetainedImage::from_color_image(
                        "plot.png",
                        draw_func(Path::new("/home/cyansky/Documents/rust/FunctionHandler/plot.png")).unwrap(),
                    )
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

fn funcManager(function : String, domaine_def : (i32, i32)) -> Result<String, Box<dyn std::error::Error>> {

    let line = do_the_math(function, domaine_def);
    let mut min:f32 = 0.0;
    let mut max:f32 = 0.0;
    for i in &line{
        if i.1 > max {max = i.1};
        if i.1 < min {min = i.1};
    }
    min-=1.0;
    max+=1.0;


    let path = "plot.png";
    let root = BitMapBackend::new(path, (400, 375)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(20)
        .y_label_area_size(20)
        .build_cartesian_2d((domaine_def.0 as f32)..(domaine_def.1 as f32), min..max)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .draw()?;

    chart.draw_series(LineSeries::new(
        line,
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


fn recursive_calcul(vec : &Vec<String>) -> f32{
    if vec[0] != "("{
        let result : &String = &vec[0];
        return result.parse().unwrap();
    }
    let mut result : f32 = 0.0;
    let operator:&str = &vec[1];
    match operator {
        "+" => {
            result = recursive_calcul(vec[3..vec.len()/2]) + recursive_calcul(vec);
        },
        "-" => {
            todo!()
        },
        "x" => {
            todo!()
        },
        "/" => {
            todo!()
        },
        _=>panic!("wrong operator {}", operator)
    }
    result
}

fn do_the_math(function : String, domaine_def : (i32, i32)) -> Vec<(f32, f32)> {

    let s_char : Vec<char> = expr(&function)
        .to_string()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect();
    let mut s:Vec<String> = s_char.iter().map(|&c| c.to_string()).collect();
    
    let mut res: Vec<(f32,f32)> = vec![];
    let mut x_indexs: Vec<usize> = vec![];
    let mut open_parenthesis_indexs: Vec<usize> = vec![];

    let definition = 100;
    for i in 0..s.len(){
        if s[i] == "x"{
            x_indexs.push(i);
        }
        else if s[i] == "("{
            open_parenthesis_indexs.push(i);
        }
    }
    
    // calculate y for every x with the precision being (definition) 
    //exemple : definition=1 x = 1, 2, 3, ... | definition=100 1, 1.01, 1.02, ...
    
    for i in (domaine_def.0 * definition)..=(domaine_def.1 * definition) {
        for x in &x_indexs{
            s[*x]=i.to_string();
        }
        for j in &open_parenthesis_indexs{
            if s[*j+2] == "("{
                todo!()
            } else {
                todo!()
            }
        }
    }

    // once the function is sliced its processed
    println!("{:?}", res);
    return res;
}

