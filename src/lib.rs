use std::{
    cmp::{PartialEq, PartialOrd},
    default::Default,
    error::Error,
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Clone)]
pub struct Matrix<T> {
    grid: Vec<Vec<T>>,
}

/* Types implementing this trait can be used to build matrices.
 * T::default() should be 0. */
pub trait MatrixElement<T>:
    Copy
    + Clone
    + Default
    + PartialEq
    + PartialOrd
    + Add<Output = T>
    + AddAssign
    + Sub<Output = T>
    + SubAssign
    + Mul<Output = T>
    + MulAssign
    + Div<Output = T>
    + DivAssign
{
}
impl<T> MatrixElement<T> for T where
    T: Copy
        + Clone
        + Default
        + PartialEq
        + PartialOrd
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
{
}

impl<T: MatrixElement<T>> Matrix<T> {
    const DIMENSION_ERR: &'static str = "Width and height must be >= 0.";

    /* Constructs a (width x height) matrix in which every entry gets the value of 'init'. */
    pub fn build(init: T, width: usize, height: usize) -> Result<Self, Box<dyn Error>> {
        if width == 0 || height == 0 {
            return Err(Box::from(Self::DIMENSION_ERR));
        }
        let vec = vec![vec![init; width]; height];
        Ok(Self { grid: vec })
    }

    /* Constructs a matrix from a two-dimensional vector. */
    pub fn from_vec(vec: Vec<Vec<T>>) -> Result<Self, Box<dyn Error>> {
        if vec.is_empty() || vec[0].is_empty() {
            return Err(Box::from(Self::DIMENSION_ERR));
        }
        let width = vec[0].len();
        for vec in &vec[1..] {
            //All rows must have the same width
            if vec.len() != width {
                return Err(Box::from("Inconsistent row sizes."));
            }
        }

        Ok(Self { grid: vec })
    }

    /* Creates and returns a scalar multiple 'lambda' of the identity matrix. */
    pub fn build_scalar_matrix(lambda: T, width: usize) -> Result<Self, Box<dyn Error>> {
        let mut matrix = Self::build(T::default(), width, width)?;
        for i in 0..width {
            matrix.grid[i][i] = lambda;
        }
        Ok(matrix)
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }
    pub fn height(&self) -> usize {
        self.grid.len()
    }
    pub fn size(&self) -> usize {
        self.width() * self.height()
    }
    /* Returns a reference to the element at position (row, col).
     * Top left is (0,0) */
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.grid.get(row)?.get(col)
    }
    /* Updates the element at position (row, col) to 'new' and returns the element
     * previously at that position. */
    pub fn set(&mut self, row: usize, col: usize, new: T) -> Option<T> {
        let old = *self.get(row, col)?;
        self.grid[row][col] = new;
        Some(old)
    }
    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        if row1 < self.grid.len() && row2 < self.grid.len() {
            self.grid.swap(row1, row2);
        }
    }

    /* Transforms the matrix into row echelon form (no full reduction). */
    pub fn to_row_echelon(&mut self) {
        let zero = T::default();
        for pivot_position in 0..self.width() - 1 {
            let mut pivot = self.grid[pivot_position][pivot_position];
            //Attempt to swap rows in case pivot == 0
            if pivot == zero {
                for row in pivot_position + 1..self.height() {
                    if self.grid[row][pivot_position] != zero {
                        self.swap_rows(pivot_position, row);

                        pivot = self.grid[pivot_position][pivot_position];
                        break;
                    }
                }
                //Pivot hasn't changed, meaning all elements in column below pivot are 0 as well.
                if pivot == zero {
                    continue;
                }
            }
            for row in pivot_position + 1..self.height() {
                if self.grid[row][pivot_position] == zero {
                    continue;
                }

                /* fac is the factor by which the pivot would be multiplied in order for the
                 * elements below the pivot to become 0 when subtracting the product of pivot and
                 * fac. */
                let fac = self.grid[row][pivot_position] / pivot;
                self.grid[row][pivot_position] = zero;
                for col in pivot_position + 1..self.width() {
                    let subtract = fac * self.grid[pivot_position][col];
                    self.grid[row][col] -= subtract;
                }
            }
        }
    }

    /* Computes and returns the determinant of an nxn matrix.
     * For n > 3, the matrix is cloned and brought into row echelon form first. */
    pub fn det(&self) -> Option<T> {
        if self.width() != self.height() {
            return None;
        }
        return Some(match self.width() {
            1 => self.grid[0][0],
            2 => self.det_2x2(),
            3 => self.det_3x3(),
            _ => {
                let mut m = self.clone();
                m.to_row_echelon();
                let mut d = m.grid[0][0];
                //Determinant as the product of the elements on the main diagonal
                for i in 1..m.height() {
                    d *= m.grid[i][i];
                }
                d
            }
        });
    }
    fn det_2x2(&self) -> T {
        return self.grid[0][0] * self.grid[1][1] - self.grid[1][0] * self.grid[0][1];
    }

    /* Computation using rule of Sarrus */
    fn det_3x3(&self) -> T {
        let mut res = T::default();
        /* top holds the values of the first row within each iteration.
         * It serves as the entry point for each diagonal multiplication.
         * add dictates whether tmp_prod should be added or subtracted to/from res. */
        let (mut top, mut add) = (0, true);
        while top >= 0 {
            let mut col = top as usize;
            //Temporary product of values in a diagonal
            let mut tmp_prod = self.grid[0][top as usize];
            for row in 0..self.height() {
                if row != 0 {
                    tmp_prod *= self.grid[row][col];
                }
                if add {
                    col = (col + 1) % self.width();
                } else {
                    if col == 0 {
                        col = self.width() - 1;
                    } else {
                        col -= 1;
                    }
                }
            }
            res = if add { res + tmp_prod } else { res - tmp_prod };

            if top as usize == self.width() - 1 && add {
                //Inversion of direction
                add = false;
            } else {
                top = if add { top + 1 } else { top - 1 };
            }
        }
        res
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let (width, height) = (self.grid[0].len(), self.grid.len());
        for i in 0..height {
            output.push_str("[");
            for j in 0..width {
                let s = format!(" {} ", self.grid[i][j]);
                output.push_str(&s);
            }
            output.push_str("]");
            if i != height - 1 {
                output.push_str("\n");
            }
        }
        write!(f, "{}", output)
    }
}

/* Matrix-Scalar multiplication */
impl<T: MatrixElement<T>> Mul<T> for Matrix<T> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.grid[i][j] *= rhs;
            }
        }
        self
    }
}
impl<T: MatrixElement<T>> MulAssign<T> for Matrix<T> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.grid[i][j] *= rhs;
            }
        }
    }
}

/* Matrix-Matrix addition and subtraction */
impl<T: MatrixElement<T>> Add<Matrix<T>> for Matrix<T> {
    type Output = Self;
    fn add(mut self, rhs: Matrix<T>) -> Self::Output {
        if self.width() != rhs.width() || self.height() != rhs.height() {
            return self;
        }

        for i in 0..self.height() {
            for j in 0..self.width() {
                self.grid[i][j] += rhs.grid[i][j];
            }
        }
        self
    }
}
impl<T: MatrixElement<T>> AddAssign<Matrix<T>> for Matrix<T> {
    fn add_assign(&mut self, rhs: Matrix<T>) {
        if self.width() != rhs.width() || self.height() != rhs.height() {
            return;
        }

        for i in 0..self.height() {
            for j in 0..self.width() {
                self.grid[i][j] += rhs.grid[i][j];
            }
        }
    }
}
impl<T: MatrixElement<T>> Sub<Matrix<T>> for Matrix<T> {
    type Output = Self;
    fn sub(mut self, rhs: Matrix<T>) -> Self::Output {
        if self.width() != rhs.width() || self.height() != rhs.height() {
            return self;
        }
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.grid[i][j] -= rhs.grid[i][j];
            }
        }
        self
    }
}
impl<T: MatrixElement<T>> SubAssign<Matrix<T>> for Matrix<T> {
    fn sub_assign(&mut self, rhs: Matrix<T>) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.grid[i][j] -= rhs.grid[i][j];
            }
        }
    }
}

/* Matrix-Matrix multiplication */
impl<T: MatrixElement<T>> Mul<Matrix<T>> for Matrix<T> {
    type Output = Self;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.width() != rhs.height() {
            return self;
        }

        //Does not panic because matrix widths and heights are > 0
        let mut product = Self::build(T::default(), rhs.width(), self.height()).unwrap();

        for row in 0..self.height() {
            for col in 0..rhs.width() {
                for pointer in 0..self.width() {
                    product.grid[row][col] += self.grid[row][pointer] * rhs.grid[pointer][col];
                }
            }
        }

        product
    }
}

impl<T: MatrixElement<T>> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.width() != other.width() || self.height() != other.height() {
            return false;
        }
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.grid[i][j] != other.grid[i][j] {
                    return false;
                }
            }
        }
        true
    }
}
