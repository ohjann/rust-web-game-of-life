#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use ndarray::prelude::*;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(App);
}

#[component]
fn App() -> Element {
    let matrix = Array2::<bool>::from_elem((100, 100), false);
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { class: "main", padding: "0.5rem", position: "relative",
             for row in matrix.rows() {
                 div {  class: "row",
                     for col in row {
                         Square { active: *col }
                     }
                 }
             }
        }
    }
}

#[component]
fn Square(active: bool) -> Element {
    let active_class = if active { "active" } else { "" };
    rsx! {
        div { class: "square {active_class}" }
    }
}
