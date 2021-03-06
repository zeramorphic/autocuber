use std::{collections::HashMap, fmt::Display, ops::Index, str::FromStr};
use wasm_bindgen::{prelude::*, JsCast};

/// Represents a *valid* (i.e. has all of the required pieces, not necessarily solvable) NxN cube.
/// Not `Copy` primarily as a lint.
#[derive(Debug, Clone)]
pub struct Cube<const N: usize> {
    /// Faces of the cube, ordered F R U B L D.
    faces: [Face<N>; 6],
}

/// A face of an NxN cube.
/// Not `Copy` primarily as a lint.
#[derive(Debug, Clone)]
pub struct Face<const N: usize> {
    rows: [[Colour; N]; N],
}

/// The colour of a face on an NxN cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
// Colours are often not constructed directly, but converted into from a face type.
#[allow(dead_code)]
pub enum Colour {
    Green,
    Red,
    White,
    Blue,
    Orange,
    Yellow,
}

impl Colour {
    /// Gets the letter name of this colour.
    pub fn letter(self) -> char {
        match self {
            Colour::Green => 'g',
            Colour::Red => 'r',
            Colour::White => 'w',
            Colour::Blue => 'b',
            Colour::Orange => 'o',
            Colour::Yellow => 'y',
        }
    }
}

/// A face on a cube.
/// Represented in Singmaster notation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FaceType {
    F,
    R,
    U,
    B,
    L,
    D,
}
use FaceType::*;

impl FromStr for FaceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F" => Ok(F),
            "R" => Ok(R),
            "U" => Ok(U),
            "B" => Ok(B),
            "L" => Ok(L),
            "D" => Ok(D),
            _ => Err(()),
        }
    }
}

impl Display for FaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            F => write!(f, "F"),
            R => write!(f, "R"),
            U => write!(f, "U"),
            B => write!(f, "B"),
            L => write!(f, "L"),
            D => write!(f, "D"),
        }
    }
}

impl Enumerable for FaceType {
    const N: usize = 6;

    fn enumerate() -> [Self; Self::N] {
        [F, R, U, B, L, D]
    }

    fn from_index(idx: usize) -> FaceType {
        unsafe { std::mem::transmute(idx as u8) }
    }

    fn index(&self) -> usize {
        *self as u8 as usize
    }
}

/// One of twelve edge types on a cube.
/// Edge names are derived from 2-axis (RL, UD) edge orientation.
/// The "key sticker" is written first.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum EdgeType {
    UR, UF, UL, UB,
    DR, DF, DL, DB,
    FR, FL, BR, BL,
}
use EdgeType::*;

impl FromStr for EdgeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UR" => Ok(UR),
            "UF" => Ok(UF),
            "UL" => Ok(UL),
            "UB" => Ok(UB),
            "DR" => Ok(DR),
            "DF" => Ok(DF),
            "DL" => Ok(DL),
            "DB" => Ok(DB),
            "FR" => Ok(FR),
            "FL" => Ok(FL),
            "BR" => Ok(BR),
            "BL" => Ok(BL),
            _ => Err(()),
        }
    }
}

impl Display for EdgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UR => write!(f, "UR"),
            UF => write!(f, "UF"),
            UL => write!(f, "UL"),
            UB => write!(f, "UB"),
            DR => write!(f, "DR"),
            DF => write!(f, "DF"),
            DL => write!(f, "DL"),
            DB => write!(f, "DB"),
            FR => write!(f, "FR"),
            FL => write!(f, "FL"),
            BR => write!(f, "BR"),
            BL => write!(f, "BL"),
        }
    }
}

impl Enumerable for EdgeType {
    const N: usize = 12;

    fn enumerate() -> [Self; Self::N] {
        [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BR, BL]
    }

    fn from_index(idx: usize) -> EdgeType {
        unsafe { std::mem::transmute(idx as u8) }
    }

    fn index(&self) -> usize {
        *self as u8 as usize
    }
}

impl EdgeType {
    /// The first face must precede the second face in the name of the edge,
    /// or None will be returned. That is, the first face must be the key sticker.
    pub fn from_faces_ordered(f1: FaceType, f2: FaceType) -> Option<EdgeType> {
        match (f1, f2) {
            (U, R) => Some(UR),
            (U, F) => Some(UF),
            (U, L) => Some(UL),
            (U, B) => Some(UB),
            (D, R) => Some(DR),
            (D, F) => Some(DF),
            (D, L) => Some(DL),
            (D, B) => Some(DB),
            (F, R) => Some(FR),
            (F, L) => Some(FL),
            (B, R) => Some(BR),
            (B, L) => Some(BL),
            _ => None,
        }
    }

    /// Yields the edge formed from the intersection of the two faces, along with
    /// the parity of the given edge. The parity is reversed if the input faces are reversed.
    pub fn from_faces(f1: FaceType, f2: FaceType) -> Option<(EdgeType, CyclicGroup<2>)> {
        if let Some(value) = Self::from_faces_ordered(f1, f2) {
            Some((value, CyclicGroup::new(0)))
        } else {
            Self::from_faces_ordered(f2, f1).map(|x| (x, CyclicGroup::new(1)))
        }
    }
}

/// One of twelve corner types on a cube.
/// Corner types are named according to the member of each axis: FB, UD, RL.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum CornerType {
    FUR, FUL, FDR, FDL,
    BUR, BUL, BDR, BDL,
}
use CornerType::*;

impl FromStr for CornerType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FUR" => Ok(FUR),
            "FUL" => Ok(FUL),
            "FDR" => Ok(FDR),
            "FDL" => Ok(FDL),
            "BUR" => Ok(BUR),
            "BUL" => Ok(BUL),
            "BDR" => Ok(BDR),
            "BDL" => Ok(BDL),
            _ => Err(()),
        }
    }
}

impl Display for CornerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FUR => write!(f, "FUR"),
            FUL => write!(f, "FUL"),
            FDR => write!(f, "FDR"),
            FDL => write!(f, "FDL"),
            BUR => write!(f, "BUR"),
            BUL => write!(f, "BUL"),
            BDR => write!(f, "BDR"),
            BDL => write!(f, "BDL"),
        }
    }
}

impl Enumerable for CornerType {
    const N: usize = 8;

    fn enumerate() -> [Self; Self::N] {
        [FUR, FUL, FDR, FDL, BUR, BUL, BDR, BDL]
    }

    fn from_index(idx: usize) -> CornerType {
        unsafe { std::mem::transmute(idx as u8) }
    }

    fn index(&self) -> usize {
        *self as u8 as usize
    }
}

impl CornerType {
    /// The first face must be on the FB axis, the second on the UD axis, and the third on the RL axis.
    pub fn from_faces_ordered(f1: FaceType, f2: FaceType, f3: FaceType) -> Option<CornerType> {
        match (f1, f2, f3) {
            (F, U, R) => Some(FUR),
            (F, U, L) => Some(FUL),
            (F, D, R) => Some(FDR),
            (F, D, L) => Some(FDL),
            (B, U, R) => Some(BUR),
            (B, U, L) => Some(BUL),
            (B, D, R) => Some(BDR),
            (B, D, L) => Some(BDL),
            _ => None,
        }
    }
}

/// An axis on a cube.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Axis {
    FB,
    RL,
    UD,
}
use Axis::*;

impl FromStr for Axis {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FB" => Ok(FB),
            "RL" => Ok(RL),
            "UD" => Ok(UD),
            _ => Err(()),
        }
    }
}

/// These impls are safe since colour and face type are `repr(u8)` and have the same possible discriminants.
impl From<FaceType> for Colour {
    fn from(face: FaceType) -> Self {
        unsafe { std::mem::transmute(face) }
    }
}
impl From<Colour> for FaceType {
    fn from(colour: Colour) -> Self {
        unsafe { std::mem::transmute(colour) }
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RotationType {
    Normal,
    Double,
    Inverse,
}

impl RotationType {
    pub fn inverse(self) -> RotationType {
        match self {
            RotationType::Normal => RotationType::Inverse,
            RotationType::Double => RotationType::Double,
            RotationType::Inverse => RotationType::Normal,
        }
    }

    pub fn rotations(self) -> i32 {
        match self {
            RotationType::Normal => 1,
            RotationType::Double => 2,
            RotationType::Inverse => -1,
        }
    }

    /// None is returned if no rotation was required.
    pub fn from_rotations(n: i32) -> Option<RotationType> {
        match ((n % 4) + 4) % 4 {
            0 => None,
            1 => Some(RotationType::Normal),
            2 => Some(RotationType::Double),
            3 => Some(RotationType::Inverse),
            _ => unreachable!(),
        }
    }
}

impl Display for RotationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RotationType::Normal => write!(f, ""),
            RotationType::Double => write!(f, "2"),
            RotationType::Inverse => write!(f, "'"),
        }
    }
}

/// Gives the inverse of a RotationType.
#[wasm_bindgen(js_name = inverse)]
#[doc(hidden)]
#[allow(dead_code)]
pub fn inverse_wasm(rot: RotationType) -> RotationType {
    rot.inverse()
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Move {
    pub axis: Axis,
    #[wasm_bindgen(js_name = rotationType)]
    pub rotation_type: RotationType,
    // We turn all slices from `start_depth` to `end_depth`.
    // If `start_depth = 0, end_depth = 1`, this is a normal turn.
    // If `start_depth = 1, end_depth = 2`, this is a slice turn.
    // If `start_depth = 0, end_depth = 2`, this is a wide turn.
    // If `start_depth = 2, end_depth = 3`, this is an inverse turn on the opposite face.
    #[wasm_bindgen(js_name = startDepth)]
    pub start_depth: usize,
    #[wasm_bindgen(js_name = endDepth)]
    pub end_depth: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const N: usize = 3;
        let mut chars = s.chars();
        let face_char = chars.next().ok_or(())?;
        let turn_direction = match face_char {
            'M' => 'L',
            'E' => 'D',
            'S' => 'F',
            x => x,
        };
        let face: FaceType = turn_direction.to_uppercase().collect::<String>().parse()?;
        let mut end_depth = if face_char.is_lowercase() { 2 } else { 1 };
        let mut start_depth = match face_char {
            'M' | 'E' | 'S' => {
                end_depth = 2;
                1
            }
            _ => 0,
        };
        let mut rotation_type = RotationType::Normal;
        for modification in chars {
            match modification {
                'w' => end_depth = 2,
                '2' => rotation_type = RotationType::Double,
                '\'' => {
                    // Sometimes, algorithms have things like U2', but we don't care
                    // about the direction of double turns.
                    if rotation_type != RotationType::Double {
                        rotation_type = RotationType::Inverse
                    }
                }
                _ => return Err(()),
            }
        }
        let axis = match face {
            F => FB,
            R => RL,
            U => UD,
            B => {
                rotation_type = rotation_type.inverse();
                let d = start_depth;
                start_depth = N - end_depth;
                end_depth = N - d;
                FB
            }
            L => {
                rotation_type = rotation_type.inverse();
                let d = start_depth;
                start_depth = N - end_depth;
                end_depth = N - d;
                RL
            }
            D => {
                rotation_type = rotation_type.inverse();
                let d = start_depth;
                start_depth = N - end_depth;
                end_depth = N - d;
                UD
            }
        };
        Ok(Self {
            axis,
            rotation_type,
            start_depth,
            end_depth,
        })
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.start_depth, self.end_depth) {
            (0, 1) => match self.axis {
                FB => write!(f, "F{}", self.rotation_type),
                RL => write!(f, "R{}", self.rotation_type),
                UD => write!(f, "U{}", self.rotation_type),
            },
            (1, 2) => match self.axis {
                FB => write!(f, "S{}", self.rotation_type),
                RL => write!(f, "M{}", self.rotation_type.inverse()),
                UD => write!(f, "E{}", self.rotation_type.inverse()),
            },
            (2, 3) => match self.axis {
                FB => write!(f, "B{}", self.rotation_type.inverse()),
                RL => write!(f, "L{}", self.rotation_type.inverse()),
                UD => write!(f, "D{}", self.rotation_type.inverse()),
            },
            _ => {
                // Fallback if we don't know how else to display the move:
                write!(
                    f,
                    "{:?}{}-{}{}",
                    self.axis, self.start_depth, self.end_depth, self.rotation_type
                )
            }
        }
    }
}

#[wasm_bindgen]
impl Move {
    pub fn new(
        axis: Axis,
        rotation_type: RotationType,
        start_depth: usize,
        end_depth: usize,
    ) -> Self {
        Self {
            axis,
            rotation_type,
            start_depth,
            end_depth,
        }
    }

    pub fn inverse(self) -> Self {
        Self {
            rotation_type: self.rotation_type.inverse(),
            ..self
        }
    }

    pub fn clone_move(&self) -> Self {
        *self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MoveSequence {
    pub moves: Vec<Move>,
}

/// As a magma, move sequences are treated like a free group.
impl Magma for MoveSequence {
    fn op(self, other: Self) -> Self {
        Self {
            moves: other.moves.into_iter().chain(self.moves).collect(),
        }
    }
}

impl Semigroup for MoveSequence {}

impl InverseSemigroup for MoveSequence {
    fn inverse(&self) -> Self {
        Self {
            moves: self.moves.iter().rev().map(|mv| mv.inverse()).collect(),
        }
    }
}

impl MoveSequence {
    pub fn canonicalise(self) -> Self {
        if self.moves.is_empty() {
            return self;
        }

        let mut moves = Vec::new();

        // If two consecutive moves have the same axis, try to collapse them into the same real move.
        let mut current_axis = self.moves[0].axis;
        let mut current_axis_moves = Vec::<Move>::new();

        let mut process_axis = |current_axis: Axis, current_axis_moves: Vec<Move>| {
            // Canonicalise the list of current axis moves, since they all must commute.
            let mut turns_by_slice = HashMap::<usize, i32>::new();
            for mv in current_axis_moves {
                for slice in mv.start_depth..mv.end_depth {
                    *turns_by_slice.entry(slice).or_default() += mv.rotation_type.rotations();
                }
            }
            // Convert this back into a move sequence.
            // For each continuous block of slices rotating the same amount, convert it into a move.
            let mut new_moves = Vec::new();
            let (mut expected_slice, mut expected_rotations) = {
                let (&k, &v) = turns_by_slice.iter().next().unwrap();
                (k, v)
            };
            let mut current_start_slice = expected_slice;

            for (slice, rotations) in turns_by_slice {
                if slice == expected_slice && rotations == expected_rotations {
                    // Continue accepting more slices on this wide move.
                    expected_slice += 1;
                } else {
                    // Finish this wide move.
                    if let Some(rotation_type) = RotationType::from_rotations(expected_rotations) {
                        new_moves.push(Move {
                            axis: current_axis,
                            rotation_type,
                            start_depth: current_start_slice,
                            end_depth: expected_slice,
                        });
                    }

                    // Set up the current start slice, and expected next slice and rotations.
                    current_start_slice = slice;
                    expected_slice = slice + 1;
                    expected_rotations = rotations;
                }
            }

            if let Some(rotation_type) = RotationType::from_rotations(expected_rotations) {
                new_moves.push(Move {
                    axis: current_axis,
                    rotation_type,
                    start_depth: current_start_slice,
                    end_depth: expected_slice,
                });
            }

            moves.extend(new_moves);
        };

        for mv in self.moves {
            if mv.axis != current_axis {
                process_axis(current_axis, std::mem::take(&mut current_axis_moves));
                current_axis = mv.axis;
            }
            current_axis_moves.push(mv);
        }
        process_axis(current_axis, std::mem::take(&mut current_axis_moves));

        moves.extend(current_axis_moves);
        Self { moves }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<Move>")]
    pub type MoveSequenceConv;
}

impl From<MoveSequence> for MoveSequenceConv {
    fn from(alg: MoveSequence) -> Self {
        alg.moves
            .into_iter()
            .map(JsValue::from)
            .collect::<js_sys::Array>()
            .unchecked_into::<MoveSequenceConv>()
    }
}

impl FromStr for MoveSequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self { moves: Vec::new() };
        for value in s.split(' ') {
            result.moves.push(value.parse()?);
        }
        Ok(result)
    }
}

impl Display for MoveSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, mv) in self.moves.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", mv)?;
        }
        Ok(())
    }
}

impl<const N: usize> Cube<N> {
    pub fn new() -> Self {
        Self {
            faces: [
                Face::new(F),
                Face::new(R),
                Face::new(U),
                Face::new(B),
                Face::new(L),
                Face::new(D),
            ],
        }
    }

    pub fn face(&self, ty: FaceType) -> &Face<N> {
        &self.faces[ty as usize]
    }

    pub fn perform(self, mv: Move) -> Self {
        // Heavily optimised move-performing logic.
        macro_rules! face {
            ( $start_depth:ident, $end_depth:ident, ($($x:tt)*) ) => {
                // Unbox parentheses.
                face!($start_depth, $end_depth, $($x)*)
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident ) => {
                self.face($face).clone()
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident cw ) => {
                if $start_depth == 0 {
                    self.face($face).rotate_cw()
                } else {
                    self.face($face).clone()
                }
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident 2 ) => {
                if $start_depth == 0 {
                    self.face($face).rotate_double()
                } else {
                    self.face($face).clone()
                }
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident ccw ) => {
                if $start_depth == 0 {
                    self.face($face).rotate_ccw()
                } else {
                    self.face($face).clone()
                }
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident b cw ) => {
                if $end_depth == N {
                    self.face($face).rotate_cw()
                } else {
                    self.face($face).clone()
                }
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident b 2 ) => {
                if $end_depth == N {
                    self.face($face).rotate_double()
                } else {
                    self.face($face).clone()
                }
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident b ccw ) => {
                if $end_depth == N {
                    self.face($face).rotate_ccw()
                } else {
                    self.face($face).clone()
                }
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident $target:ident $source_face:ident $source_type:ident ) => {
                self.face($face).overwrite_from(
                    $start_depth,
                    $end_depth,
                    $target,
                    self.face($source_face),
                    $source_type,
                )
            };
        }

        macro_rules! perform {
            ( $start_depth:ident, $end_depth:ident, $($x:tt)* ) => {
                [$(face!($start_depth, $end_depth, $x),)*]
            };
        }

        Self {
            faces: match mv {
                // FB turns
                Move {
                    axis: FB,
                    rotation_type: RotationType::Normal,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    // Read this:
                    // "F is clockwise, but only if the front face is modified"
                    (F cw)
                    // "R left comes from U bottom"
                    // (the left part of R's face is copied from the bottom part of U's face)
                    (R Left U Bottom)
                    (U Bottom L Right)
                    // "B is clockwise, but only if the back face is modified" (back face signalled by the `b` character)
                    (B b cw)
                    (L Right D Top)
                    (D Top R Left)
                ),
                Move {
                    axis: FB,
                    rotation_type: RotationType::Double,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F 2)
                    (R Left L Right)
                    (U Bottom D Top)
                    (B b 2)
                    (L Right R Left)
                    (D Top U Bottom)
                ),
                Move {
                    axis: FB,
                    rotation_type: RotationType::Inverse,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F ccw)
                    (R Left D Top)
                    (U Bottom R Left)
                    (B b ccw)
                    (L Right U Bottom)
                    (D Top L Right)
                ),
                // RL turns
                Move {
                    axis: RL,
                    rotation_type: RotationType::Normal,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F Right D Right)
                    (R cw)
                    (U Right F Right)
                    (B Left U Right)
                    (L b cw)
                    (D Right B Left)
                ),
                Move {
                    axis: RL,
                    rotation_type: RotationType::Double,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F Right B Left)
                    (R 2)
                    (U Right D Right)
                    (B Left F Right)
                    (L b 2)
                    (D Right U Right)
                ),
                Move {
                    axis: RL,
                    rotation_type: RotationType::Inverse,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F Right U Right)
                    (R ccw)
                    (U Right B Left)
                    (B Left D Right)
                    (L b ccw)
                    (D Right F Right)
                ),
                // UD turns
                Move {
                    axis: UD,
                    rotation_type: RotationType::Normal,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F Top R Top)
                    (R Top B Top)
                    (U cw)
                    (B Top L Top)
                    (L Top F Top)
                    (D b cw)
                ),
                Move {
                    axis: UD,
                    rotation_type: RotationType::Double,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F Top B Top)
                    (R Top L Top)
                    (U 2)
                    (B Top F Top)
                    (L Top R Top)
                    (D b 2)
                ),
                Move {
                    axis: UD,
                    rotation_type: RotationType::Inverse,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F Top L Top)
                    (R Top F Top)
                    (U ccw)
                    (B Top R Top)
                    (L Top B Top)
                    (D b ccw)
                ),
            },
        }
    }
}

impl<const N: usize> Display for Cube<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write the U face.
        for i in 0..N {
            // Write each row.
            for _ in 0..N {
                // Add a gap at the start for the L face.
                write!(f, "  ")?;
            }
            // Display the row.
            for j in 0..N {
                write!(f, "{} ", self.face(U)[(i, j)].letter())?;
            }
            writeln!(f)?;
        }

        // Write the L, F, R, B faces.
        for i in 0..N {
            for face in [L, F, R, B] {
                for j in 0..N {
                    write!(f, "{} ", self.face(face)[(i, j)].letter())?;
                }
            }
            writeln!(f)?;
        }

        // Write the D face.
        for i in 0..N {
            // Write each row.
            for _ in 0..N {
                // Add a gap at the start for the L face.
                write!(f, "  ")?;
            }
            // Display the row.
            for j in 0..N {
                write!(f, "{} ", self.face(D)[(i, j)].letter())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
enum FaceSegment {
    Top,
    Right,
    Bottom,
    Left,
}
use FaceSegment::*;

use crate::group::{CyclicGroup, Enumerable, InverseSemigroup, Magma, Semigroup};

// The range is there as an optimisation for the compiler, since we
// know the size of each array at compile time. It also helps unify
// code style across each of the different functions.
#[allow(clippy::needless_range_loop)]
impl<const N: usize> Face<N> {
    pub fn new(ty: FaceType) -> Self {
        Self {
            rows: [[ty.into(); N]; N],
        }
    }

    fn row(&self, row: usize) -> [Colour; N] {
        self.rows[row]
    }

    fn row_rev(&self, row: usize) -> [Colour; N] {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self[(row, N - 1 - i)]);
        }
        unsafe { std::mem::transmute_copy(&array) }
    }

    fn col(&self, col: usize) -> [Colour; N] {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self[(i, col)]);
        }
        unsafe { std::mem::transmute_copy(&array) }
    }

    fn col_rev(&self, col: usize) -> [Colour; N] {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self[(N - 1 - i, col)]);
        }
        unsafe { std::mem::transmute_copy(&array) }
    }

    fn rotate_cw(&self) -> Self {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self.col_rev(i));
        }
        Self {
            rows: unsafe { std::mem::transmute_copy(&array) },
        }
    }

    fn rotate_ccw(&self) -> Self {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self.col(N - 1 - i));
        }
        Self {
            rows: unsafe { std::mem::transmute_copy(&array) },
        }
    }

    fn rotate_double(&self) -> Self {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self.row_rev(N - 1 - i));
        }
        Self {
            rows: unsafe { std::mem::transmute_copy(&array) },
        }
    }

    fn set_row(&mut self, row: usize, data: [Colour; N]) {
        self.rows[row] = data;
    }

    fn set_col(&mut self, col: usize, data: [Colour; N]) {
        for i in 0..N {
            self.rows[i][col] = data[i];
        }
    }

    /// Read this function:
    /// "overwrite \[depth\] slices on the \[target_type\] from \[source\]'s \[source_type\]"
    #[inline(always)]
    fn overwrite_from(
        &self,
        start_depth: usize,
        end_depth: usize,
        target_type: FaceSegment,
        source: &Face<N>,
        source_type: FaceSegment,
    ) -> Self {
        // Considering the face segments on the source and the target,
        // when we collect an individual row or column from the source,
        // we might need to flip it such that its image on the target is correctly oriented.

        // The source/target is said to go "clockwise" if the row/column index increases as we rotate clockwise around the given face.
        let source_clockwise = matches!(source_type, Top | Right);
        let target_clockwise = matches!(target_type, Top | Right);
        // If the source and target's orientations differ, we must reverse the indices of each element in the source,
        // that is, reverse the row or column itself.
        let reverse_direction = source_clockwise != target_clockwise;

        let mut face = self.clone();
        // i counts from left to right.
        for i in start_depth..end_depth {
            // j counts from right to left.
            let j = N - 1 - i;
            let source_row = match (source_type, reverse_direction) {
                (Top, false) => source.row(i),
                (Top, true) => source.row_rev(i),
                (Right, false) => source.col(j),
                (Right, true) => source.col_rev(j),
                (Bottom, false) => source.row(j),
                (Bottom, true) => source.row_rev(j),
                (Left, false) => source.col(i),
                (Left, true) => source.col_rev(i),
            };

            match target_type {
                Top => face.set_row(i, source_row),
                Right => face.set_col(j, source_row),
                Bottom => face.set_row(j, source_row),
                Left => face.set_col(i, source_row),
            };
        }
        face
    }
}

impl<const N: usize> Index<(usize, usize)> for Face<N> {
    type Output = Colour;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.rows[row][col]
    }
}
