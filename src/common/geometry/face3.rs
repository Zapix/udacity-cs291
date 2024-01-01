pub struct Face3 {
    p1: u16,
    p2: u16,
    p3: u16,
}

impl Face3 {
    pub fn new(p1: u16, p2: u16, p3: u16) -> Self {
        Self { p1, p2, p3 }
    }

    pub fn as_array(&self) -> [u16; 3] {
        return [self.p1, self.p2, self.p3]
    }
}