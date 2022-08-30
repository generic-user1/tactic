use tactic::gameboard::{GameBoard, BoardSpace, BoardSpaceLocation};
use tactic::winner;
fn main() {
    let mut board = GameBoard::new();
    
    println!("Default: {}", board);
    println!("Winner: {:?}\n", winner::get_winner(&board));

    for location in BoardSpaceLocation::all(){
        *board.space_mut(location) = BoardSpace::X;
    }
    println!("All Xs: {}", board);
    println!("Winner: {:?}\n", winner::get_winner(&board));

    *board.space_mut(BoardSpaceLocation::TopLeft) = BoardSpace::O;
    *board.space_mut(BoardSpaceLocation::MiddleMiddle) = BoardSpace::O;
    *board.space_mut(BoardSpaceLocation::BottomRight) = BoardSpace::O;
    println!("TopLeft to BottomRight O: {}", board);
    println!("Winner: {:?}", winner::get_winner(&board));
}
