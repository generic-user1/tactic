//! Utilities for AI player

use crate::game_outcome::GameOutcome;
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
pub fn do_turn(board: &mut GameBoard, player: &ActivePlayer) -> bool
{
    // return early if game is already finished
    if board.game_outcome().game_finished(){
        return false;
    }

    let mut possible_moves: Vec<PossibleMove> = Vec::new();
    for location in BoardSpaceLocation::all(){
        if board.space(location) == &BoardSpace::Empty {
            possible_moves.push(PossibleMove::new(
                board, 
                location, 
                player, 
                player
            ));
        }
    }

    let mut top_win_score = 0.0;
    let mut top_win_move: Option<&PossibleMove> = None;
    for possible_move in possible_moves.iter(){
        if top_win_move.is_none(){
            top_win_score = possible_move.win_score();
            top_win_move = Some(possible_move);
        } else {
            let new_win_score = possible_move.win_score();
            if new_win_score > top_win_score{
                top_win_score = new_win_score;
                top_win_move = Some(possible_move);
            }
        }
    }

    match top_win_move {
        Some(next_move) => {
            let new_location = *next_move.new_location();
            *board.space_mut(new_location) = player.get_board_space();
        },
        None => panic!("Couldn't find a valid move!")
    }
    true
}

#[derive(Clone)]
struct PossibleMove {
    new_location: BoardSpaceLocation,
    win_score: f64
}

impl PossibleMove{

    /// Creates and returns a new `PossibleMove`
    /// 
    ///# Notes
    /// 
    /// This constructor evaluates all sub moves from the newly created move.
    /// The process of evaluating all sub moves may take significant time; when appropriate
    /// it is usually best to reference or clone an existing `PossibleMove` instance
    pub fn new(
        board: &GameBoard, 
        new_location: BoardSpaceLocation, 
        active_player: &ActivePlayer,
        winning_player: &ActivePlayer
    ) -> Self
    {
        let mut new_board = board.clone();
        *new_board.space_mut(new_location) = 
            active_player.get_board_space();
        
        let sub_moves = if !new_board.game_outcome().game_finished() {
            let sub_active_player = active_player.opposite();
            let mut sub_moves = Vec::new();
            for sub_location in BoardSpaceLocation::all(){
                if new_board.space(sub_location) == &BoardSpace::Empty{
                    sub_moves.push(Self::new(
                        &new_board, 
                    sub_location, 
                        &sub_active_player,
                        winning_player
                    ))
                }
            }
            sub_moves
        } else {
            Vec::new()
        };

        let win_score = Self::calculate_win_score(
            &sub_moves, 
            &new_board, 
            winning_player
        );
        Self {
            new_location,
            win_score
        }
    }

    /// Gets the win score for this PossibleMove
    /// 
    /// Win score is an abstract value representing how likely this possible move
    /// is to result in a win for desired player or a draw
    /// 
    /// The exact value isn't especially meaningful, it is most useful for 
    /// comparison against other win scores from other possible moves
    pub fn win_score(&self) -> f64
    {
        self.win_score
    }

    /// Gets the preference for this `PossibleMove` when seeking a win 
    /// 
    /// Represented as a float from -1 to 1, where -1 is least preferable, 1 is most preferable
    fn calculate_win_score(
        sub_moves: &Vec<PossibleMove>,
        board: &GameBoard,
        winning_player: &ActivePlayer
    ) -> f64
    {
        let sub_move_count = sub_moves.len();
        if sub_move_count > 0 {
            //if there are sub moves, return half the average of their scores
            //halving is used to de-emphaize distant moves
            let total_wins: f64 = sub_moves.iter().map(|sub_move|{
                sub_move.win_score}
            ).sum();
            
            //de-emphasize distant moves by halving
            (total_wins/(sub_move_count as f64)) * 0.5
        } else {
            match board.game_outcome() {
                GameOutcome::PlayerX(_) => {
                    match winning_player{
                        ActivePlayer::PlayerX => 1.0,
                        ActivePlayer::PlayerO => -1.0
                    }
                },
                GameOutcome::PlayerO(_) => {
                    match winning_player{
                        ActivePlayer::PlayerX => -1.0,
                        ActivePlayer::PlayerO => 1.0
                    }
                },
                //Incomplete and Draw always get a score of 0
                _ => 0.0
            }
        }
    }

    /// Returns the [BoardSpaceLocation] associated with this possible move 
    pub fn new_location(&self) -> &BoardSpaceLocation
    {
        &self.new_location
    }
}