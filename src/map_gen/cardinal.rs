#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cardinal {
    U,
    R,
    D,
    L,
}

impl Cardinal {
    pub fn inverse(&self) -> Cardinal {
        match self {
            Cardinal::U => Cardinal::D,
            Cardinal::R => Cardinal::L,
            Cardinal::D => Cardinal::U,
            Cardinal::L => Cardinal::R,
        }
    }
    pub fn get_char_eq(&self) -> char {
        match self {
            Cardinal::U => {'↑'}, 
            Cardinal::R => {'→'},
            Cardinal::D => {'↓'},
            Cardinal::L => {'←'},
        }
    }

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Cardinal::U => (0, -1),
            Cardinal::R => (1, 0),
            Cardinal::D => (0, 1),
            Cardinal::L => (-1, 0),
        }
    }
    pub fn delta_from(&self, coord: (i32,i32)) -> (i32, i32) {
        let (dx, dy) = self.delta();
        (coord.0+dx, coord.1+dy)
    }

    pub fn iter_all() -> Vec<Cardinal> {
        let mut cardinals: Vec<Cardinal> = Vec::new();

        cardinals.push(Cardinal::U);
        cardinals.push(Cardinal::R);
        cardinals.push(Cardinal::D);
        cardinals.push(Cardinal::L);

        return cardinals;
    }
}

