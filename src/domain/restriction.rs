pub struct Restriction {
    group_name: String,
    limit: usize,
    pub members: Vec<usize>,
}

impl Restriction {
    pub fn new(group_name: String, limit: usize, members: Vec<usize>) -> Self {
        assert_ne!(limit, 0, "Group limit can't be 0");

        Self {
            group_name,
            limit,
            members,
        }
    }
    pub fn group_name(&self) -> &str {
        &self.group_name
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn decrement(&mut self) {
        if self.limit() != 0 {
            self.limit -= 1;
        }
    }

    pub fn members(&self) -> &Vec<usize> {
        &self.members
    }
}
