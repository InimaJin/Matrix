use std::{
    fmt,
    ops::{
        Mul, MulAssign, Add, AddAssign, Sub, SubAssign
    }
};


pub struct Matrix {
    content: Vec<Vec<f64>>
}

impl Matrix {
    pub fn build(init: f64, width: usize, height: usize) -> Result<Self, String> {
        if width == 0 || height == 0 {
            return Err(String::from("Width and height must be >= 0."));
        }
        let vec = vec![vec![init; width]; height];
        Ok(Self { content: vec })
    }

    pub fn from_vec(vec: Vec<Vec<f64>>) -> Result<Self, String> {
        if vec.is_empty() || vec[0].is_empty() { return Err(String::from("Width and height must be >= 0.")) }
        let width = vec[0].len();
        for vec in &vec[1..] {
            //All rows must have the same width
            if vec.len() != width {
                return Err(String::from("Invalid matrix."));
            }
        }
        
        Ok(Self { content: vec })
    }

    pub fn width(&self) -> usize {
        self.content[0].len()
    }
    pub fn height(&self) -> usize {
        self.content.len()
    }
    //Number of
    pub fn size(&self) -> usize {
        self.width() * self.height()
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let (width, height) = (self.content[0].len(), self.content.len());
        for i in 0..height {
            output.push_str("[");
            for j in 0..width {
                let s = format!(" {} ", self.content[i][j]);
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
impl Mul<f64> for Matrix {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.content[i][j] *= rhs;
            }
        }
        self
    }
}
impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.content[i][j] *= rhs;
            }
        }
    }
}

/* Matrix-Matrix addition and subtraction */
impl Add<Matrix> for Matrix {
    type Output = Self;
    fn add(mut self, rhs: Matrix) -> Self::Output {
        if self.width() != rhs.width() || self.height() != rhs.height() {
            return self;
        }

        for i in 0..self.height() {
            for j in 0..self.width() {
                self.content[i][j] += rhs.content[i][j];
            }
        }
        self
    }
}
impl AddAssign<Matrix> for Matrix {
    fn add_assign(&mut self, rhs: Matrix) {
        if self.width() != rhs.width() || self.height() != rhs.height() {
            return;
        }
        
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.content[i][j] += rhs.content[i][j];
            }
        }
    }
}
impl Sub<Matrix> for Matrix {
    type Output = Self;
    fn sub(self, rhs: Matrix) -> Self::Output {
        self + rhs * -1.0
    }
}
impl SubAssign<Matrix> for Matrix {
    fn sub_assign(&mut self, rhs: Matrix) {
        *self += rhs * -1.0;
    }
}

/* Matrix-Matrix multiplication */
impl Mul<Matrix> for Matrix {
    type Output = Self;
    fn mul(self, rhs: Matrix) -> Self::Output {
        if self.width() != rhs.height() { return self }

        let mut product = Self::build(0.0, rhs.width(), self.height()).unwrap();
        
        for row in 0..self.height() {
            for col in 0..rhs.width() {
                for pointer in 0..self.width() {
                    product.content[row][col] += self.content[row][pointer] * rhs.content[pointer][col];
                }
            }
        }
        
        product
    }
}
