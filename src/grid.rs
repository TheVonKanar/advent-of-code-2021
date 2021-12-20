#[derive(Default, Clone)]
pub struct Grid {
    pub w: usize,
    pub h: usize,
    values: Vec<Vec<isize>>,
}

impl Grid {
    pub fn new(w: usize, h: usize) -> Grid {
        Grid {
            values: vec![vec![0; w]; h],
            w: w,
            h: h,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> isize {
        return self.values[y][x];
    }

    pub fn increment(&mut self, val: isize, x: usize, y: usize) {
        self.values[y][x] += val;
    }
}
