#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(App);
}

#[component]
fn App() -> Element {
    let rowLength = 100;
    let mut matrix = use_signal(|| vec![false; rowLength * rowLength]);

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { class: "main", padding: "0.5rem", position: "relative",
            div { class: "container", max_width: "{rowLength*10}px",
                 for (i, &item) in matrix().iter().enumerate() {
                    div { onclick: move |_| {
                        matrix.write()[i] = !item;
                    }, class: if item { "square active" } else { "square" } }
                 }
            }
        }
    }
}
