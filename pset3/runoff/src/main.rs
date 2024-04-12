pub mod rust50;
use std::collections::HashMap;
use std::env;

// note on a space/time dilemma:
// the space-efficient way would be to store Candidates as a Vec<(String, bool)>
// and have Ballot be a Vec<usize> where each usize is an index into the candidates vec;
// but this means searching through the candidates on each vote would be in O(n).
// the time-efficient way would be to store Candidates as a HashMap<String, bool>
// and have Ballot be a Vec<String> where each String is the name of a candidate,
// which costs more space, but allows for O(1) lookups on each vote.
// both are no biggie in this case, but we'll go with the time-efficient way,
// to vary it up from the cs50 implementation.

// a variable of this type will store whether candidates are eliminated
type Candidates = HashMap<String, bool>;

// a voter's preferences, stored as a vec of the names of the candidates
type Ballot = Vec<String>;
type Ballots = Vec<Ballot>;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [candidate ...]", args[0]);
        return;
    }

    // hashmap from candidate names to elimination status
    let mut candidates: Candidates = HashMap::new();
    for candidate in args.iter().skip(1) {
        candidates
            .entry(candidate.to_owned().to_lowercase())
            .or_insert(false);
    }

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
            match candidates.get(&vote) {
                Some(_) => {
                    ballot.push(vote);
                }
                None => {
                    println!("Invalid vote.");
                    return;
                }
            }
        }
        ballots.push(ballot);
        println!();
    }

    // simulate election
    let winners = runoff(candidates, ballots);
    for winner in winners {
        println!("{}", winner);
    }
}

fn runoff(mut candidates: Candidates, ballots: Ballots) -> Vec<String> {
    // type for hashmap from candidate names to number of votes
    type Election = HashMap<String, u32>;

    let mut winners: Vec<String>;
    loop {
        let mut election: Election = HashMap::new();

        // tally up the votes
        for ballot in &ballots {
            for preference in ballot {
                if !candidates[preference] {
                    election
                        .entry(preference.to_string())
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                    break;
                }
            }
        }

        // return all those candidates with a majority of the vote;
        let num_voters = ballots.len() as u32;
        winners = election
            .iter()
            .filter_map(|(candidate, votes)| {
                if *votes >= (num_voters + num_voters % 2) / 2 {
                    return Some(candidate.to_string());
                }
                None
            })
            .collect();
        if !winners.is_empty() {
            break;
        }

        // if there are none, eliminate the candidate(s) with the fewest votes
        let min_voters = election
            .values()
            .min()
            .expect("there should be at least one candidate");
        for (candidate, votes) in &election {
            if votes == min_voters {
                candidates.insert(candidate.to_string(), true);
            }
        }
    }
    winners
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runoff() {
        let mut test_cases = [
            (
                vec!["alice", "bob", "charlie"],
                vec![
                    vec!["alice", "bob", "charlie"],
                    vec!["alice", "charlie", "bob"],
                    vec!["bob", "charlie", "alice"],
                    vec!["bob", "alice", "charlie"],
                    vec!["charlie", "alice", "bob"],
                ],
                vec!["alice"],
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
                    vec!["charlie", "bob", "alice"],
                    vec!["charlie", "bob", "alice"],
                ],
                vec!["bob"],
            ),
            (
                vec!["Schmumphert", "Gilliamsville"],
                vec![
                    vec!["Gilliamsville", "Schmumphert"],
                    vec!["Gilliamsville", "Schmumphert"],
                    vec!["Schmumphert", "Gilliamsville"],
                    vec!["Schmumphert", "Gilliamsville"],
                ],
                vec!["Schmumphert", "Gilliamsville"],
            ),
        ];

        for (candidates_test, ballots_test, expected) in test_cases.iter_mut() {
            let mut candidates: Candidates = HashMap::new();
            for candidate in candidates_test {
                candidates.entry(candidate.to_string()).or_insert(false);
            }

            let ballots: Vec<Ballot> = ballots_test
                .iter()
                .map(|ballot| ballot.iter().map(|s| s.to_string()).collect())
                .collect();

            let mut expected: Vec<String> = expected.iter().map(|s| s.to_string()).collect();

            let mut winners = runoff(candidates, ballots);
            winners.sort();
            expected.sort();
            assert_eq!(winners, expected);
        }
    }
}
