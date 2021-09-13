use std::ops::{Index, IndexMut};

use crate::evaluation::Evaluation;

pub struct Position {
    bitboards: Bitboards,
    white_occupancy_board: u64,
    black_occupancy_board: u64,
    hash: u64,
    centipawn_evaluation: i16,
    position_flags: u16,
}

impl Position {
    // TODO: what about mate
    pub fn evaluation(&self) -> Evaluation {
        assert!(Evaluation::MIN_CENTIPAWN_EVALUATION <= self.centipawn_evaluation);
        assert!(self.centipawn_evaluation <= Evaluation::MAX_CENTIPAWN_EVALUATION);
        Evaluation::from_raw(self.centipawn_evaluation)
    }

    pub const fn hash(&self) -> u64 {
        self.hash
    }

    // en passant
    // captured piece - kinda done
    // castling rights
    // promotion
    pub fn make_move(&mut self, move_to_make: Move) {
        // remove old piece
        // `captured_mask` may be zero, in which case nothing happens.
        // TODO: test if the zero mask is good for performance
        *self.get_piece_color_occupancy_board(move_to_make.captured_piece) ^= move_to_make.captured_mask;
        self.bitboards[move_to_make.captured_piece] ^= move_to_make.captured_mask;
        
        // move source to dest
        *self.get_piece_color_occupancy_board(move_to_make.moved_piece) ^= move_to_make.move_mask;
        self.bitboards[move_to_make.moved_piece] ^= move_to_make.move_mask;

        todo!()
    }

    pub fn unmake_move(&mut self, move_to_unmake: Move) {
        // move dest to source
        self.bitboards[move_to_unmake.moved_piece] ^= move_to_unmake.move_mask;
        *self.get_piece_color_occupancy_board(move_to_unmake.moved_piece) ^= move_to_unmake.move_mask;
        
        // set dest to old piece
        self.bitboards[move_to_unmake.captured_piece] ^= move_to_unmake.captured_mask;
        *self.get_piece_color_occupancy_board(move_to_unmake.captured_piece) ^= move_to_unmake.captured_mask;

        todo!()
    }

    fn get_piece_color_occupancy_board(&mut self, piece: PieceType) -> &mut u64 {
        match piece {
            PieceType::WhiteKing
            | PieceType::WhiteQueen
            | PieceType::WhiteRook
            | PieceType::WhiteBishop
            | PieceType::WhiteKnight
            | PieceType::WhitePawn => &mut self.white_occupancy_board,
            PieceType::BlackKing
            | PieceType::BlackQueen
            | PieceType::BlackRook
            | PieceType::BlackBishop
            | PieceType::BlackKnight
            | PieceType::BlackPawn => &mut self.black_occupancy_board,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    captured_piece: PieceType,
    moved_piece: PieceType,
    move_mask: u64,
    captured_mask: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PieceType {
    WhiteKing = 0,
    WhiteQueen = 1,
    WhiteRook = 2,
    WhiteBishop = 3,
    WhiteKnight = 4,
    WhitePawn = 5,
    BlackKing = 6,
    BlackQueen = 7,
    BlackRook = 8,
    BlackBishop = 9,
    BlackKnight = 10,
    BlackPawn = 11,
}

struct Bitboards {
    boards: [u64; 12],
}

impl Index<PieceType> for Bitboards {
    type Output = u64;
    fn index(&self, index: PieceType) -> &Self::Output {
        &self.boards[index as usize]
    }
}

impl IndexMut<PieceType> for Bitboards {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        &mut self.boards[index as usize]
    }
}
