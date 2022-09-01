//! Utilities for AI player

use crate::{
    game_outcome::GameOutcome,
    gameboard::{GameBoard, BoardSpaceLocation, BoardSpace},
    active_player::ActivePlayer
};
use rand::Rng;
/// Represents an AI player
#[derive(Debug, PartialEq)]
pub struct AiPlayer{
    difficulty: f64
}

impl AiPlayer{
    
    /// Construct and return a new `AiPlayer` at the specified difficulty
    /// 
    /// `difficulty` is a value within the range `[0.0, 1.0]` that represents
    /// the difficulty of the AI player. `1.0` is the maximum difficulty.
    /// 
    ///# Panics
    /// 
    /// This method panics if difficulty is less than or equal to 0,
    /// or if difficulty is greater than 1.
    pub fn new(difficulty:f64) -> Self
    {
        let mut new_instance = Self::default();
        new_instance.set_difficulty(difficulty);
        new_instance
    }

    /// Set the difficulty of this `AiPlayer`
    /// 
    /// `difficulty` is a value within the range `[0.0, 1.0]` that represents
    /// the difficulty of the AI player. `1.0` is the maximum difficulty.
    /// 
    ///# Panics
    /// 
    /// This method panics if difficulty is less than or equal to 0,
    /// or if difficulty is greater than 1.
    pub fn set_difficulty(&mut self, difficulty:f64)
    {
        if difficulty < 0.0 || difficulty > 1.0 || difficulty.is_nan(){
            panic!("Provided difficulty of {} is outside the difficulty range of (0.0,1.0]", 
                difficulty);
        }

        self.difficulty = difficulty;
    }

    /// Returns the difficulty of this `AiPlayer`
    /// 
    /// The difficulty will always be within the range `(0.0, 1.0]`
    pub fn difficulty(&self) -> f64
    {
        self.difficulty
    }

    /// Returns the mistake chance of this `AiPlayer`
    /// 
    /// The mistake chance is the chance (from 0 to 1) that on any given turn,
    /// this `AiPlayer` will make a `mistake` and select a non-optimal move.
    /// How non-optimal this move is depends on the difficulty (lower difficulty means less optimal).
    /// 
    /// The mistake chance is a function of the difficulty; more specifically `mistake_chance = 1 - difficulty`.
    /// This means that a higher difficulty results in a lower mistake chance (and vice versa). A difficulty of `1.0`
    /// results in a mistake chance of `0.0`.
    /// 
    /// The mistake chance will always be within the range `[0.0, 1.0]`.
    pub fn mistake_chance(&self) -> f64
    {
        // return the mistake chance with bounds checking to ensure value is within valid range
        (1.0 - self.difficulty).min(1.0).max(0.0)
    }

    /// Plays a turn on the specified game board
    /// 
    /// Which turn to play (player X or player O) is determined by `player`
    /// 
    /// `board` is the [GameBoard] to play on. 
    /// 
    /// If a move can be played successfully, this method will return `Ok(new_board)`
    /// where `new_board` is the given [GameBoard] after the AI has played its turn.
    /// 
    /// If a move cannot be played (for example, because the game is finished), this method
    /// will return `Err(AiError)` with an appropriate [AiError] describing the issue.
    pub fn do_turn(&self, board: &GameBoard, player: &ActivePlayer) -> Result<GameBoard, AiError>
    {

        // return early if game is already finished
        if board.game_outcome().game_finished(){
            return Err(AiError::GameFinished);
        }

        // generate possible moves
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

        // return if there are no possible moves found
        if possible_moves.is_empty() {
            return Err(AiError::NoMovesFound);
        }

        // sort possible moves by win score (lowest to highest)
        possible_moves.sort_by(|move_a, move_b|{
            match move_a.win_score().partial_cmp(&move_b.win_score()){
                Some(ordering) => ordering,
                None => std::cmp::Ordering::Equal // assume equality if no ordering exists
            }
        });
        
        //cache the rng as it will be used more than once
        let mut rng = rand::thread_rng();

        // generate a number from 0 to (not including) 1
        // if the mistake chance is greater than this value, do mistake; otherwise play optimally
        // 1.0 mistake chance is always greater than generated value
        // 0.0 mistake chance is always less than or equal to (thus not greater than) generated value
        let do_mistake = self.mistake_chance() > rng.gen_range(0.0..1.0);
        
        // determine next move 
        let next_move = if do_mistake {
            // pick non-optimal move by scaling difficulty to length of possible_moves
            // rounding down means we never pick the last move unless it's the only move
            let move_index = (self.difficulty * (possible_moves.len() as f64)) as usize;
            match possible_moves.get(move_index){
                Some(pmove) => pmove,
                None => {
                    //get first move in this case, which must exist because we already returned if possible moves was empty
                    possible_moves.first().unwrap()
                }
            }
        } else {
            // play optimally if do_mistake is false
            possible_moves.last().unwrap()
        };

        // Clone the input board; this gets a new, mutable board to play move on
        let mut new_board = board.clone();

        // play next move and return modified board
        let new_location = *next_move.new_location();
        *new_board.space_mut(new_location) = player.get_board_space();
        Ok(new_board)
    }
}

impl Default for AiPlayer{
    fn default() -> Self {
        Self{difficulty:1.0}
    }
}

/// Reasons why a turn may fail
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AiError{
    /// The provided [GameBoard] was already finished, and no more moves are possible
    GameFinished,
    /// The provided [GameBoard] was not finished, but no valid moves could be found
    NoMovesFound
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