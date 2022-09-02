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
//!
//! Menu should be a list of interactable options, where some keys (probably left and right arrows)
//! allow the user to change the setting, and up/down allow the user to select different settings to change
//! 
//! If we have lots of options, there will need to be scrolling support. Selecting an option that is too high
//! or too low to be viewable should scroll the entire menu. This could be handled with some kind of dynamic offset.
//! 
//! Option types would include:
//!
//! - "radio buttons" (select one option from a pre-defined set)
//!
//!   - toggles could be implemented as a 2-state "radio button" 
//! 
//!   - options could be implemented as enums with some method that gives a name to each option
//! 
//!   - may want 'descriptions' for some options, where there is some pre-defined space to print 
//!     a description of the currently selected option.
//! 
//! - numeric inputs (for both floats and ints)
//!   
//!   - bounds need to be configurable
//! 
//!   - float input could take an int from a known range in and scale it to some float output (0-100 scales to 0.0-1.0)
//!  
//! 
//! Options need to be capable of being 'hidden' or 'deactivated'
//! 
//! - either no display or a differently styled display to indicate the option is deactivated
//! 
//! - a deactivated option may or may not still be selectable, but should be unchangeable
//! 
//! 
//! Selected options need to be highlighted somehow, probably by filling the background of their text.