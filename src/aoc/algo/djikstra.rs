use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

pub trait DjikstraState: Sized + PartialOrd + Ord + PartialEq + Eq {
    type Position: Sized + Clone + PartialEq + Eq + Hash;
    type Cost: Sized + PartialOrd + Ord + Copy;

    fn cost(&self) -> Self::Cost;
    fn position(&self) -> Self::Position;
    fn next(&self) -> Vec<Self>;
}

pub struct QueueState<S>
where
    S: DjikstraState,
{
    state: S,
    path: Vec<S::Position>,
}

impl<S> QueueState<S>
where
    S: DjikstraState,
{
    pub fn cost(&self) -> S::Cost {
        self.state.cost()
    }

    pub fn position(&self) -> S::Position {
        self.state.position()
    }

    pub fn path(&self) -> &[S::Position] {
        &self.path
    }

    pub fn add(&self, state: S) -> Self {
        let p = state.position();
        Self {
            state,
            path: self.path.iter().cloned().chain([p]).collect(),
        }
    }

    pub fn next(&self) -> Vec<Self> {
        self.state.next().into_iter().map(|s| self.add(s)).collect()
    }
}

impl<S> Ord for QueueState<S>
where
    S: DjikstraState,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost()
            .cmp(&self.cost())
            .then_with(|| other.state.cmp(&self.state))
    }
}

impl<S> PartialOrd for QueueState<S>
where
    S: DjikstraState,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S> PartialEq for QueueState<S>
where
    S: DjikstraState,
{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.path == other.path
    }
}

impl<S> Eq for QueueState<S> where S: DjikstraState {}

impl<S> From<S> for QueueState<S>
where
    S: DjikstraState,
{
    fn from(state: S) -> Self {
        let pos = state.position();
        Self {
            state,
            path: vec![pos],
        }
    }
}

pub struct Djikstra<S, F>
where
    S: DjikstraState,
    F: Fn(&S) -> bool,
{
    costs: HashMap<S::Position, S::Cost>,
    queue: BinaryHeap<QueueState<S>>,
    is_end: F,
    min_cost: Option<S::Cost>,
}

impl<S, F> Djikstra<S, F>
where
    S: DjikstraState,
    F: Fn(&S) -> bool,
{
    pub fn new(start: S, is_end: F) -> Self {
        let start_state: QueueState<S> = start.into();
        Self {
            costs: [(start_state.position(), start_state.cost())].into(),
            queue: [start_state].into(),
            is_end,
            min_cost: None,
        }
    }

    fn add_state(&mut self, state: QueueState<S>) {
        self.costs.insert(state.position(), state.cost());
        self.queue.push(state);
    }

    fn next_state(&mut self) -> Option<QueueState<S>> {
        self.queue.pop()
    }

    fn existing_cost(&self, state: &QueueState<S>) -> Option<S::Cost> {
        self.costs.get(&state.position()).copied()
    }

    fn is_end(&self, state: &QueueState<S>) -> bool {
        (self.is_end)(&state.state)
    }
}

impl<S, F> Iterator for Djikstra<S, F>
where
    S: DjikstraState,
    F: Fn(&S) -> bool,
{
    type Item = QueueState<S>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.next_state() {
            let cost = state.cost();

            if self.is_end(&state) {
                return match self.min_cost {
                    Some(min_cost) => (cost == min_cost).then_some(state),
                    None => {
                        self.min_cost = Some(cost);
                        Some(state)
                    }
                };
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
