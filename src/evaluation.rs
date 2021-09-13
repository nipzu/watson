use std::fmt;

/// Wrapper struct for storing the heuristic evaluation of a position.
///
/// The value `Self::RESERVED_VALUE` is reserved and illegal to have as a value in this struct.
///
/// Values in the range
///     `MIN_CENTIPAWN_EVALUATION <= value <= MAX_CENTIPAWN_EVALUATION`
/// are used for centipawn evaluations.
///
/// Values greater than `MAX_CENTIPAWN_EVALUATION` in absolute
/// value are used for distance to mate evaluations. The distance to mate
/// is defined to be `i16::MAX - |value|`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Evaluation {
    value: i16,
}

impl Evaluation {
    pub const RESERVED_VALUE: i16 = i16::MIN;

    // leaves space for a mate in 2500
    pub const MAX_CENTIPAWN_EVALUATION: i16 = 30_000;
    pub const MIN_CENTIPAWN_EVALUATION: i16 = -Self::MAX_CENTIPAWN_EVALUATION;

    /// Construct a new Evaluation from a raw `i16` value.
    ///
    /// # Panics
    ///
    /// This function will panic if `value == i16::MIN`
    pub fn from_raw(value: i16) -> Self {
        // TODO: maybe debug_assert
        assert_ne!(value, Self::RESERVED_VALUE);
        Self { value }
    }

    /// Gets the raw value of the Evaluation struct as an `i16`.
    pub const fn to_raw(self) -> i16 {
        self.value
    }
}

impl fmt::Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Self::RESERVED_VALUE => unreachable!(),
            Self::MIN_CENTIPAWN_EVALUATION..=Self::MAX_CENTIPAWN_EVALUATION => {
                write!(f, "{}", self.value)
            }
            _ => write!(
                f,
                "M{}",
                self.value.signum() * (i16::MAX - self.value.abs())
            ),
        }
    }
}

// impl Neg for Evaluation {
//     type Output = Self;

//     fn neg(self) -> Self {
//         Self {
//             value: -self.value
//         }
//     }
// }

// pub fn heuristic_evaluation(position: Board) -> Evaluation {
//     todo!()
// }
