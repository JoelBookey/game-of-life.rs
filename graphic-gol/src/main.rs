use gol_core::Game;
use raylib::prelude::*;

const GAME_WIDTH: usize = (ASPECT_RATIO * GAME_HEIGHT as f32) as usize;
const GAME_HEIGHT: usize = 100;

const WINDOW_WIDTH: i32 = 1080;
const WINDOW_HEIGHT: i32 = 720;

const ASPECT_RATIO: f32 = 1080.0 / 720.0;

const PIXEL_WIDTH: i32 = (WINDOW_WIDTH as f64 / GAME_WIDTH as f64) as i32;
const PIXEL_HEIGHT: i32 = (WINDOW_HEIGHT as f64 / GAME_HEIGHT as f64) as i32;

fn main() {
    let _ = std::panic::take_hook();
    println!("GAME_WIDTH: {GAME_WIDTH}\nGAME_HEIGHT: {GAME_HEIGHT}");
    let mut game: Game<GAME_WIDTH, GAME_HEIGHT> = Game::new();
    game.change(7, 20);
    game.change(8, 21);
    game.change(8, 22);
    game.change(7, 22);
    game.change(6, 22);

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .build();

    let mut fps = 60;
    rl.set_target_fps(60);

    let mut backup: Option<[[bool; GAME_WIDTH]; GAME_HEIGHT]> = None;
    let mut play = false;
    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            handle_input(&mut game, rl.get_mouse_x(), rl.get_mouse_y());
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            play ^= true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            backup = Some(game.save_grid())
        }
        if rl.is_key_pressed(KeyboardKey::KEY_L) {
            if backup.is_some() {
                game.load_grid(backup.unwrap());
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            fps += 5;
            rl.set_target_fps(fps);
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            fps -= 5;
            rl.set_target_fps(fps);
        }
        if play {
            game.tick();
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        draw_game(&mut d, &game);
        d.draw_fps(0, 0);
    }
}

fn handle_input(game: &mut Game<GAME_WIDTH, GAME_HEIGHT>, x: i32, y: i32) {
    let scaled_x = ((x as f64 / WINDOW_WIDTH as f64) * GAME_WIDTH as f64) as usize;
    let scaled_y = ((y as f64 / WINDOW_HEIGHT as f64) * GAME_HEIGHT as f64) as usize;
    if scaled_y < 100 {
        game.change(scaled_x, scaled_y);
    }
}

fn draw_game(handle: &mut RaylibDrawHandle, game: &Game<GAME_WIDTH, GAME_HEIGHT>) {
    let grid = game.show();
    let x_pos = (0..GAME_WIDTH)
        .map(|pos| (pos as f64 * WINDOW_WIDTH as f64 / GAME_WIDTH as f64) as i32)
        .collect::<Vec<i32>>();
    for row in 0..GAME_HEIGHT {
        let y = (row as f64 * WINDOW_HEIGHT as f64 / GAME_HEIGHT as f64) as i32;
        for col in 0..GAME_WIDTH {
            if grid[row][col] {
                handle.draw_rectangle(
                    *x_pos.get(col).unwrap(),
                    y,
                    PIXEL_WIDTH,
                    PIXEL_HEIGHT,
                    Color::BLACK,
                );
            }
        }
    }
}
