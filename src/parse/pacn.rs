// Pure algebraic coordinate notation

use {
    crate::{
        board::{
            line::*,
            square::*,
            piece::*,
        },
        game::{
            board_move::*,
            position::*
        },
    },
    super::chr::*,
};


#[derive(Debug)]
pub enum PacnError {
    // Pure Algebraic Coordinate Notation
    MalformedPacn,
    ConversionError(ConversionError),
    IllegalMove(IllegalMove),
}


fn converts<T>(result: Result<T, ConversionError>) -> Result<T, PacnError> {
    match result {
        Err(e) => Err(PacnError::ConversionError(e)),
        Ok(r) => Ok(r),
    }
}

fn expect<T>(option: Option<T>) -> Result<T, PacnError> {
    match option {
        None => Err(PacnError::MalformedPacn),
        Some(val) => Ok(val),
    }
}

impl Move {

    pub fn from_str(game: &GameState, s: &str) -> Result<Move, PacnError> {
        let mut chars = s.chars();
        let origin = Square::from_file_rank(
            converts(File::from_chr(expect(chars.next())?))?,
            converts(Rank::from_chr(expect(chars.next())?))?);
        let destination = Square::from_file_rank(
            converts(File::from_chr(expect(chars.next())?))?,
            converts(Rank::from_chr(expect(chars.next())?))?);
        let promotion: Option<Promotion> = match chars.next() {
            None => None,
            Some(c) => match chars.next() {
                Some(_) => return Err(PacnError::MalformedPacn),
                None => Some(converts(Promotion::from_chr(c))?),
            }
        };

        Ok(Move::new(game, origin, destination, promotion))
    }
}
