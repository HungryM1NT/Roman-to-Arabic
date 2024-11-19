use eframe::egui;
use egui::{FontFamily, FontId, TextStyle};
use std::collections::BTreeMap;

mod program;

fn main() -> eframe::Result {
    // Стартовые настройки окна
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    // Запуск приложения (название окна, настройки окна, приложение)
    eframe::run_native(
        "Название окна",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
        // Box::new(|cc| {
        //     Ok(Box::<MyApp>::default())
        // }),
    )
}


// Данные, которые будут изменяться
struct MyApp {
    arabic_number: String,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_text_styles(&cc.egui_ctx);
        Self {
            arabic_number: "".to_owned(),
        }
    }
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace};

    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::new(25.0, Monospace)),  // Top heading
        (TextStyle::Body, FontId::new(15.0, Monospace)),     // Text after top heading
    ]
    .into();
    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}

// Данные, которые будут выставлены изначально
impl Default for MyApp {
    fn default() -> Self {
        Self {
            arabic_number: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let a = |x: i32| {x * 2}; 
        // println!("{}", a(5));
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Roman to arabic converter");
            ui.add_space(15.);
            ui.horizontal(|ui| {
                let arabic_num = ui.label("Enter arabic number: ");
                ui.text_edit_singleline(&mut self.arabic_number)
                    .labelled_by(arabic_num.id);
            });
            ui.add_space(15.);
            match program::roman_to_arabic(&mut self.arabic_number.to_uppercase()) {
                Ok(answer) => {
                    ui.label(format!("Arabic number: {}", answer));
                },
                Err(err) => {
                    ui.label(format!("Error: {err}"));
                }
            }

        });
    }
}