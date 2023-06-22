use std::{ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign}, fmt::Display};

#[derive(Debug)]
pub struct Matrix {
    row_count: usize,
    column_count: usize,
    elements: Vec<f64>,
}

impl Matrix {
    pub fn new(row_count: usize, column_count: usize, elements: Vec<f64>) -> Result<Matrix, ()> {
        if row_count * column_count != elements.len() {
            return Err(());
        }
        Ok(Matrix { row_count, column_count, elements })
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

    pub fn cross_product(&self, other: Matrix) -> Result<Matrix, ()> {
        if self.column_count != other.row_count {
            return Err(());
        }
        let mut elements = vec![0.0; self.row_count * other.column_count];
        let lhs_rows = self.get_rows();
        let rhs_columns = other.get_columns();
        for (row_index, row) in lhs_rows.iter().enumerate() {
            for (column_index, column) in rhs_columns.iter().enumerate() {
                let mut element = f64::default();
                for i in 0..other.row_count {
                    element += row[i] * column[i];
                }
                elements[row_index * other.column_count + column_index] = element;
            }
        }

        Ok(Matrix { row_count: self.row_count, column_count: other.column_count, elements })
    }

    pub fn transpose(&self) -> Matrix {
        Matrix { row_count: self.column_count, column_count: self.row_count, elements: self.get_columns().concat() }
    }

    pub fn is_zero_matrix(&self) -> bool {
        self.elements.iter().all(|&element| element == 0.0)
    }

    pub fn is_identity_matrix(&self) -> bool {
        if !self.is_square_matrix() {
            return false;
        }
        let rows = self.get_rows();
        for i in 0..self.row_count {
            for j in 0..self.column_count {
                if i == j && rows[i][i] != 1.0 {
                    return false;
                } else if i != j && rows[i][j] != 0.0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_square_matrix(&self) -> bool {
        self.row_count == self.column_count
    }

    pub fn is_diagonal_matrix(&self) -> bool {
        //? if !self.is_square_matrix() {
        //?     return false;
        //? }
        let rows = self.get_rows();
        for i in 0..self.row_count {
            for j in 0..self.column_count {
                if i != j && rows[i][j] != 0.0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_anti_diagonal_matrix(&self) -> bool {
        //? if !self.is_square_matrix() {
        //?     return false;
        //? }
        let rows = self.get_rows();
        for i in 0..self.row_count {
            for j in 0..self.column_count {
                dbg!(self.column_count - i, j, rows[i][j]);
                if self.column_count - i != j + 1 && rows[i][j] != 0.0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_tridiagonal_matrix(&self) -> bool {
        //? if !self.is_square_matrix() {
        //?     return false;
        //? }
        let rows = self.get_rows();
        for i in 0..self.row_count {
            for j in 0..self.column_count {
                if i == j || i + 1 == j || j + 1 == i { continue; }
                if rows[i][j] != 0.0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_upper_triangular_matrix(&self) -> bool {
        if !self.is_square_matrix() {
            return false;
        }
        let rows = self.get_rows();
        for j in 0..self.row_count {
            for i in 0..self.column_count {
                if i <= j { continue; }
                if rows[i][j] != 0.0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_lower_triangular_matrix(&self) -> bool {
        if !self.is_square_matrix() {
            return false;
        }
        let rows = self.get_rows();
        for i in 0..self.row_count {
            for j in 0..self.column_count {
                if i >= j { continue; }
                if rows[i][j] != 0.0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_symmetric_matrix(&self) -> bool {
        self.is_square_matrix() && self.transpose().elements == self.elements
    }

    pub fn is_persymmetric_matrix(&self) -> bool {
        if !self.is_square_matrix() {
            return false;
        }
        let rows = self.get_rows();
        for i in 0..self.row_count {
            for j in 0..self.column_count {
                if i > j || self.column_count - i >= j + 1 { continue; }
                if rows[i][j] != rows[self.row_count - j - 1][self.column_count - i - 1] {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_bisymmetric_matrix(&self) -> bool {
        self.is_square_matrix() && self.is_symmetric_matrix() && self.is_persymmetric_matrix()
    }

    pub fn is_centrosymmetric_matrix(&self) -> bool {
        if !self.is_square_matrix() {
            return false;
        }
        let rows = self.get_rows();
        for i in 0..self.row_count {
            for j in 0..self.column_count {
                if i > j || self.column_count - i > j + 1 { continue; }
                if rows[i][j] != rows[self.row_count - i - 1][self.column_count - j - 1] {
                    return false;
                }
            }
        }
        true
    }
    
    pub fn is_skew_symmetric_matrix(&self) -> bool {
        if !self.is_square_matrix() {
            return false;
        }
        let rows = self.get_rows();
        for i in 0..self.row_count {
            for j in 0..self.column_count {
                if i > j { continue; }
                if i == j && rows[i][i] != 0.0 {
                    return false;
                }
                if rows[i][j] != -1.0 * rows[j][i] {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_row_matrix(&self) -> bool {
        self.row_count == 1 && self.column_count != 0
    }

    pub fn is_column_matrix(&self) -> bool {
        self.column_count == 1 && self.row_count != 0
    }

    pub fn determinant(&self) -> Result<f64, ()> {
        if !self.is_square_matrix() {
            return Err(());
        }
        match Some(self.elements.len()) {
            Some(1) => {
                // this is math
                return Ok(self.elements[0]);
            },
            Some(4) => {
                // not bad
                return Ok(self.elements[0] * self.elements[3] - self.elements[1] * self.elements[2]);
            },
            Some(9) => {
                // rule of sarrus
                let a = self.elements[0];
                let b = self.elements[1];
                let c = self.elements[2];
                let d = self.elements[3];
                let e = self.elements[4];
                let f = self.elements[5];
                let g = self.elements[6];
                let h = self.elements[7];
                let i = self.elements[8];
                return Ok(a*e*i + b*f*g + c*d*h - (c*e*g + b*d*i + a*f*h));
            },
            Some(_) => {
                // CHAOS
                // This algorithm uses the logic of the Lapalace Expansion (a.k.a Cofactor Expansion) method.
                // It is quite inefficient way for calculating determinant,
                // algorithms using decompositions generally better.
                let mut det = 0.0;
                const DEFAULT_ROW_INDEX: usize = 0;
                let rows = self.get_rows();

                for (index, element) in rows[DEFAULT_ROW_INDEX].iter().enumerate() {
                    let mut new_rows: Vec<Vec<f64>> = self.get_rows();
                    new_rows.remove(DEFAULT_ROW_INDEX);

                    for (_, row) in new_rows.iter_mut().enumerate() {
                        row.remove(index);
                    }

                    let sub_matrix = Matrix {
                        row_count: self.row_count - 1,
                        column_count: self.column_count - 1,
                        elements: new_rows.concat(),
                    };

                    let multiple = match index % 2 {
                        0 => 1.0,
                        _ => -1.0,
                    };

                    det += multiple * sub_matrix.determinant().unwrap() * element;
                }

                return Ok(det);
            },
            None => panic!("ftw"),
        }
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

impl Mul for Matrix {
    type Output = Result<Matrix, ()>;
    fn mul(self, other: Matrix) -> Self::Output {
        self.cross_product(other)
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
