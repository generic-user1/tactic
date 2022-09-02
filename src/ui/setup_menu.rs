//! UI implementations for the pre-game menu
//!
//! todo: add pre-game menu with various options including:
//! 
//!- player types for x and o
//! 
//!  - possibly as single/multiplayer
//! 
//!  - possibly as human/ai for both players seperately
//! 
//!- game number limit (auto-exit when limit reached)
//! 
//!  - unlimited (must disallow if/when using 2 ai players)
//! 
//!  - to n (custom value) games
//! 
//!  - to n (custom value) non-draw games
//! 
//!  - to n (custom value) wins for a single player
//! 
//!  - to score percent? unsure of this one
//! 
//!- ai player difficulty 
//! 
//!- reverse mode
//! 
//!  - getting three of your own pieces in a row counts as a loss
//! 
//!  - try to force the opposite player to get three in a row
//! 
//!  - AI difficulty is reversed in this mode (if player inputs a 1, a 0 
//!    should be passed to the AiPlayer instance) 