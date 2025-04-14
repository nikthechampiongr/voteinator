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

                return Some((*candidate_id, group_id));
            }
        }
        None
    }

    pub(super) fn calculate_votes(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut votes: HashMap<usize, Vec<usize>> = self.generate_vote_tally_map();

        if votes.is_empty() {
            return votes;
        }

        for (id, vote) in self.votes.iter_mut() {
            while vote.peek().is_some() && !votes.contains_key(&vote.peek().unwrap()) {
                vote.pop();
            }
            let vote_pref = vote.peek();

            if vote_pref.is_none() {
                continue;
            }
            votes.entry(vote_pref.unwrap()).and_modify(|v| v.push(*id));
        }
        votes
    }

    pub(super) fn create_vote_map(
        &self,
        votes: &HashMap<usize, Vec<usize>>,
    ) -> HashMap<String, usize> {
        HashMap::from_iter(
            votes
                .iter()
                .map(|(k, v)| (self.get_name(*k).unwrap(), self.sum_votes(v))),
        )
    }
}
