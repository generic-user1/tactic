//! Utilities for AI player

use crate::gameboard::{GameBoard, BoardSpaceLocation, BoardSpace};
use crate::active_player::ActivePlayer;

/// Plays a turn on the specified game board
/// 
/// Which turn to play (player X or player O) is determined by `player`
/// 
/// `board` is the [GameBoard] to play on. The board is modified in-place.
/// 
/// If a turn is played correctly, the function will return `true`.
/// If a turn cannot be played (because, for example, the game is finished),
/// the function will return `false`.
/// 
/// TODO: actually implement AI
/// this is currently just a stub that plays the first available move   
pub fn do_turn(board: &mut GameBoard, player: &ActivePlayer) -> bool
{
    // return early if game is already finished
    if board.game_outcome().game_finished(){
        return false;
    }

    //search for empty space
    for location in BoardSpaceLocation::all(){
        let space = board.space_mut(location);
        if space == &BoardSpace::Empty {
            *space = match player {
                ActivePlayer::PlayerX => BoardSpace::X,
                ActivePlayer::PlayerO => BoardSpace::O
            };
            return true;
        }
    }

    panic!("Empty space not found despite game not being finished! Programmer Error!")
}