use crate::board::{
    color::Color,
    piece::Piece,
    zone::Quadrant,
};

use super::{
    chr::ConversionError,
    fen::FenSection
};


#[derive(Debug)]
pub enum FenError {
    ConversionError(ConversionError),
    TooManyRows,
    TooManyColumns(i32),
    IncompleteRow(i32),
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
    ConversionError(ConversionError),
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
    InvalidCastling(Quadrant),
    EnPassantSquareOccupied,
    NoEnPassantAttacker,
    NoEnPassantDefender,
    CorruptedBitboard(CorruptedBitboard),
}
