use tic_tac_toe::*;
fn main() {
    let mut new_board = Board::default();

    loop {
        new_board.print_board();

        get_move(&mut new_board);

        // Check game conditions
        if check_draw(&new_board) && check_win(&new_board) {
            new_board.print_board();
            println!("Draw");
            break;
        } else if check_win(&new_board) {
            new_board.print_board();
            println!("Victory! Player {} won!", &new_board.get_current_player());
            break;
        }

        // Change current player for next round
        new_board.change_player();
    }
}
