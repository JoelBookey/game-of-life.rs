use gol_core::Game;
use std::io;

fn main() {
    let mut game = Game::new();
    println!("Welcome to the Game of LIFE!!!!\nType q to exit and t to see no. of ticks.\nType nothing to tick\nType two numbers to add a cell!");
    loop {
        print_game(&game);

        let mut input_s = String::new();
        io::stdin().read_line(&mut input_s).unwrap();
        if input_s.trim() == "q" {
            break;
        }
        if input_s.trim().is_empty() {
            game.tick();
            continue;
        }
        if input_s.trim() == "t" {
            println!("ticks: {}", game.ticks);
            continue;
        }
        let parsed_res = parse_input(&input_s);
        if let Ok(parsed) = parsed_res {
            match game.change(parsed) {
                Err(_) => println!("invalid indices"),
                Ok(()) => {}
            }
        } else {
            if parsed_res == Err(InputErr::NotInt) {
                println!("that isn't a number");
            }
            if parsed_res == Err(InputErr::IndexErr) {
                println!("TWO numbers");
            }
            if parsed_res == Err(InputErr::IsZero) {
                println!("not zero please");
            }
        }
    }
}

fn print_game(game: &Game) {
    let grid = game.show();
    for (_, line) in grid.iter().enumerate() {
        //print!("{}|", i);
        for c in line.iter() {
            if *c {
                print!("*");
            } else {
                print!("-");
            }
        }
        println!();
    }
    //println!("------------------------------");
}

fn parse_input(s: &String) -> Result<(usize, usize), InputErr> {
    let sp_s = s.split_whitespace().collect::<Vec<&str>>();
    let mut res = (0, 0);
    if let Some(first) = sp_s.get(0) {
        if let Ok(val) = first.parse::<usize>() {
            if val == 0 {
                return Err(InputErr::IsZero);
            }
            res.0 = val - 1;
        } else {
            return Err(InputErr::NotInt);
        }
    } else {
        return Err(InputErr::IndexErr);
    }
    if let Some(second) = sp_s.get(1) {
        if let Ok(val) = second.parse::<usize>() {
            if val == 0 {
                return Err(InputErr::IsZero);
            }
            res.1 = val - 1;
        } else {
            return Err(InputErr::NotInt);
        }
    } else {
        return Err(InputErr::IndexErr);
    }

    Ok(res)
}

#[derive(PartialEq)]
enum InputErr {
    NotInt,
    IndexErr,
    IsZero,
}
