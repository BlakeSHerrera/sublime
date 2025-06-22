use crate::board_move::Castling;
use crate::color::Color;
use crate::piece::Piece;
use crate::position::FenSection;


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
    InvalidEnPassantRank,
    InvalidHalfmove,
    InvalidFullMove,
    IllegalPosition(IllegalPosition),
    MissingSection(FenSection),
    TooManySections,
    ExpectedSpace(i32),
    CastlingOutOfOrder,
}


#[derive(Debug)]
pub enum IllegalMove {
    InCheck,
    InvalidMove,
    OpponentPieceMove,
    EmptySquareMove,
    AlliedCapture,
    CastleOutOfCheck,
    CastleThroughCheck,
    InvalidPromotion,
}


#[derive(Debug)]
pub enum PacnError {
    // Pure Algebraic Coordinate Notation
    MalformedPacn,
    CoordinateError(CoordinateError),
    IllegalMove(IllegalMove),
}

#[derive(Debug)]
pub enum CorruptedBitboard {
    // Corrupted bitboards should never arise from user error;
    // possible user error is handled by IllegalPosition.
    OccupancyMismatch(u64),
    ZobristMismatch(u64, u64),  // Expected, actual
    InvalidEnPassantCode(u32),
}

#[derive(Debug)]
pub enum IllegalPosition {
    OpponentInCheck,
    TooManyPieces(Piece, u32),
    MissingKing(Color),
    SameColorBishops(Piece, Color),  // No pawn promotions
    InvalidEPTarget,
    InvalidPawnRank,
    InvalidCastling(Castling),
    EnPassantSquareOccupied,
    NoEnPassantAttacker,
    NoEnPassantDefender,
    CorruptedBitboard(CorruptedBitboard),
}
