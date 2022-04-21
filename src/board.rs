use std::collections::HashMap;
use std::time::SystemTime;
use crate::Player;

pub struct Board {
    pub squares: Vec<Vec<Player>>
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: vec![
                vec![Player::None, Player::None, Player::None],
                vec![Player::None, Player::None, Player::None],
                vec![Player::None, Player::None, Player::None],
            ]
        }
    }
    pub fn clean(&self){
        match std::env::consts::OS {
            "windows" => {
                std::process::Command::new("cmd")
                    .args(["/C", "cls"])
                    .output()
                    .unwrap();
            },
            _ => {
                std::process::Command::new("sh")
                    .args(["-c", "clear"])
                    .output()
                    .unwrap();
            }
        };
    }
    pub fn render(&self, c_choice: &String, h_choice: &String){
        let mut chars = HashMap::new();
        chars.insert(Player::Human, h_choice.to_owned());
        chars.insert(Player::Computer, c_choice.to_owned());
        chars.insert(Player::None, " ".to_string());

        let str_line = "---------------";

        println!("\n{}", str_line);
        for row in &self.squares {
            for cell in row{
                let symbol = chars.get(&cell.clone()).unwrap();
                print!("| {} |", symbol);
            }
            println!("\n{}", str_line);
        }
    }

    pub fn empty_cells(&self)->Vec<(usize, usize)>{
        let mut cells: Vec<(usize, usize)> = Vec::new();

        for (x, row) in self.squares.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell == Player::None {
                    cells.push((x, y));
                }
            }
        }
        cells
    }

    pub fn wins(&self)->Player{
        let squares = self.squares.clone();
        let win_state = vec![
            vec![squares[0][0], squares[0][1], squares[0][2]],
            vec![squares[1][0], squares[1][1], squares[1][2]],
            vec![squares[2][0], squares[2][1], squares[2][2]],
            vec![squares[0][0], squares[1][0], squares[2][0]],
            vec![squares[0][1], squares[1][1], squares[2][1]],
            vec![squares[0][2], squares[1][2], squares[2][2]],
            vec![squares[0][0], squares[1][1], squares[2][2]],
            vec![squares[2][0], squares[1][1], squares[0][2]]
        ];
        match win_state/*.iter().map(|s| &**s).collect::<Vec<Vec<&String>>>()*/
                            .into_iter()
                            .find(|x| {
                                *x == vec![Player::Human, Player::Human, Player::Human]
                                || *x == vec![Player::Computer, Player::Computer, Player::Computer]
                            }){
                                Some(i) => i[0].clone(),
                                None => Player::None
                            }
    }

    pub fn game_over(&self) -> bool {
        match self.wins() {
            Player::None => false,
            _ => true
        }
    }

    pub fn valid_move(&self, x: usize, y: usize) -> bool{
        match self.empty_cells().into_iter().find(|&i| i == (x,y)){
            Some(i) => true,
            None => false
        }
    }

    pub fn set_move(&mut self, x: usize, y: usize, player: Player) -> bool{
        if self.valid_move(x, y) {
            self.squares[x][y] = player;
            true
        } else {
            false
        }
    }
}