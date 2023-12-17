use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

/**
Represents a searchable world
*/
pub trait Searchable: Sized {
    type State: Debug;
    type Key: Hash + Eq;
    type Value: Ord + Debug;

    fn initial_state(&self) -> Self::State;
    fn successors(&self, state: Self::State) -> Vec<Self::State>;
    fn key(&self, state: &Self::State) -> Self::Key;
    fn value(&self, state: &Self::State) -> Self::Value;
    // must OVERESTIMATE the value of the highest value goal state that is a successor to this state
    // Ideally should be as low as possible while still being an overestimate
    fn value_estimate(&self, state: &Self::State) -> Self::Value;
    // return true if this state is an acceptable end state
    fn is_goal(&self, state: &Self::State) -> bool;

    /**
    return true if the `value_estimate` is such that the first goal state reached is guaranteed to be
    optimal.
    */
    fn break_on_goal() -> bool {
        false
    }
}

struct KeyWithItem<S: Searchable>(S::Key, S::State);

impl<S: Searchable> Hash for KeyWithItem<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<S: Searchable> PartialEq for KeyWithItem<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<S: Searchable> Eq for KeyWithItem<S> {}

pub fn search<S: Searchable>(search: &S) -> Option<(S::State, S::Value)> {
    let initial_state = search.initial_state();
    let break_on_goal = S::break_on_goal();
    let mut q = PriorityQueue::new();
    let mut seen = HashMap::new();
    {
        let key = search.key(&initial_state);
        let value_est = search.value_estimate(&initial_state);
        q.push(KeyWithItem::<S>(key, initial_state), value_est);
    }

    let mut best = None;

    while !q.is_empty() {
        let (KeyWithItem(key, s), value_est) = q.pop().unwrap();

        log::debug!("checking {s:?}, estimate {value_est:?}");

        if let Some((_, best_v)) = best.as_ref() {
            if &value_est <= best_v {
                continue;
            }
        }

        let value = search.value(&s);
        if let Some(prev_value) = seen.get(&key) {
            if prev_value >= &value {
                continue;
            }
        }

        if search.is_goal(&s) {
            // TODO assumes there are no successor states after reaching the goal
            if let Some((_, best_v)) = best.as_ref() {
                if &value > best_v {
                    best = Some((s, value));
                    continue;
                }
            } else {
                best = Some((s, value));
                if break_on_goal {
                    break;
                }
                continue;
            }
        }

        seen.insert(key, value);

        for succ in search.successors(s) {
            let succ_key = search.key(&succ);
            let succ_value_est = search.value_estimate(&succ);
            log::debug!("next {succ:?}, estimate {succ_value_est:?}");
            let mut succ_item = KeyWithItem(succ_key, succ);
            if let Some((a, old_prio)) = q.get_mut(&succ_item) {
                if &succ_value_est > old_prio {
                    std::mem::swap(a, &mut succ_item);
                } else {
                    continue;
                }
            }
            /*if let Some(old_prio) = q.get_priority(&succ_item) {
                if old_prio > &succ_value_est {
                    continue;
                }
            }
            q.remove(&succ_item);*/
            q.push(succ_item, succ_value_est);
        }
    }

    best
}

/*
If this is pathfinding, successor states have lower value than predecessors.

You can set the value_estimate to the same as value. In this case, the algo with pick the shortest
path each iter, so the first one that is a goal is the shortest (highest value). This is equiv to
BFS. By setting the value estimate lower than value (but still an overestimate) the performance will
be improved, and the optimal solution will still be found first.

However if you set the value_estimate to higher than value (i.e. the value of the initial
state), the algo changes to picking the longest path each iter. This is equiv to DFS, and will not
find an optimal path on the first time it finds a goal state. Too much higher and it will actually
find the worst paths first.
 */

/*
If this is value building, successor states have higher value than predecessors.

You must set the value_estimate to be higher than value (unless no more value can be gained in
successor states). This will always be similar to DFS, and will not necessarily find the optimal
state first.
 */
