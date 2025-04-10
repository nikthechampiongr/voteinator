pub struct Candidate {
    interned_index: usize,
    eliminated: bool,
    group: Option<usize>,
}

impl Candidate {
    pub fn new(interned_index: usize) -> Self {
        Candidate {
            interned_index,
            eliminated: false,
            group: None,
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

    pub fn insert_group(&mut self, group_id: usize) -> Result<(), &'static str> {
        if self.group.is_some() {
            return Err("Group already assigned");
        }
        self.group = Some(group_id);
        Ok(())
    }

    pub fn group(&self) -> Option<usize> {
        self.group
    }
}
