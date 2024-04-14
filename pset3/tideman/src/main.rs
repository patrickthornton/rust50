pub mod graph;
pub mod rust50;
use election::{Ballot, Ballots, Candidates, Election};
use std::env;

mod election {
    use super::graph::{Edge, Graph, Vertex};
    use std::cmp::Ordering;

    // a variable of this type will store whether candidates are eliminated
    pub type Candidates = Vec<String>;

    // a voter's preferences, stored as a vec of indexes into the candidates vec
    pub type Ballots = Vec<Ballot>;
    pub type Ballot = Vec<usize>;

    // represents an election
    pub struct Election {
        pub candidates: Candidates,
        pub ballots: Ballots,
    }

    impl Election {
        // simulates a tideman election; presumes there will be no more than one sink
        pub fn tideman(&self) -> Option<String> {
            let graph = self.construct_graph()?;
            graph
                .source()
                .map(|Vertex(index)| self.candidates[*index].to_owned())
        }

        // in election terms: construct a graph where each candidate is a vertex
        // and each matchup is an edge; only include those edges that don't form
        // a cycle; place edges in descending order of strength (tideman algo)
        fn construct_graph(&self) -> Option<Graph<usize, usize>> {
            let mut graph = Graph {
                v: self.get_vertices(),
                e: Vec::new(),
            };
            for edge in self.get_edges() {
                if !graph.dfs(&edge.1, &edge.0) {
                    graph.e.push(edge);
                }
            }
            Some(graph)
        }

        // in election terms: get the list of candidates
        fn get_vertices(&self) -> Vec<Vertex<usize>> {
            (0..self.candidates.len()).map(Vertex).collect()
        }

        // in election terms: get the list of head-to-head matchups
        // between all pairs of candidates
        // this is currently a little ungainly; could use a second pass
        fn get_edges(&self) -> Vec<Edge<usize, usize>> {
            let mut edges: Vec<Edge<usize, usize>> = Vec::new();
            for i in 0..self.candidates.len() {
                for j in i + 1..self.candidates.len() {
                    let i_victories = self
                        .ballots
                        .iter()
                        .filter(|b| {
                            let i_index = b
                                .iter()
                                .position(|&x| x == i)
                                .expect("i_index should be in candidates' bounds");
                            let j_index = b
                                .iter()
                                .position(|&x| x == j)
                                .expect("j_index should be in candidates' bounds");
                            i_index < j_index
                        })
                        .count();
                    let j_victories = self.ballots.len() - i_victories;
                    match i_victories.cmp(&j_victories) {
                        Ordering::Greater => {
                            edges.push(Edge(Vertex(i), Vertex(j), i_victories - j_victories));
                        }
                        Ordering::Less | Ordering::Equal => {
                            edges.push(Edge(Vertex(j), Vertex(i), j_victories - i_victories));
                        }
                    }
                }
            }
            edges.sort_by_key(|e| e.2);
            edges.reverse();
            edges
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [candidate ...]", args[0]);
        return;
    }

    // hashmap from candidate names to elimination status;
    // sorted to get O(log n) lookups when reading votes
    // (i understand it's gonna be a tiny list of candidates, let me have this)
    let mut candidates: Candidates = Vec::new();
    for candidate in args.iter().skip(1) {
        candidates.push(candidate.to_lowercase());
    }
    candidates.sort();

    let mut ballots: Ballots = Vec::new();

    // read ballots from stdin
    let num_voters = rust50::get_u32("Number of voters: ")
        .expect("rust50 module should work when getting number of voters");
    let num_candidates = candidates.len();
    for _ in 1..=num_voters {
        let mut ballot: Ballot = Vec::new();
        for j in 1..=num_candidates {
            let vote = rust50::get_string(format!("Rank {}: ", j).as_str())
                .expect("rust50 module should work when getting vote")
                .to_lowercase();
            match candidates.binary_search(&vote) {
                Ok(index) => {
                    ballot.push(index);
                }
                Err(_) => {
                    println!("Invalid vote.");
                    return;
                }
            }
        }
        ballots.push(ballot);
        println!();
    }

    // simulate election
    let election = Election {
        candidates,
        ballots,
    };
    match election.tideman() {
        Some(winner) => println!("{}", winner),
        None => println!("No winner."),
    }
}

// woefully undertested for sure; shame on me
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tideman() {
        let mut test_cases = [
            (
                vec!["alice", "bob", "charlie"],
                vec![
                    vec!["alice", "bob", "charlie"],
                    vec!["alice", "charlie", "bob"],
                    vec!["bob", "charlie", "alice"],
                    vec!["bob", "alice", "charlie"],
                    vec!["charlie", "bob", "alice"],
                ],
                "bob",
            ),
            (
                vec!["alice", "bob", "charlie"],
                vec![
                    vec!["alice", "bob", "charlie"],
                    vec!["alice", "bob", "charlie"],
                    vec!["bob", "alice", "charlie"],
                    vec!["bob", "alice", "charlie"],
                    vec!["bob", "alice", "charlie"],
                    vec!["charlie", "alice", "bob"],
                    vec!["charlie", "alice", "bob"],
                    vec!["charlie", "alice", "bob"],
                    vec!["charlie", "bob", "alice"],
                ],
                "alice",
            ),
            (vec!["bilbo", "oboe"], vec![vec!["oboe", "bilbo"]], "oboe"),
            (
                vec!["alice", "bob", "charlie"],
                vec![
                    vec!["alice", "bob", "charlie"],
                    vec!["alice", "bob", "charlie"],
                    vec!["alice", "bob", "charlie"],
                    vec!["bob", "charlie", "alice"],
                    vec!["bob", "charlie", "alice"],
                    vec!["charlie", "alice", "bob"],
                    vec!["charlie", "alice", "bob"],
                    vec!["charlie", "alice", "bob"],
                    vec!["charlie", "alice", "bob"],
                ],
                "charlie",
            ),
        ];

        for (candidates_test, ballots_test, expected) in test_cases.iter_mut() {
            let mut candidates: Candidates = Vec::new();
            for candidate in candidates_test {
                candidates.push(candidate.to_string());
            }

            let ballots: Vec<Ballot> = ballots_test
                .iter()
                .map(|ballot| {
                    ballot
                        .iter()
                        .map(|s| {
                            candidates
                                .iter()
                                .position(|s2| s == s2)
                                .expect("test case vote names should be valid")
                        })
                        .collect()
                })
                .collect();

            let expected: String = expected.to_string();

            let election = Election {
                candidates,
                ballots,
            };
            match election.tideman() {
                Some(winner) => assert_eq!(winner, expected),
                None => assert_eq!(expected, "No winner."),
            }
        }
    }
}
