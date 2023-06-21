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
}
