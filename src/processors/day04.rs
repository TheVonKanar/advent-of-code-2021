fn parse_input() -> Vec<&'static str> {
    include_str!("../../data/day04.txt")
        .split("\r\n\r\n")
        .collect()
}

pub fn process() -> (usize, usize) {
    let mut input = parse_input();
    let mut output: (isize, isize) = (-1, -1);

    let sequence = input.swap_remove(0);

    let mut boards = Vec::new();
    for elem in &input {
        let mut board = Vec::new();
        for line in elem.lines() {
            board.push(
                line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<isize>>(),
            );
        }

        boards.push(board);
    }

    let mut winning_boards = Vec::new();
    for draw in sequence.split(",").map(|x| x.parse().unwrap()) {
        for board in 0..boards.len() {
            for row in 0..boards[board].len() {
                for col in 0..boards[board][row].len() {
                    if boards[board][row][col] == draw {
                        boards[board][row][col] = -1;
                    }

                    if boards[board].iter().all(|x| x[col] < 0) {
                        if !winning_boards.contains(&board) {
                            winning_boards.push(board);
                            output.1 = draw * compute_board_sum(&boards[board]);
                        }

                        if output.0 < 0 {
                            output.0 = draw * compute_board_sum(&boards[board]);
                        }
                    }
                }

                if boards[board][row].iter().all(|&x| x < 0) {
                    if !winning_boards.contains(&board) {
                        winning_boards.push(board);
                        output.1 = draw * compute_board_sum(&boards[board]);
                    }

                    if output.0 < 0 {
                        output.0 = draw * compute_board_sum(&boards[board]);
                    }
                }
            }
        }
    }

    (output.0 as usize, output.1 as usize)
}

fn compute_board_sum(board: &Vec<Vec<isize>>) -> isize {
    let mut sum = 0;
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] > 0 {
                sum += board[row][col];
            }
        }
    }

    sum
}
