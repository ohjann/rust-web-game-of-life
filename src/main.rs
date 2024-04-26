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
    let rowLength = 100;
    let matrix = vec![false; rowLength * rowLength];

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { class: "main", padding: "0.5rem", position: "relative",
            div { class: "container", max_width: "{rowLength*10}px",
                 for (i, item) in matrix.iter().enumerate() {
                     Square {}
                 }
            }
        }
    }
}

#[component]
fn Square() -> Element {
    let mut active = use_signal(|| false);
    rsx! {
        div { onclick: move |_| {
            active.set(if active() { false } else { true });
        }, class: if active() { "square active" } else { "square" } }
    }
}
