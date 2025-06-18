use crate::piece::*;


#[derive(Debug)]
pub enum FenSection {
    Board,
    SideToMove,
    Castling,
    EnPassant,
    HalfmoveCounter,
    FullmoveCounter,
}


#[derive(Debug)]
pub enum CoordinateError {
    InvalidRank(char),
    InvalidFile(char),
    IncompleteSquare,
}


#[derive(Debug)]
pub enum FenError {
    TooManyRows,
    TooManyColumns(i32),
    IncompleteRow(i32),
    CoordinateError(CoordinateError),
    InvalidPiece(char),
    InvalidColor(char),
    InvalidCastlingChar(char),
    InvalidCastling,
    InvalidEP,
    WrongEPRank,
    HalfmoveNotANumber,
    FullmoveNotANumber,
    IllegalPosition(IllegalPosition),
    MissingSection(FenSection),
    TooManySections,
    ExpectedSpace(i32),
    CastlingOutOfOrder,
}


#[derive(Debug)]
pub enum IllegalMove {
    InCheck,
    WrongMovement,
    OpponentPieceMove,
    EmptySquareMove,
    AlliedCapture,
}


#[derive(Debug)]
pub enum PacnError {
    // Pure Algebraic Coordinate Notation
    CoordinateError(CoordinateError),
    IllegalMove(IllegalMove),
}


#[derive(Debug)]
pub enum IllegalPosition {
    OpponentInCheck,
    TooManyPieces(Piece, i32),
    MissingKing(Color),
    SameColorBishops(Color),  // No pawn promotions
    InvalidEPTarget,
    InvalidPawnRank,
}
