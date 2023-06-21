use std::{ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign}, fmt::Display};

#[derive(Debug)]
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

    pub fn get_rows(&self) -> Vec<Vec<f64>> {
        if self.row_count == 0 {
            return vec![];
        }
        self.elements.chunks(self.column_count).map(|x| x.to_vec()).collect()
    }

    pub fn get_column_count(&self) -> &usize {
        &self.column_count
    }

    pub fn get_columns(&self) -> Vec<Vec<f64>> {
        if self.column_count == 0 {
            return vec![];
        }

        let rows = self.get_rows();
        let mut columns = vec![vec![self.elements[0]; self.row_count]; self.column_count];

        for i in 0..self.row_count {
            for j in 0..self.column_count {
                columns[j][i] = rows[i].to_vec()[j];
            }
        }

        columns
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

    pub fn dot_product(&self, other: Matrix) -> Result<f64, ()> {
        if &self.row_count != &other.row_count || &self.column_count != &other.column_count {
            return Err(());
        }
        let mut result = 0.0;
        for i in 0..self.row_count {
            let mut vec_dot_product_result = 0.0;
            for j in 0..self.column_count {
                vec_dot_product_result += self.elements[i * self.column_count + j] * other.elements[i * self.column_count + j];
            }
            result += vec_dot_product_result;
        }
        Ok(result)
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

impl Sub for Matrix {
    type Output = Result<Matrix, ()>;
    fn sub(self, other: Self) -> Self::Output {
        if &self.row_count != &other.row_count || &self.column_count != &other.column_count {
            return Err(());
        }
        let mut elements = vec![0.0; self.elements.len()];
        for i in 0..self.elements.len() {
            elements[i] = &self.elements[i] - &other.elements[i];
        }
        Ok(Matrix { row_count: self.row_count, column_count: self.column_count, elements })
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, other: Self) {
        if &self.row_count != &other.row_count || &self.column_count != &other.column_count {
            panic!()
        }
        for i in 0..self.elements.len() {
            self.elements[i] = &self.elements[i] - &other.elements[i];
        }
    }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, scalar: f64) -> Self::Output {
        let mut elements = vec![0.0; self.elements.len()];
        for i in 0..self.elements.len() {
            elements[i] = &self.elements[i] * scalar;
        }
        Matrix { row_count: self.row_count, column_count: self.column_count, elements }
    }
}

impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, scalar: f64) {
        for i in 0..self.elements.len() {
            self.elements[i] = &self.elements[i] * scalar;
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut matlab_string = "[".to_string();
        let rows: Vec<&[f64]> = self.elements.chunks(self.column_count).collect();
        for (i, row) in rows.iter().enumerate() {
            for (inner_i, element) in row.iter().enumerate() {
                matlab_string += &format!("{:?}", element);
                if inner_i != self.column_count - 1 {
                    matlab_string += " ";
                }
            }
            if i != self.row_count - 1 {
                matlab_string += "; "
            }
        }
        matlab_string += "]";
        write!(
            f,
            "Matrix({}x{}={}) = {} ",
            self.row_count, self.column_count, self.row_count * self.column_count,
            matlab_string
        )
    }
}
