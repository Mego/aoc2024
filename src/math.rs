#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Matrix {
    elems: [usize; 4],
}

impl Matrix {
    pub fn new(a: usize, b: usize, c: usize, d: usize) -> Self {
        Self {
            elems: [a, b, c, d],
        }
    }

    pub fn det(&self) -> isize {
        self.elems[0] as isize * self.elems[3] as isize
            - self.elems[1] as isize * self.elems[2] as isize
    }

    pub fn is_singular(&self) -> bool {
        self.det() == 0
    }

    pub fn solve(&self, c1: usize, c2: usize) -> Option<(usize, usize)> {
        if !self.is_singular() {
            let d = self.det();
            let x = (c1 as isize * self.elems[3] as isize) - (c2 as isize * self.elems[1] as isize);
            let y = c2 as isize * self.elems[0] as isize - c1 as isize * self.elems[2] as isize;
            if x % d == 0 && y % d == 0 {
                return Some(((x / d) as usize, (y / d) as usize));
            }
        }
        None
    }
}

impl From<[usize; 4]> for Matrix {
    fn from(value: [usize; 4]) -> Self {
        Self { elems: value }
    }
}
