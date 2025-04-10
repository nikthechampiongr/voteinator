pub struct Candidate {
    interned_index: usize,
    eliminated: bool,
}

impl Candidate {
    pub fn new(interned_index: usize) -> Self {
        Candidate {
            interned_index,
            eliminated: false,
        }
    }

    pub fn eliminate(&mut self) {
        self.eliminated = true;
    }

    pub fn interned_id(&self) -> usize {
        self.interned_index
    }

    pub fn is_eliminated(&self) -> bool {
        self.eliminated
    }
}
