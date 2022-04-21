use crate::board::Board;
use crate::Player;

pub fn eval(board: &Board) -> Player {
    board.wins()
}

pub fn estimate_value(board: &Board, player: &Player, depth: i32) -> i32 {
    let score;
    if board.wins() == *player{
        score = 1;
    } else {
        score = -1;
    }

    score * (depth + 1)
}

pub fn minimax(board: &mut Board, depth: i32, player: &Player) -> Vec<i32>{
    let mut best;
    if *player == Player::Computer {
        best = vec![-1, -1, i32::MIN];
    } else {
        best = vec![-1,-1, i32::MAX];
    }
    if depth == 0 || board.game_over(){
        let mut score = match eval(&board) {
            Player::Computer => 1,
            Player::Human => -1,
            Player::None => 0,
        };
        return vec![-1, -1, score];
    }
    for cell in board.empty_cells(){
        let x = cell.0;
        let y = cell.1;
        board.squares[x][y] = player.clone();
        let mut score = minimax(board, depth - 1, player);
        board.squares[x][y] = Player::None;
        score[0] = x as i32;
        score[1] = y as i32;
        if *player == Player::Computer {
            if score[2] > best[2]{
                best = score
            }
        } else {
            if score[2] < best[2]{
                best = score;
            }
        }
    }
    best
}

pub fn negamax(board: &mut Board, depth: i32, player: &Player) -> Vec<i32>{
    let mut best = vec![-1, -1, i32::MIN];

    if depth == 0 || board.game_over(){
        let mut score = estimate_value(board, player, depth);
        return vec![-1, -1, -score];
    }
    for cell in board.empty_cells() {
        let x = cell.0;
        let y = cell.1;
        board.squares[x][y] = player.clone();
        let mut score = negamax(board, depth - 1, &player);
        board.squares[x][y] = Player::None;
        score[0] = x as i32;
        score[1] = y as i32;
        if score[2] > best[2] {
            best = score;
        }
    }
    best[2] *= -1;
    best
}