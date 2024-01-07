#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::egui;


pub fn start()-> Result<(), eframe::Error> {

let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([320.0,240.0]),
    ..Default::default()
};
eframe::run_native(
    "My egui App",
    options,
    Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Box::<MyApp>::default()
    }),

)
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Alice".to_owned(),
            age: 20,
        }
    }
    
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui App");
            ui.horizontal(|ui| {
                let name_Label = ui.label("Your name is: ");
                ui.text_edit_multiline(&mut self.name)
                .labelled_by(name_Label.id);
            });
            ui.add(egui::Slider::new( & mut self.age, 0..=123).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello, '{}', age {}", self.name, self.age));
            ui.image("../resource/background.png")
        });
    }
}