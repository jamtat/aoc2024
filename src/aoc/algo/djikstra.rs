use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

pub trait DjikstraState: Sized + Clone + PartialOrd + Ord + PartialEq + Eq {
    type Position: Sized + Clone + PartialEq + Eq + Hash;
    type Cost: Sized + PartialOrd + Ord + Copy;

    fn cost(&self) -> Self::Cost;
    fn position(&self) -> Self::Position;
    fn next(&self) -> Vec<Self>;
}

pub struct QueueState<S>(Vec<S>)
where
    S: DjikstraState;

impl<S> QueueState<S>
where
    S: DjikstraState,
{
    pub fn cost(&self) -> S::Cost {
        self.state().cost()
    }

    pub fn position(&self) -> S::Position {
        self.state().position()
    }

    pub fn path(&self) -> Vec<S::Position> {
        self.0.iter().map(|s| s.position()).collect()
    }

    pub fn add(&self, state: S) -> Self {
        Self(self.0.iter().cloned().chain([state]).collect())
    }

    pub fn next(&self) -> Vec<Self> {
        self.state()
            .next()
            .into_iter()
            .map(|s| self.add(s))
            .collect()
    }

    pub fn state(&self) -> &S {
        self.0.last().unwrap()
    }

    pub fn history(&self) -> &[S] {
        &self.0
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
            .then_with(|| other.state().cmp(self.state()))
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
        self.state() == other.state() && self.path() == other.path()
    }
}

impl<S> Eq for QueueState<S> where S: DjikstraState {}

impl<S> From<S> for QueueState<S>
where
    S: DjikstraState,
{
    fn from(state: S) -> Self {
        Self(vec![state])
    }
}

pub struct Djikstra<S, F>
where
    S: DjikstraState,
    F: Fn(&S) -> bool,
{
    costs: HashMap<S::Position, S::Cost>,
    queue: BinaryHeap<QueueState<S>>,
    depriority: Option<BinaryHeap<QueueState<S>>>,
    is_end: F,
    min_cost: Option<S::Cost>,
    exhaustive: bool,
}

impl<S, F> Djikstra<S, F>
where
    S: DjikstraState,
    F: Fn(&S) -> bool,
{
    pub fn new<I>(starts: I, is_end: F) -> Self
    where
        I: IntoIterator<Item = S>,
    {
        // let start_state: QueueState<S> = start.into();
        let mut queue = BinaryHeap::new();
        let mut costs = HashMap::new();

        for s in starts {
            costs.insert(s.position(), s.cost());
            queue.push(s.into());
        }
        Self {
            costs,
            queue,
            depriority: Some(BinaryHeap::new()),
            is_end,
            min_cost: None,
            exhaustive: false,
        }
    }

    pub fn exhaustive<I>(starts: I, is_end: F) -> Self
    where
        I: IntoIterator<Item = S>,
    {
        let mut out = Self::new(starts, is_end);
        out.exhaustive = true;
        out
    }

    fn add_state(&mut self, state: QueueState<S>) {
        let cost = state.cost();
        if let Some(existing_cost) = self.costs.insert(state.position(), cost) {
            if cost < existing_cost || self.depriority.is_none() {
                self.queue.push(state);
            } else {
                self.depriority.as_mut().unwrap().push(state);
            }
        } else {
            self.queue.push(state);
        }
    }

    pub fn min_cost(&self) -> Option<S::Cost> {
        self.min_cost
    }

    fn next_state(&mut self) -> Option<QueueState<S>> {
        self.queue.pop()
    }

    fn existing_cost(&self, state: &QueueState<S>) -> Option<S::Cost> {
        self.costs.get(&state.position()).copied()
    }

    fn is_end(&self, state: &QueueState<S>) -> bool {
        (self.is_end)(state.state())
    }

    pub fn queue_size(&self) -> usize {
        self.queue.len()
            + match self.depriority.as_ref() {
                Some(dp) => dp.len(),
                None => 0,
            }
    }

    pub fn costs(&self) -> &HashMap<S::Position, S::Cost> {
        &self.costs
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
            // println!("Queue Size: {}", self.queue_size());
            let cost = state.cost();

            if self.is_end(&state) {
                return match self.min_cost {
                    Some(min_cost) => (self.exhaustive || cost == min_cost).then_some(state),
                    None => {
                        self.min_cost = Some(cost);
                        // Now we've found the min cost we need to take everything from
                        // the depriority bin and push it into the queue
                        self.queue.append(&mut self.depriority.take().unwrap());
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
