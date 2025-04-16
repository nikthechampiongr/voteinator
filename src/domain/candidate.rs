pub struct Candidate {
    interned_index: usize,
    eliminated: bool,
    group: Option<usize>,
    previous_vote_powers: Vec<usize>,
}

impl Candidate {
    pub fn new(interned_index: usize) -> Self {
        Candidate {
            interned_index,
            eliminated: false,
            group: None,
            previous_vote_powers: Vec::new(),
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

    pub fn add_prev_voting_power(&mut self, power: usize) {
        self.previous_vote_powers.push(power);
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.interned_index.eq(&other.interned_index)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            return Some(std::cmp::Ordering::Equal);
        }

        for (v1, v2) in self
            .previous_vote_powers
            .iter()
            .zip(other.previous_vote_powers.iter())
        {
            match v1.cmp(v2) {
                std::cmp::Ordering::Equal => {}
                ord @ std::cmp::Ordering::Less | ord @ std::cmp::Ordering::Greater => {
                    return Some(ord);
                }
            }
        }
        None
    }
}
