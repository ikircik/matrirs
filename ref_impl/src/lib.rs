use std::ops::{Add, AddAssign};

pub struct Matrix {
    row_count: usize,
    column_count: usize,
    elements: Vec<f64>,
}

impl Matrix {
    pub fn new(row_count: usize, column_count: usize, elements: Vec<f64>) -> Matrix {
        Matrix { row_count, column_count, elements }
    }

    pub fn get_row_count(&self) -> &usize {
        &self.row_count
    }

    pub fn get_column_count(&self) -> &usize {
        &self.column_count
    }

    pub fn get_elements(&self) -> &Vec<f64> {
        &self.elements
    }

    pub fn set_elements(&mut self, elements: Vec<f64>) -> Result<(), ()> {
        if elements.len() == self.row_count * self.column_count {
            self.elements = elements;
            return Ok(());
        }
        Err(())
    }

    pub fn direct_sum(&self, other: Matrix) -> Matrix {
        let row_count = self.row_count + other.row_count;
        let column_count = self.column_count + other.column_count;
        let mut matrix = Matrix {
            row_count,
            column_count,
            elements: vec![0.0; row_count * column_count],
        };

        for i in 0..self.row_count {
            for j in 0..self.column_count {
                matrix.elements[i * column_count + j] = self.elements[i * self.column_count + j];
            }
        }

        for i in 0..other.row_count {
            for j in 0..other.column_count {
                matrix.elements[self.row_count * column_count + self.column_count * (i*2+1) + j]
                = other.elements[i * other.column_count + j];
            }
        }

        matrix
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.row_count == other.row_count &&
        self.column_count == other.column_count &&
        self.elements == other.elements
    }
}

impl Add for Matrix {
    type Output = Result<Matrix, ()>;
    fn add(self, other: Self) -> Self::Output {
        if &self.row_count != &other.row_count || &self.column_count != &other.column_count {
            return Err(());
        }
        let mut elements = vec![0.0; self.elements.len()];
        for i in 0..self.elements.len() {
            elements[i] = &self.elements[i] + &other.elements[i];
        }
        Ok(Matrix { row_count: self.row_count, column_count: self.column_count, elements })
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, other: Self) {
        if &self.row_count != &other.row_count || &self.column_count != &other.column_count {
            panic!()
        }
        for i in 0..self.elements.len() {
            self.elements[i] = &self.elements[i] + &other.elements[i];
        }
    }
}
