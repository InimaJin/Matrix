use std::{
    fmt,
    ops::{
        Mul, MulAssign
    }
};


pub struct Matrix {
    contents: Vec<Vec<f32>>
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        let vec = vec![vec![0.0; width]; height];
        Self { contents: vec }
    }

    pub fn from_vec(vec: Vec<Vec<f32>>) -> Result<Self, String> {
        if vec.is_empty() { return Ok(Self { contents: vec![] }) }
        let width = vec[0].len();
        for vec in &vec[1..] {
            if vec.len() != width {
                return Err(String::from("Invalid matrix."));
            }
        }
        
        Ok(Self { contents: vec })
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.is_empty() { return write!(f, "[ ]") }

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
impl Mul<f32> for Matrix {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        if self.contents.is_empty() { return self }
        let mut res = self;
        let (width, height) = (res.contents[0].len(), res.contents.len());
        for i in 0..height {
            for j in 0..width {
                res.contents[i][j] *= rhs;
            }
        }
        res
    }
}
impl MulAssign<f32> for Matrix {
    fn mul_assign(&mut self, rhs: f32) {
        if self.contents.is_empty() { return }
        let (width, height) = (self.contents[0].len(), self.contents.len());
        for i in 0..height {
            for j in 0..width {
                self.contents[i][j] *= rhs;
            }
        }
    }
}
