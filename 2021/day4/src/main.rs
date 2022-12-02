use utils::read_puzzle_input;

fn check_victory(board: &[Option<usize>]) -> Option<usize> {
    for row in board.chunks(5) {
        if row.iter().all(|cell| cell.is_none()) {
            return Some(score_board(board))
        }
    }

    for column in 0..5 {
        if board.chunks(5).all(|row| row[column].is_none()) {
            return Some(score_board(board))
        }
    }

    None
}

fn score_board(board: &[Option<usize>]) -> usize {
    board.iter().map(|&cell| cell.unwrap_or(0)).sum()
}

fn main() {
//     let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

// 22 13 17 11  0
// 8  2 23  4 24
// 21  9 14 16  7
// 6 10  3 18  5
// 1 12 20 15 19

// 3 15  0  2 22
// 9 18 13 17  5
// 19  8  7 25 23
// 20 11 10 24  4
// 14 21 16 12  6

// 14 21 17 24  4
// 10 16 15  9 19
// 18  8 23 26 20
// 22 11 13  6  5
// 2  0 12  3  7";

    let input = read_puzzle_input!().unwrap();

    let mut input = input.lines()
        .filter(|line| !line.trim().is_empty());
    
    let draws: Vec<_> = input.next().unwrap().split(',')
        .map(|d| d.trim().parse::<usize>().unwrap())
        .collect();
    
    let boards: Vec<_> = input.collect();
    
    let mut boards: Vec<_> = boards.chunks(5).map(|board| {
        board.iter()
            .flat_map(|row| row.split_whitespace().map(|cell| cell.parse::<usize>().ok()))
            .collect::<Vec<_>>()
        })
        .collect();
    
    for draw in draws {
        {
            let draw = Some(draw);
            boards.iter_mut().for_each(|board| {
                board.iter_mut().for_each(|cell| if cell == &draw {
                    *cell = None;
                });
            });
        }

        if boards.len() > 1 {
            boards.retain(|board| check_victory(board).is_none());
        } else if let Some(score) = check_victory(&boards[0]) {
            println!("Last winning board: {score}");
            println!("Final score: {}", score * draw);
            break;
        }
    }
}
