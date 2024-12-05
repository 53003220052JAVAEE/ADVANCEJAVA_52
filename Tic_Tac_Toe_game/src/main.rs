use std::io;


fn main() {

    let mut board = vec![vec![' ' ; 3];3];
    let mut current_player = 'X';


    println!("welcome to tic tac toe!!!!");

    loop
    {
        print_board(&board);

        println!("player {} turn",current_player);

        let (row,col) = get_player_move();
        if board[row][col] != ' ' {
            println!("the space is occupied");
            continue;
        }

        board [row] [col] = current_player;

        if is_winner(&board, current_player) {
            print_board(&board);
            println!("player {} wins",current_player);
            break;
        }
        else if is_draw(&board) {
            print_board(&board);
            println!("Its a Draw");
            break;
        }
        current_player = if current_player == 'X' {'O'} else {'X'};
    }
}

fn print_board(board: &[Vec<char>]) {
    println!("current board");
    for row in board {
        println!("{}",row.iter().map(|&c| if c == ' ' {'.'} else {c}).collect::<String>());
    }
    println!();
 }
    fn get_player_move() -> (usize,usize) {
        loop {
            let mut input = String::new();
            println!("Enter your move as row,col");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            if let Some((row,col)) = parse_move(input) {
                if row < 3 && col < 3 {
                    return (row,col);
                }
            }
            println!("Invalid input");
        }
    }

    fn parse_move(input: &str) -> Option<(usize, usize)> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() !=2 {
            return None;
        }   
        let row = parts[0].trim().parse::<usize>().ok()?;
        let col = parts[1].trim().parse::<usize>().ok()?;

        Some((row-1,col-1))

     }

     fn is_winner(board: &[Vec<char>], player:char) -> bool {
        for i in 0..3 {
            if(board[i][0] == player && board[i][1] == player && board[i][2] == player)  ||
            (board[0][i] == player && board[1][i] == player && board[2][i] == player)
        {
            return true;
        }
        }
        (board[0][0] == player && board[1][1] == player && board[2][2] == player) ||
        (board[0][2] == player && board[1][1] == player && board[2][0] == player)
     }

     fn is_draw(board: &[Vec<char>]) -> bool {
        board.iter().all(|row| row.iter().all(|&cell| cell!= ' '))
     }