#[derive(Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy + Default> Matrix<T> {
    pub fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Vec<T>> {
        self.0.into_iter()
    }

    pub fn insert_row(&mut self, row: usize, val: T) {
        self.0.insert(row, vec![val; self.0[0].len()])
    }

    pub fn insert_col(&mut self, col: usize, val: T) {
        self.0.iter_mut().for_each(|row| row.insert(col, val));
    }

    pub fn rotate(&self) -> Matrix<T> {
        let rows = self.0.len();
        let cols = self.0[0].len();
        let mut rotated = vec![vec![Default::default(); rows]; cols];

        for row in 0..rows {
            for col in 0..cols {
                rotated[col][rows - 1 - row] = self.0[row][col];
            }
        }

        Matrix(rotated)
    }
}
