#[derive(Clone)]
pub struct Grid<T: Clone> {
    grid: Vec<Vec<T>>,
}

impl<T: Clone> Grid<T> {
    pub fn new() -> Self {
        Self { grid: Vec::new() }
    }

    pub fn with_capacity(rows: usize) -> Self {
        Self {
            grid: Vec::with_capacity(rows),
        }
    }

    pub fn add_row(&mut self, row: Vec<T>) {
        self.grid.push(row);
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Cell<T>> {
        self.grid
            .get(row)
            .and_then(|r| r.get(col))
            .map(|val| Cell { row, col, val })
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        if let Some(row) = self.grid.get_mut(row) {
            row[col] = val;
        } else {
            panic!("Row out of bounds");
        }
    }

    pub fn row_count(&self) -> usize {
        self.grid.len()
    }

    pub fn col_count(&self) -> usize {
        self.grid.first().map(|r| r.len()).unwrap_or_default()
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = Cell<T>> {
        GridIterator {
            r: 0,
            c: 0,
            grid: self,
        }
    }
}

struct GridIterator<'a, T: Clone> {
    r: usize,
    c: usize,
    grid: &'a Grid<T>,
}

impl<'a, T: Clone> Iterator for GridIterator<'a, T> {
    type Item = Cell<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.r >= self.grid.row_count() {
            return None;
        }

        if self.c >= self.grid.col_count() {
            self.r += 1;
            self.c = 0;
        }

        let res = self.grid.get(self.r, self.c);

        self.c += 1;

        res
    }
}

impl<T: Clone> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Cell<'a, T> {
    pub row: usize,
    pub col: usize,
    pub val: &'a T,
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_iter() {
        let mut grid = Grid::new();
        grid.add_row(vec![1, 2, 3]);
        grid.add_row(vec![4, 5, 6]);
        let mut it = grid.iter_cells();
        assert_eq!(1, *it.next().unwrap().val);
        assert_eq!(2, *it.next().unwrap().val);
        assert_eq!(3, *it.next().unwrap().val);
        assert_eq!(4, *it.next().unwrap().val);
        assert_eq!(5, *it.next().unwrap().val);
        assert_eq!(6, *it.next().unwrap().val);
        assert!(it.next().is_none());
    }
}
