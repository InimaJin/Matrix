use std::{
    error::Error,
    fmt,
    ops::{
        Mul, MulAssign, Add, AddAssign, Sub, SubAssign
    }
};


pub struct Matrix {
    content: Vec<Vec<f64>>
}

impl Matrix {
    const DIMENSION_ERR: &'static str = "Width and height must be >= 0.";
    pub fn build(init: f64, width: usize, height: usize) -> Result<Self, Box<dyn Error>> {
        if width == 0 || height == 0 {
            return Err( Box::from(Self::DIMENSION_ERR) );
        }
        let vec = vec![vec![init; width]; height];
        Ok(Self { content: vec })
    }

    pub fn from_vec(vec: Vec<Vec<f64>>) -> Result<Self, Box<dyn Error>> {
        if vec.is_empty() || vec[0].is_empty() { return Err( Box::from(Self::DIMENSION_ERR) ) }
        let width = vec[0].len();
        for vec in &vec[1..] {
            //All rows must have the same width
            if vec.len() != width {
                return Err(Box::from("Inconsistent row sizes."));
            }
        }
        
        Ok(Self { content: vec })
    }

    
    /* Creates and returns a scalar multiple 'lambda' of the identity matrix */
    pub fn scalar_matrix(lambda: f64, width: usize) -> Result<Self, Box<dyn Error>> {
        let mut matrix = Self::build(0.0, width, width)?;
        for i in 0..width {
            matrix.content[i][i] = lambda;
        }
        Ok(matrix)
    }

    pub fn width(&self) -> usize {
        self.content[0].len()
    }
    pub fn height(&self) -> usize {
        self.content.len()
    }
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
        
        //Does not panic because matrix widths and heights are > 0
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
