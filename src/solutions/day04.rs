use crate::Solve;

pub struct Problem {
    nums: Vec<i32>,
    boards: Vec<Vec<Vec<i32>>>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut retval: i64 = 0;
        let mut marks: Vec<Vec<Vec<i32>>> = day_04_create_mark_board(self.boards.len());

        for num in &self.nums {
            day_04_mark_boards(&self.boards, &mut marks, *num);
            let winning = day_04_win_horz_vert(&self.boards, &marks);
            if winning > 0 {
                retval = winning * i64::from(*num);
                break;
            }
        }
        retval
    }
    fn p2(&mut self) -> i64 {
        let mut retval: i64 = 0;
        let mut marks: Vec<Vec<Vec<i32>>> = day_04_create_mark_board(self.boards.len());
        let mut losing_boards: Vec<usize> = Vec::new();

        for i in 0..self.boards.len() {
            losing_boards.push(i);
        }

        'num_loop: for num in &self.nums {
            day_04_mark_boards(&self.boards, &mut marks, *num);
            let mut i = 0;
            while i < losing_boards.len() {
                if day_04_board_won(&marks[losing_boards[i]]) {
                    if losing_boards.len() == 1 {
                        retval = day_04_sum_unmarked(
                            &self.boards[losing_boards[i]],
                            &marks[losing_boards[i]],
                        ) * i64::from(*num);
                        break 'num_loop;
                    }
                    losing_boards.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }
        retval
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        let nums: Vec<i32> = input[0].split(',').map(|s| s.parse().unwrap()).collect();
        let boards: Vec<Vec<Vec<i32>>> = day_04_parse_boards(input);
        Problem { nums, boards }
    }
}

fn day_04_parse_boards(v: &[String]) -> Vec<Vec<Vec<i32>>> {
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut line = 1;
    for _i in 0..((v.len() - 1) / 6) {
        let b: Vec<Vec<i32>> = vec![
            v[line + 1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            v[line + 2]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            v[line + 3]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            v[line + 4]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            v[line + 5]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        ];

        boards.push(b);
        line += 6;
    }
    boards
}

fn day_04_create_mark_board(count: usize) -> Vec<Vec<Vec<i32>>> {
    let mut marks: Vec<Vec<Vec<i32>>> = Vec::new();
    for _i in 0..count {
        marks.push(vec![vec![0, 0, 0, 0, 0]; 5]);
    }
    marks
}

#[allow(dead_code)]
fn day_04_print_boards(boards: &[Vec<Vec<i32>>]) {
    for board in boards {
        for row in board {
            for col in row {
                print!("{col} ");
            }
            println!();
        }
        println!();
    }
}

fn day_04_mark_boards(boards: &[Vec<Vec<i32>>], marks: &mut Vec<Vec<Vec<i32>>>, num: i32) {
    for i in 0..boards.len() {
        for j in 0..5 {
            for k in 0..5 {
                if boards[i][j][k] == num {
                    marks[i][j][k] = 1;
                }
            }
        }
    }
}

fn day_04_win_horz_vert(boards: &[Vec<Vec<i32>>], marks: &[Vec<Vec<i32>>]) -> i64 {
    let mut retval: i64 = 0;
    'board_loop: for i in 0..boards.len() {
        for j in 0..5 {
            let mut row_sum = 0;
            let mut col_sum = 0;
            for k in 0..5 {
                row_sum += marks[i][j][k];
                col_sum += marks[i][k][j];
            }
            if row_sum == 5 || col_sum == 5 {
                retval = day_04_sum_unmarked(&boards[i], &marks[i]);
                break 'board_loop;
            }
        }
    }
    retval
}

fn day_04_board_won(m: &[Vec<i32>]) -> bool {
    let mut retval = false;
    for j in 0..5 {
        let mut row_sum = 0;
        let mut col_sum = 0;
        for k in 0..5 {
            row_sum += m[j][k];
            col_sum += m[k][j];
        }
        if row_sum == 5 || col_sum == 5 {
            retval = true;
            break;
        }
    }
    retval
}

fn day_04_sum_unmarked(b: &[Vec<i32>], m: &[Vec<i32>]) -> i64 {
    let mut retval: i32 = 0;
    for j in 0..5 {
        for k in 0..5 {
            if m[j][k] == 0 {
                retval += b[j][k];
            }
        }
    }
    i64::from(retval)
}
