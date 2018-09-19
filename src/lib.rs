use std::collections::HashMap;
use std::hash::Hash;

// This is a workaround while we wait for https://github.com/rust-lang/rust/issues/41517 to be merged
// Copied for here: https://github.com/aatxe/markov/blob/stable/src/lib.rs#L59
trait Token: Clone + Eq + Hash {}
impl<T> Token for T where T: Clone + Eq + Hash {}

struct MarkovChain<T>
where
    T: Token,
{
    order: usize,
    graph: HashMap<Vec<T>, HashMap<T, usize>>,
}

impl<T> MarkovChain<T>
where
    T: Token,
{
    // TODO: filter on order >= 1
    fn new(order: usize) -> MarkovChain<T> {
        MarkovChain {
            order,
            graph: HashMap::new(),
        }
    }

    fn train(&mut self, tokens: impl IntoIterator<Item = T>) -> &mut Self {
        for list in (tokens.into_iter().collect::<Vec<T>>()).windows(self.order + 1) {
            let children = self
                .graph
                .entry(list[..self.order].to_vec())
                .or_insert_with(HashMap::new);
            children
                .entry(list[self.order].clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::MarkovChain;
    use std::collections::HashMap;
    use std::hash::Hash;

    fn hashmap_creator<K, V>(tuples: Vec<(K, V)>) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        let map: HashMap<K, V> = tuples.into_iter().collect();
        map
    }

    #[test]
    fn train_first_order() {
        let mut map = MarkovChain::<&str>::new(1);
        map.train("one fish two fish red fish red".split_whitespace());
        let graph = &map.graph;
        assert_eq!(
            graph.get(&vec!["one"]).unwrap(),
            &hashmap_creator(vec!(("fish", 1usize)))
        );
        // This test cares about the order. In reality, it doesn't have to matter
        assert_eq!(
            graph.get(&vec!["fish"]).unwrap(),
            &hashmap_creator(vec!(("two", 1usize), ("red", 2usize)))
        );
    }
}
