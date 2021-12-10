fn main() {
    let mut input: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    let mut output = (-1, -1);

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
    for draw in sequence.split(",").map(|x| x.parse::<isize>().unwrap()) {
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

    println!("Part 1 = {}", output.0);
    println!("Part 2 = {}", output.1);
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
