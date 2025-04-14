use std::collections::HashMap;

use super::Context;

impl Context {
    pub(super) fn generate_vote_tally_map(&self) -> HashMap<usize, Vec<usize>> {
        let mut map = HashMap::new();

        for (key, value) in &self.candidates {
            if value.is_eliminated() {
                continue;
            }

            map.insert(*key, Vec::new());
        }

        map
    }

    pub(super) fn sum_votes(&self, votes: &[usize]) -> usize {
        let v: f64 = votes
            .iter()
            .map(|v| 1.0 * self.votes.get(v).unwrap().strength())
            .sum();
        v.floor() as usize
    }

    pub(super) fn handle_restrictions(&mut self) -> Option<(usize, usize)> {
        if let Some(group_id) = self.active_group_elimination {
            let group = self.restrictions.get_mut(group_id).unwrap();

            if group.limit() != 0 {
                return None;
            }

            for candidate_id in self.restrictions[group_id].members() {
                let member = self.candidates.get_mut(candidate_id).unwrap();

                if member.is_eliminated() {
                    continue;
                }
                member.eliminate();

                for vote in self.votes.values_mut() {
                    if let Some(id) = vote.peek() {
                        if id == *candidate_id {
                            vote.pop();
                        }
                    }
                }

                return Some((*candidate_id, group_id));
            }
        }
        None
    }
}
