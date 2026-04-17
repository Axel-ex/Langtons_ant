use clap::Parser;
use figlet_rs::Toilet;
use macroquad::{color, prelude::*};

pub mod ant;
pub mod automaton;
pub mod rule;

use automaton::Automaton;
use rule::Rule;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value_t = 30)]
    tick_rate_ms: u64,

    #[arg(short, long, default_value_t = 10.0)]
    cell_size: f32,

    #[arg(short, long, default_value_t = 0)]
    skip_iter: u64,

    #[arg(short, long, default_value = "RL")]
    rule: Rule,
}

#[macroquad::main("Langton's ants")]
async fn main() {
    let args = Args::parse();

    let mono_9_font = Toilet::mono9().unwrap();
    println!("{}", mono_9_font.convert("Langton's Ants").unwrap());

    let mut automaton = Automaton::new(args.rule);
    automaton.skip_n_iter(args.skip_iter);

    let tick_seconds = args.tick_rate_ms as f32 / 1000.0;
    let mut accumulated = 0.0;

    loop {
        accumulated += get_frame_time();

        while accumulated >= tick_seconds {
            automaton.update();
            accumulated -= tick_seconds;
        }

        render(&automaton, args.cell_size);
        next_frame().await;
    }
}

pub fn render(automaton: &Automaton, cell_size: f32) {
    // Divide the screen into fixed sized cells -> viewport coordinates
    let cols = (screen_width() / cell_size) as i32;
    let rows = (screen_height() / cell_size) as i32;

    let (ax, ay) = automaton.get_ant_position();

    // center viewport on ant -> world coordinates of the top left corner
    let x_origin = ax - cols / 2;
    let y_origin = ay - rows / 2;

    for row in 0..rows {
        for col in 0..cols {
            let current_x = x_origin + col;
            let current_y = y_origin + row;

            // to screen space
            let px = col as f32 * cell_size;
            let py = row as f32 * cell_size;

            let color = choose_color(automaton.cell_state(current_x, current_y));

            draw_rectangle(px, py, cell_size, cell_size, color);
        }
    }

    // draw ant in screen space
    let ant_px = (ax - x_origin) as f32 * cell_size;
    let ant_py = (ay - y_origin) as f32 * cell_size;

    draw_rectangle(ant_px, ant_py, cell_size, cell_size, RED);
    draw_text(
        format!("Iteration: {}", automaton.iteration()).as_str(),
        40.0,
        screen_height() - 40.0,
        30.0,
        DARKGRAY,
    );
    draw_text(
        format!("Rule: \"{}\"", automaton.rule_name()).as_str(),
        40.0,
        screen_height() - 60.0,
        30.0,
        DARKGRAY,
    );
}

fn choose_color(cell_state: usize) -> Color {
    if cell_state == 0 {
        return WHITE;
    }

    // select the hue based on cell state (step of 0,1, max 10 different colors)
    let h = (cell_state as f32 * 0.1) % 1.0;
    const S: f32 = 0.8;
    const L: f32 = 0.8;

    color::hsl_to_rgb(h, S, L)
}
