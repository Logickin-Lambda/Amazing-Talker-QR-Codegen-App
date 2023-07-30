// src/main.rs
use fltk::{app, image::SvgImage, prelude::*};
use qrcode::QrCode;
use qrcode::render::svg;

mod amazing_logger_ui;

fn main() {
    let app = app::App::default();
    let mut ui = amazing_logger_ui::UserInterface::make_window();
    let mut qr_code_field = ui.qr_code_field.clone();
    let url_path_input: fltk::input::Input = ui.url_path.clone();
    
    ui.generate_btn.set_callback(move |_| {

        let url_path =  url_path_input.value();
        let raw_qr = QrCode::new(url_path).unwrap();
        let image = raw_qr.render()
            .min_dimensions(600, 600)
            .dark_color(svg::Color("#bbddff"))
            .light_color(svg::Color("#000000"))
            .build();
        
        // Have to set an empty string here; otherwise, the image simple don't load
        let image = SvgImage::from_data(&image).unwrap();
        qr_code_field.set_label("");
        qr_code_field.set_image(Some(image));
    });

    app.run().unwrap();
}