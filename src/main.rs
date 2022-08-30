use tactic::gameboard::{GameBoard, BoardSpace, BoardSpaceLocation};
use tactic::game_outcome::GameOutcome;
fn main() {
    let mut board = GameBoard::new();
    
    println!("Default: {}", board);
    println!("Winner: {:?}\n", GameOutcome::analyze_game(&board));

    for location in BoardSpaceLocation::all(){
        *board.space_mut(location) = BoardSpace::X;
    }
    println!("All Xs: {}", board);
    println!("Winner: {:?}\n", GameOutcome::analyze_game(&board));

    *board.space_mut(BoardSpaceLocation::TopLeft) = BoardSpace::O;
    *board.space_mut(BoardSpaceLocation::MiddleMiddle) = BoardSpace::O;
    *board.space_mut(BoardSpaceLocation::BottomRight) = BoardSpace::O;
    println!("TopLeft to BottomRight O: {}", board);
    println!("Winner: {:?}", GameOutcome::analyze_game(&board));

    *board.space_mut(BoardSpaceLocation::TopLeft) = BoardSpace::O;
    *board.space_mut(BoardSpaceLocation::TopMiddle) = BoardSpace::X;
    *board.space_mut(BoardSpaceLocation::TopRight) = BoardSpace::O;
    *board.space_mut(BoardSpaceLocation::MiddleLeft) = BoardSpace::X;
    *board.space_mut(BoardSpaceLocation::MiddleMiddle) = BoardSpace::O;
    *board.space_mut(BoardSpaceLocation::MiddleRight) = BoardSpace::X;
    *board.space_mut(BoardSpaceLocation::BottomLeft) = BoardSpace::X;
    *board.space_mut(BoardSpaceLocation::BottomMiddle) = BoardSpace::O;
    *board.space_mut(BoardSpaceLocation::BottomRight) = BoardSpace::X;
    println!("Draw example: {}", board);
    println!("Winner: {:?}", GameOutcome::analyze_game(&board));
    
}
