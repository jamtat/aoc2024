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

    fn existing_cost(&self, state: &S) -> Option<S::Cost> {
        self.costs.get(&state.position()).copied()
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

            match self.existing_cost(&state) {
                Some(existing_cost) if cost > existing_cost => continue,
                _ => {}
            }

            for next in state.next() {
                match self.existing_cost(&next) {
                    Some(existing_cost) if next.cost() <= existing_cost => self.add_state(next),
                    None => self.add_state(next),
                    _ => {}
                }
            }
        }
        None
    }
}
