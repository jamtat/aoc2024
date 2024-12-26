use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

pub trait DjikstraState: Sized + PartialOrd + Ord + PartialEq + Eq {
    type Position: Sized + PartialEq + Eq + Hash;
    type Cost: Sized + PartialOrd + Copy;

    fn cost(&self) -> Self::Cost;
    fn position(&self) -> Self::Position;
    fn next(&self) -> Vec<Self>;
}

pub struct Djikstra<S, F>
where
    S: DjikstraState,
    F: Fn(&S) -> bool,
{
    costs: HashMap<S::Position, S::Cost>,
    heap: BinaryHeap<S>,
    is_end: F,
    min_cost: Option<S::Cost>,
}

impl<S, F> Djikstra<S, F>
where
    S: DjikstraState,
    F: Fn(&S) -> bool,
{
    pub fn new(start: S, is_end: F) -> Self {
        Self {
            costs: [(start.position(), start.cost())].into(),
            heap: [start].into(),
            is_end,
            min_cost: None,
        }
    }

    fn add_state(&mut self, state: S) {
        self.costs.insert(state.position(), state.cost());
        self.heap.push(state);
    }

    fn next_state(&mut self) -> Option<S> {
        self.heap.pop()
    }
}

impl<S, F> Iterator for Djikstra<S, F>
where
    S: DjikstraState,
    F: Fn(&S) -> bool,
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.next_state() {
            let cost = state.cost();

            if (self.is_end)(&state) {
                if let Some(min_cost) = self.min_cost {
                    return (cost == min_cost).then_some(state);
                } else {
                    self.min_cost = Some(cost);
                    return Some(state);
                }
            }

            if self
                .costs
                .get(&state.position())
                .map(|&existing_cost| cost > existing_cost)
                .unwrap_or(false)
            {
                continue;
            }

            for next in state.next() {
                let next_position = next.position();
                let next_cost = next.cost();

                if let Some(&existing_cost) = self.costs.get(&next_position) {
                    if next_cost <= existing_cost {
                        self.add_state(next);
                    }
                } else {
                    self.add_state(next);
                }
            }
        }
        None
    }
}
