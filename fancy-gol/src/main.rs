use gol_core::{Game, GAME_SIZE};
use std::io::{self, stdout, Write};
use std::panic::set_hook;
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor::{position, Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> io::Result<()> {
    println!(
        "
Q to quit.
T to go to next gen.
N to go to see gen number.
C to reset.
Enter to flip cell
Play to play/pause
DO NOT RESIZE TERMINAL!!!!!!!!!!!!!!!!!!!!!!!!"
    );

    thread::sleep(Duration::from_secs(3));

    let t_size = size().unwrap();
    if GAME_SIZE >= t_size.1 as usize || GAME_SIZE > t_size.0 as usize {
        panic!("terminal is too small.");
    }

    let mut stdout = stdout();
    let mut game = Game::new();
    execute!(stdout, EnterAlternateScreen).unwrap();
    println!("{}", "\n".to_string().repeat(100));

    let start_pos = position().unwrap();
    let subtract: u16 = start_pos.1 - GAME_SIZE as u16;

    set_hook(Box::new(|_| {
        std::io::stdout().flush().unwrap();
        execute!(std::io::stdout(), LeaveAlternateScreen).unwrap();
    }));

    let mut play_game = false;

    print_game(&game, &start_pos);
    loop {
        if poll(Duration::from_millis(500))? {
            let t_size = size().unwrap();
            if GAME_SIZE >= t_size.1 as usize || GAME_SIZE > t_size.0 as usize {
                panic!("ternminal is too small.");
            }
            // Blocking read
            let event = read()?;
            if event == Event::Key(KeyCode::Up.into()) {
                let _ = execute!(stdout, MoveUp(1));
            } else if event == Event::Key(KeyCode::Right.into()) {
                let _ = execute!(stdout, MoveRight(1));
            } else if event == Event::Key(KeyCode::Left.into()) {
                let _ = execute!(stdout, MoveLeft(1));
            } else if event == Event::Key(KeyCode::Down.into()) {
                let _ = execute!(stdout, MoveDown(1));
            } else if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            } else if event == Event::Key(KeyCode::Char('t').into()) {
                game.tick();
                print_game(&game, &start_pos);
            } else if event == Event::Key(KeyCode::Char('n').into()) {
                println!("{}", game.ticks);
            } else if event == Event::Key(KeyCode::Char('c').into()) {
                game.reset();
                print_game(&game, &start_pos);
            } else if event == Event::Key(KeyCode::Enter.into()) {
                let cur_pos = position().unwrap();
                let _ = game.change((cur_pos.0 as usize, (cur_pos.1 - subtract) as usize));
                print_game(&game, &start_pos);
            } else if event == Event::Key(KeyCode::Char(' ').into()) {
                play_game ^= true;
            }
        } else if play_game {
            game.tick();
            print_game(&game, &start_pos);
        }
    }

    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

fn print_game(game: &Game, start_pos: &(u16, u16)) {
    let cur_pos = position().unwrap();
    execute!(
        stdout(),
        Hide,
        MoveTo(start_pos.0, start_pos.1),
        Clear(ClearType::All),
    )
    .expect("could not clear screen");
    for row in game.show().iter() {
        for cell in row.iter() {
            if *cell {
                execute!(stdout(), Print("*".to_string())).unwrap();
            } else {
                execute!(stdout(), Print("-".to_string())).unwrap();
            }
        }
        println!();
    }
    execute!(stdout(), MoveTo(cur_pos.0, cur_pos.1), Show).unwrap();
    stdout().flush().unwrap();
}
