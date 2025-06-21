use crate::board_move::Castling;
use crate::piece::*;
use crate::square::Square;


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
    InvalidEnPassant,
    WrongEnPassantRank,
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
pub enum CorruptedBitboard {
    OccupancyMismatch(u64),
    ZobristMismatch(u64),
    InvalidEnPassantCode(u32),
}

#[derive(Debug)]
pub enum IllegalPosition {
    OpponentInCheck,
    TooManyPieces(ColoredPiece, u32),
    MissingKing(Color),
    SameColorBishops(ColoredPiece, Color),  // No pawn promotions
    InvalidEPTarget,
    InvalidPawnRank,
    CastlingIncorrect(Castling),
    EnPassantSquareOccupied,
    NoEnPassantAttacker,
    NoEnPassantDefender,
    CorruptedBitboard(CorruptedBitboard),
}
