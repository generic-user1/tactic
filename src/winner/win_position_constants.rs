//! constants for each WinPostion variant

use crate::gameboard::BoardSpaceLocation;

pub const TOP_ROW: [BoardSpaceLocation; 3] = [
    BoardSpaceLocation::TopLeft, 
    BoardSpaceLocation::TopMiddle, 
    BoardSpaceLocation::TopRight
];
pub const MIDDLE_ROW: [BoardSpaceLocation; 3] = [
    BoardSpaceLocation::MiddleLeft,
    BoardSpaceLocation::MiddleMiddle,
    BoardSpaceLocation::MiddleRight
];
pub const BOTTOM_ROW: [BoardSpaceLocation; 3] = [
    BoardSpaceLocation::BottomLeft,
    BoardSpaceLocation::BottomMiddle,
    BoardSpaceLocation::BottomRight
];
pub const LEFT_COLUMN: [BoardSpaceLocation; 3] = [
    BoardSpaceLocation::TopLeft,
    BoardSpaceLocation::MiddleLeft,
    BoardSpaceLocation::BottomLeft
];
pub const MIDDLE_COLUMN: [BoardSpaceLocation; 3] = [
    BoardSpaceLocation::TopMiddle,
    BoardSpaceLocation::MiddleMiddle,
    BoardSpaceLocation::BottomMiddle
];
pub const RIGHT_COLUMN: [BoardSpaceLocation; 3] = [
    BoardSpaceLocation::TopRight,
    BoardSpaceLocation::MiddleRight,
    BoardSpaceLocation::BottomRight
];
pub const TOP_LEFT_TO_BOTTOM_RIGHT: [BoardSpaceLocation; 3] = [
    BoardSpaceLocation::TopLeft,
    BoardSpaceLocation::MiddleMiddle,
    BoardSpaceLocation::BottomRight
];
pub const BOTTOM_LEFT_TO_TOP_RIGHT: [BoardSpaceLocation; 3] = [
    BoardSpaceLocation::BottomLeft,
    BoardSpaceLocation::MiddleMiddle,
    BoardSpaceLocation::TopRight
];