use std::{fmt::Display, ops::Index};

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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
pub enum RotationType {
    Normal,
    Double,
    Inverse,
}

#[derive(Debug)]
pub enum Move {
    Face {
        face: FaceType,
        rotation_type: RotationType,
        /// How many slices to turn?
        depth: usize,
    },
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

    /// Asserts that this cube is valid.
    pub fn validate(&self) {}

    pub fn perform(self, mv: Move) -> Self {
        // Heavily optimised move-performing logic.
        macro_rules! face {
            ( $face:ident ) => {
                self.face($face).clone()
            };
            ( $face:ident cw ) => {
                self.face($face).rotate_cw()
            };
            ( $face:ident 2 ) => {
                self.face($face).rotate_double()
            };
            ( $face:ident ccw ) => {
                self.face($face).rotate_ccw()
            };
            ( $face:ident $depth:ident $target:ident $source_face:ident $source_type:ident ) => {
                self.face($face).overwrite_from(
                    $depth,
                    $target,
                    self.face($source_face),
                    $source_type,
                )
            };
        }

        macro_rules! face_depth {
            ( $depth:ident, ($($x:tt)*) ) => {
                // Unbox parentheses.
                face_depth!($depth, $($x)*)
            };
            ( $depth:ident, $face:ident ) => {
                face!($face)
            };
            ( $depth:ident, $face:ident cw ) => {
                face!($face cw)
            };
            ( $depth:ident, $face:ident 2 ) => {
                face!($face 2)
            };
            ( $depth:ident, $face:ident ccw ) => {
                face!($face ccw)
            };
            ( $depth:ident, $face:ident $target:ident $source_face:ident $source_type:ident ) => {
                face!($face $depth $target $source_face $source_type)
            };
        }

        macro_rules! perform {
            ( $depth:ident, $($x:tt)* ) => {
                [$(face_depth!($depth, $x),)*]
            };
        }

        Self {
            faces: match mv {
                Move::Face {
                    face: F,
                    rotation_type: RotationType::Normal,
                    depth,
                } => perform!(depth,
                    // Read this:
                    // "F clockwise"
                    (F cw)
                    // "R left comes from U bottom"
                    // (the left part of R's face is copied from the bottom part of U's face)
                    (R Left U Bottom)
                    (U Bottom L Right)
                    // "B is unchanged"
                    (B)
                    (L Right D Top)
                    (D Top R Left)
                ),
                Move::Face {
                    face: F,
                    rotation_type: RotationType::Double,
                    depth,
                } => perform!(depth,
                    (F 2)
                    (R Left L Right)
                    (U Bottom D Top)
                    (B)
                    (L Right R Left)
                    (D Top U Bottom)
                ),
                Move::Face {
                    face: F,
                    rotation_type: RotationType::Inverse,
                    depth,
                } => perform!(depth,
                    (F ccw)
                    (R Left D Top)
                    (U Bottom R Left)
                    (B)
                    (L Right U Bottom)
                    (D Top L Right)
                ),
                Move::Face {
                    face: R,
                    rotation_type: RotationType::Normal,
                    depth,
                } => perform!(depth,
                    (F Right D Right)
                    (R cw)
                    (U Right F Right)
                    (B Left U Right)
                    (L)
                    (D Right B Left)
                ),
                Move::Face {
                    face: R,
                    rotation_type: RotationType::Double,
                    depth,
                } => perform!(depth,
                    (F Right B Left)
                    (R 2)
                    (U Right D Right)
                    (B Left F Right)
                    (L)
                    (D Right U Right)
                ),
                Move::Face {
                    face: R,
                    rotation_type: RotationType::Inverse,
                    depth,
                } => perform!(depth,
                    (F Right U Right)
                    (R ccw)
                    (U Right B Left)
                    (B Left D Right)
                    (L)
                    (D Right F Right)
                ),
                _ => panic!("move {:#?} not recognised", mv),
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

    fn overwrite_row(&self, row: usize, data: [Colour; N]) -> Self {
        let mut face = self.clone();
        face.set_row(row, data);
        face
    }

    fn set_col(&mut self, col: usize, data: [Colour; N]) {
        for i in 0..N {
            self.rows[i][col] = data[i];
        }
    }

    fn overwrite_col(&self, col: usize, data: [Colour; N]) -> Self {
        let mut face = self.clone();
        face.set_col(col, data);
        face
    }

    /// Read this function:
    /// "overwrite \[depth\] slices on the \[target_type\] from \[source\]'s \[source_type\]"
    #[inline(always)]
    fn overwrite_from(
        &self,
        depth: usize,
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
        for i in 0..depth {
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
