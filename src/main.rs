use std::time::Duration;

use clap::Parser;
use figlet_rs::Toilet;
use macroquad::prelude::*;

use crate::automaton::automaton::Automaton;

pub mod automaton;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value_t = 200)]
    tick_rate_ms: u64,

    #[arg(short, long, default_value_t = 10.0)]
    cell_size: f32,

    #[arg(short, long, default_value_t = 0)]
    skip_iter: u64,
}

#[macroquad::main("Langton's ants")]
async fn main() {
    let args = Args::parse();
    let mono_9_font = Toilet::mono9().unwrap();
    println!("{}", mono_9_font.convert("Langton's Ants").unwrap());

    let mut automaton = Automaton::new();
    automaton.skip_n_iter(args.skip_iter);

    loop {
        automaton.update();
        render(&automaton, args.cell_size);
        next_frame().await;
        std::thread::sleep(Duration::from_millis(args.tick_rate_ms));
    }
}

pub fn render(automaton: &Automaton, cell_size: f32) {
    clear_background(WHITE);

    let cols = (screen_width() / cell_size) as i32;
    let rows = (screen_height() / cell_size) as i32;

    let (ax, ay) = automaton.get_ant_position();

    // center viewport on ant
    let start_x = ax - cols / 2;
    let start_y = ay - rows / 2;

    for sy in 0..rows {
        for sx in 0..cols {
            let wx = start_x + sx;
            let wy = start_y + sy;

            let px = sx as f32 * cell_size;
            let py = sy as f32 * cell_size;

            let color = if automaton.is_black(wx, wy) {
                BLACK
            } else {
                WHITE
            };

            draw_rectangle(px, py, cell_size, cell_size, color);
        }
    }

    // draw ant in screen space
    let ant_px = (ax - start_x) as f32 * cell_size;
    let ant_py = (ay - start_y) as f32 * cell_size;

    draw_rectangle(ant_px, ant_py, cell_size, cell_size, RED);
    draw_text(
        format!("Iteration: {}", automaton.iteration()).as_str(),
        40.0,
        screen_height() - 40.0,
        30.0,
        DARKGRAY,
    );
}
