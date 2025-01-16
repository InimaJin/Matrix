use std::{
    fmt,
    ops::{
        Mul, MulAssign
    }
};


pub struct Matrix {
    contents: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(init: f64, width: usize, height: usize) -> Result<Self, String> {
        if width == 0 || height == 0 {
            return Err(String::from("Width and height must be >= 0."));
        }
        let vec = vec![vec![init; width]; height];
        Ok(Self { contents: vec })
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
        
        Ok(Self { contents: vec })
    }

    pub fn width(&self) -> usize {
        self.contents[0].len()
    }
    pub fn height(&self) -> usize {
        self.contents.len()
    }
    //Number of
    pub fn size(&self) -> usize {
        self.width() * self.height()
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let (width, height) = (self.contents[0].len(), self.contents.len());
        for i in 0..height {
            output.push_str("[");
            for j in 0..width {
                let s = format!(" {} ", self.contents[i][j]);
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

/* Scalar multiplication */
impl Mul<f64> for Matrix {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self {
        let (width, height) = (self.contents[0].len(), self.contents.len());
        for i in 0..height {
            for j in 0..width {
                self.contents[i][j] *= rhs;
            }
        }
        self
    }
}
impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, rhs: f64) {
        let (width, height) = (self.contents[0].len(), self.contents.len());
        for i in 0..height {
            for j in 0..width {
                self.contents[i][j] *= rhs;
            }
        }
    }
}
