// mod moves;
mod board;
mod strategy;
mod player;

use std::io::{stdin, stdout, Write};
use board::Board;
// use crate::moves::{ai_turn, human_turn};
use crate::player::{Player, Moove};

fn main(){
    let mut board = Board::new();
    
    board.clean();

    let mut h_choice = String::from("");
    let mut c_choice = String::from("");
    let mut first = String::from("");

    while h_choice != "O" && h_choice != "X" {
        h_choice = "".to_string();
        println!("Выберите сторону: X или O");
        let _ = stdout().flush();
        stdin().read_line(&mut h_choice).expect("Не верный ввод");
        if let Some('\n')=h_choice.chars().next_back() {
            h_choice.pop();
        }
        if let Some('\r')=h_choice.chars().next_back() {
            h_choice.pop();
        }
    }
    c_choice = if h_choice == "X" { String::from("O") } else { String::from("X") };
    board.clean();

    while first != "Y" && first != "N" {
        first = "".to_string();
        println!("Начинаете первым?(Y или N)");
        let _ = stdout().flush();
        stdin().read_line(&mut first).expect("Не верный ввод");
        if let Some('\n')=first.chars().next_back() {
            first.pop();
        }
        if let Some('\r')=first.chars().next_back() {
            first.pop();
        }
    }
    let mut player = match first.as_str() {
        "Y" => Player::Human,
        "N" => Player::Computer,
        _ => panic!("Error")
    };
    while board.empty_cells().len() > 0 && !board.game_over() {
        player.turn(&c_choice, &h_choice, &mut board);
        player = match player {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
            Player::None => Player::None
        };
        // human_turn(&c_choice, &h_choice, &mut board);
        // ai_turn(&c_choice, &h_choice, &mut board);
    }
    match board.wins(){
        Player::Human => {
            board.clean();
            println!("Ход человека {}", h_choice);
            board.render(&c_choice, &h_choice);
            println!("Вы победили");
        },
        Player::Computer => {
            board.clean();
            println!("Ход компьютера {}", c_choice);
            board.render(&c_choice, &h_choice);
            println!("Компьютер победил");
        },
        _ => {
            board.clean();
            board.render(&c_choice, &h_choice);
            println!("Ничья!");
        }
    };
}