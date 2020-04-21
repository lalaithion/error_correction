#[derive(PartialEq, Eq, Debug, Hash)]
pub struct BitMatrix {
    rows: usize,
    columns: usize,
    data: Vec<bool>,
}

impl BitMatrix {
    pub fn shape(data: Vec<bool>, rows: usize, columns: usize) -> BitMatrix {
        if rows * columns != data.len() {
            panic!(
                "wrong size of data: {} values cannot be reshaped to a {} by {} matrix",
                data.len(),
                rows,
                columns
            );
        }

        BitMatrix {
            rows: rows,
            columns: columns,
            data: data,
        }
    }

    pub fn column_vector(data: Vec<bool>) -> BitMatrix {
        BitMatrix {
            rows: data.len(),
            columns: 1,
            data: data,
        }
    }

    pub fn row_vector(data: Vec<bool>) -> BitMatrix {
        BitMatrix {
            rows: 1,
            columns: data.len(),
            data: data,
        }
    }

    fn set(&mut self, row: usize, column: usize, value: bool) {
        debug_assert!(
            row < self.rows,
            "row out of bounds: the row length is {}, but the row index is {}",
            self.rows,
            row
        );
        debug_assert!(
            column < self.columns,
            "column out of bounds: the column length is {}, but the column index is {}",
            self.columns,
            column
        );
        self.data[self.columns * row + column] = value;
    }

    pub fn get(&self, row: usize, column: usize) -> bool {
        debug_assert!(
            row < self.rows,
            "row out of bounds: the row length is {}, but the row index is {}",
            self.rows,
            row
        );
        debug_assert!(
            column < self.columns,
            "column out of bounds: the column length is {}, but the column index is {}",
            self.columns,
            column
        );
        self.data[self.columns * row + column]
    }

    pub fn flip(&mut self, row: usize, column: usize) {
        debug_assert!(
            row < self.rows,
            "row out of bounds: the row length is {}, but the row index is {}",
            self.rows,
            row
        );
        debug_assert!(
            column < self.columns,
            "column out of bounds: the column length is {}, but the column index is {}",
            self.columns,
            column
        );
        self.data[self.columns * row + column] ^= true;
    }

    pub fn row_len(&self) -> usize {
        self.rows
    }

    pub fn column_len(&self) -> usize {
        self.columns
    }

    pub fn transpose(&self) -> BitMatrix {
        let mut data = Vec::with_capacity(self.data.len());
        for column in 0..self.column_len() {
            for row in 0..self.row_len() {
                data.push(self.get(row, column))
            }
        }
        BitMatrix::shape(data, self.columns, self.rows)
    }

    pub fn dot(&self, other: &BitMatrix) -> BitMatrix {
        debug_assert!(self.column_len() == other.row_len(), "size mismatch: a matrix with {} columns may not be multiplied to a matrix with {} rows", self.column_len(), other.row_len());

        let data = vec![false; self.row_len() * other.column_len()];
        let mut output = BitMatrix::shape(data, self.row_len(), other.column_len());
        for row in 0..self.row_len() {
            for column in 0..other.column_len() {
                let mut val = false;
                for i in 0..self.column_len() {
                    val = val ^ (self.get(row, i) & other.get(i, column))
                }
                output.set(row, column, val);
            }
        }

        return output;
    }

    pub fn format(&self) -> String {
        let mut visual = String::from("[");
        for column in 0..self.column_len() {
            if column != 0 {
                visual.push('\n');
                visual.push(' ');
            }
            for row in 0..self.row_len() {
                if row != 0 {
                    visual.push(' ');
                }
                if self.get(row, column) {
                    visual.push('1');
                } else {
                    visual.push('0');
                }
            }
        }
        visual.push(']');
        return visual;
    }

    pub fn to_vec(self) -> Vec<bool> {
        assert!(self.column_len() == 1 || self.row_len() == 1);
        return self.data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access() {
        let x = BitMatrix::shape(vec![false, true, false, true], 2, 2);
        assert_eq!(x.get(0, 0), false);
        assert_eq!(x.get(0, 1), true);
        assert_eq!(x.get(1, 0), false);
        assert_eq!(x.get(1, 1), true);
    }

    #[test]
    #[should_panic]
    fn error_access() {
        let x = BitMatrix::shape(vec![false, true, false, true], 2, 2);
        assert_eq!(x.get(2, 2), false);
    }

    #[test]
    #[should_panic]
    fn error_create() {
        let x = BitMatrix::shape(vec![false, true, false, true], 2, 3);
    }

    #[test]
    fn transpose() {
        let x = BitMatrix::column_vector(vec![false, true, false]);
        let y = x.transpose();
        assert_eq!(x.get(0, 0), y.get(0, 0));
        assert_eq!(x.get(1, 0), y.get(0, 1));
        assert_eq!(x.get(2, 0), y.get(0, 2));
    }

    #[test]
    fn matrix() {
        let x = BitMatrix::column_vector(vec![true, false, true]);
        let y = BitMatrix::column_vector(vec![true, true, false]).transpose();
        let z = x.dot(&y);
        assert_eq!(z.row_len(), 3);
        assert_eq!(z.column_len(), 3);
        assert_eq!(z.get(0, 0), true);
        assert_eq!(z.get(1, 0), false);
        assert_eq!(z.get(2, 0), true);
        assert_eq!(z.get(0, 1), true);
        assert_eq!(z.get(1, 1), false);
        assert_eq!(z.get(2, 1), true);
        assert_eq!(z.get(0, 2), false);
        assert_eq!(z.get(1, 2), false);
        assert_eq!(z.get(2, 2), false);
    }

    #[test]
    fn multiply() {
        let x = BitMatrix::shape(vec![true, false, false, true], 2, 2);
        let y = BitMatrix::column_vector(vec![true, false]);
        let z = x.dot(&y);
        assert_eq!(z.row_len(), 2);
        assert_eq!(z.column_len(), 1);
        assert_eq!(z.get(0, 0), true);
        assert_eq!(z.get(1, 0), false);
    }

    #[test]
    fn big() {
        let x = BitMatrix::shape(vec![true, false, false, false, true, false], 2, 3);
        let y = BitMatrix::column_vector(vec![true, false, true]);
        let z = x.dot(&y);
        assert_eq!(z.row_len(), 2);
        assert_eq!(z.column_len(), 1);
        assert_eq!(z.get(0, 0), true);
        assert_eq!(z.get(1, 0), false);
    }

    #[test]
    fn dot() {
        let x = BitMatrix::column_vector(vec![true, false, true]).transpose();
        let y = BitMatrix::column_vector(vec![true, false, true]);
        let z = x.dot(&y);
        assert_eq!(z.row_len(), 1);
        assert_eq!(z.column_len(), 1);
        assert_eq!(z.get(0, 0), false);
    }
}
