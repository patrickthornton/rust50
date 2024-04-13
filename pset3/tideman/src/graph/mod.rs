// general-use mod for graph structures

use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct Graph<T, G>
where
    T: PartialEq + Eq + Hash,
{
    pub v: Vec<Vertex<T>>,
    pub e: Vec<Edge<T, G>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Vertex<T: PartialEq + Eq + Hash>(pub T);

// i32 is the weight of this edge
#[derive(Debug, PartialEq, Eq)]
pub struct Edge<T: PartialEq + Eq + Hash, G>(pub Vertex<T>, pub Vertex<T>, pub G);

impl<T, G> Graph<T, G>
where
    T: PartialEq + Eq + Hash,
{
    // returns true if target is in graph, false otherwise;
    // uses depth-first-search starting at start
    pub fn dfs(&self, start: &Vertex<T>, target: &Vertex<T>) -> bool {
        let mut discovered: HashSet<&Vertex<T>> = HashSet::new();
        let mut stack: Vec<&Vertex<T>> = Vec::new();

        // initialize with starting vertex
        discovered.insert(start);
        stack.push(start);

        // dfs body
        while let Some(v) = stack.pop() {
            if v == target {
                return true;
            }

            for neighbor in self.neighbors(v) {
                if discovered.insert(neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        false
    }

    // returns the neighbors of v in the graph
    // (this is where it's important that the graph is directed;
    // we don't count incoming edges as neighbors here)
    fn neighbors(&self, v: &Vertex<T>) -> Vec<&Vertex<T>> {
        self.e
            .iter()
            .filter_map(|Edge(v1, v2, _)| if *v1 == *v { Some(v2) } else { None })
            .collect()
    }

    // returns the source of the graph if it exists
    pub fn source(&self) -> Option<&Vertex<T>> {
        for v in &self.v {
            if self.e.iter().all(|Edge(_, v2, _)| *v != *v2) {
                return Some(v);
            }
        }
        None
    }
}
