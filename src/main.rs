#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use std::cmp;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(App);
}

fn count_left_right(
    index: usize,
    row_length: usize,
    matrix: &Vec<bool>,
    should_count_center: bool,
) -> usize {
    let left_index = if index % row_length == 0 {
        // if we're at an edge don't wrap around
        index
    } else {
        index.saturating_sub(1)
    };
    let right_index = if (index + 1) % row_length == 0 {
        // if we're at an edge don't wrap around
        index + 1
    } else {
        cmp::min(index + 2, matrix.len())
    };
    let count = matrix
        .get(left_index..right_index)
        .unwrap_or(&[])
        .iter()
        .filter(|&&n| n == true)
        .count();
    if !should_count_center {
        if let Some(&center) = matrix.get(index) {
            if center && count > 0 {
                return count - 1;
            }
        }
    }
    count
}

fn count_alive_neighbours(index: usize, row_length: usize, matrix: &Vec<bool>) -> usize {
    let row_above = if index >= row_length {
        count_left_right(index - row_length, row_length, matrix, true)
    } else {
        0
    };
    let center = count_left_right(index, row_length, matrix, false);
    let bottom = count_left_right(index + row_length, row_length, matrix, true);
    center + bottom + row_above
}

fn square_state(index: usize, row_length: usize, matrix: &Vec<bool>) -> bool {
    let current_state = &matrix[index];
    let living_neighbour_count = count_alive_neighbours(index, row_length, matrix);
    if *current_state {
        // square is alive
        living_neighbour_count == 2 || living_neighbour_count == 3
    } else {
        // square is dead
        living_neighbour_count == 3
    }
}

#[component]
fn App() -> Element {
    let row_length = 100;
    let mut matrix = use_signal(|| vec![false; row_length * row_length]);

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { class: "main", padding: "0.5rem", position: "relative",
            button { onclick: move |_| {
                let new_vec: Vec<bool> = matrix().iter().enumerate().map(|(i, _)| square_state(i, row_length, &*matrix.read())).collect();
                *matrix.write() = new_vec;
            }, "NEXT"}
            div { class: "container", max_width: "{row_length*10}px",
                 for (i, &item) in matrix().iter().enumerate() {
                    div { onclick: move |_| {
                        matrix.write()[i] = !item;
                        let state = square_state(i, row_length, &*matrix.read());
                        log::info!("{i} {state}");
                    }, class: if item { "square active" } else { "square" } }
                 }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_count_left_and_right() {
        assert_eq!(
            count_left_right(1, 4, &vec![false, false, false, true], false),
            0
        );
        assert_eq!(
            count_left_right(1, 4, &vec![false, false, true, false], false),
            1
        );
        assert_eq!(
            count_left_right(1, 4, &vec![false, false, true, false], false),
            1
        );
        assert_eq!(
            count_left_right(1, 4, &vec![true, true, true, true], false),
            2
        );
        assert_eq!(
            count_left_right(2, 4, &vec![true, true, true, true], false),
            2
        );
        assert_eq!(
            count_left_right(4, 4, &vec![true, true, true, true], false),
            0
        );
        assert_eq!(
            count_left_right(0, 4, &vec![true, true, true, true], false),
            1
        );

        assert_eq!(
            count_left_right(0, 4, &vec![true, true, true, true], true),
            2
        );
        assert_eq!(
            count_left_right(1, 4, &vec![true, true, true, true], true),
            3
        );
    }

    #[test]
    fn can_count_neighbours() {
        let row_length = 3;
        let mut matrix = vec![
            // corresponds to 3x3 matrix
            false, false, false, false, false, false, false, false, false,
        ];
        assert_eq!(count_alive_neighbours(4, row_length, &matrix), 0);

        matrix = vec![true, false, false, false, false, false, false, false, false];
        assert_eq!(count_alive_neighbours(4, row_length, &matrix), 1);

        matrix = vec![true, false, false, false, false, false, false, false, true];
        assert_eq!(count_alive_neighbours(4, row_length, &matrix), 2);

        matrix = vec![true, true, true, true, true, true, true, true, true];
        assert_eq!(count_alive_neighbours(4, row_length, &matrix), 8);

        matrix = vec![true, true, true, true, false, true, true, true, true];
        assert_eq!(count_alive_neighbours(4, row_length, &matrix), 8);

        matrix = vec![false, false, false, false, true, false, false, false, false];
        assert_eq!(count_alive_neighbours(4, row_length, &matrix), 0);

        matrix = vec![true, true, true, false, false, false, false, false, false];
        assert_eq!(count_alive_neighbours(1, row_length, &matrix), 2);

        matrix = vec![true, true, true, false, false, false, false, false, false];
        assert_eq!(count_alive_neighbours(0, row_length, &matrix), 1);
    }
}
