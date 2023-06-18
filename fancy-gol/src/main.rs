use gol_core::Game;
use std::io::{stdout, Write};
use std::panic::set_hook;

use crossterm::{
    cursor::{position, Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, Show},
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    let mut game = Game::new();
    execute!(stdout, EnterAlternateScreen).unwrap();
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    let start_pos = position().unwrap();
    let subtract = start_pos.1 - 30;

    set_hook(Box::new(|_| {
        std::io::stdout().flush().unwrap();
        execute!(std::io::stdout(), LeaveAlternateScreen).unwrap();
    }));

    print_game(&game, &start_pos);
    loop {
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
            let cur_pos = position().unwrap();
            println!("{:?}", cur_pos);
        } else if event == Event::Key(KeyCode::Enter.into()) {
            let cur_pos = position().unwrap();
            game.change((cur_pos.0 as usize, (cur_pos.1 - subtract) as usize))
                .expect("invalid pos");
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
