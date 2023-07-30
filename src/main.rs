// src/main.rs
use fltk::{prelude::*, *};
mod amazing_logger_ui;

fn main() {
    let app = app::App::default();
    let mut ui = amazing_logger_ui::UserInterface::make_window();
    

    app.run().unwrap();
}