pub mod rust50;
use std::collections::HashMap;
use std::env;

type Election = HashMap<String, u32>;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [candidate ...]", args[0]);
        return;
    }

    // hashmap from candidate names to number of votes
    let mut election: Election = HashMap::new();
    for candidate in args.iter().skip(1) {
        election
            .entry(candidate.to_owned().to_lowercase())
            .or_insert(0);
    }

    // read votes from stdin
    let num_voters = rust50::get_u32("Number of voters: ")
        .expect("rust50 module should work when getting number of voters");
    for _ in 1..=num_voters {
        let vote = rust50::get_string("Vote: ")
            .expect("rust50 module should work when getting vote")
            .to_lowercase();
        match election.get(&vote) {
            Some(votes) => {
                election.insert(vote, votes + 1);
            }
            None => {
                println!("Invalid vote.");
            }
        }
    }

    // simulate election
    let winners = plurality(election);
    for winner in winners {
        println!("{}", winner);
    }
}

fn plurality(election: Election) -> Vec<String> {
    let max_votes = election
        .values()
        .max()
        .expect("there should be at least one candidate");
    election
        .iter()
        .filter_map(|(candidate, votes)| {
            if votes == max_votes {
                Some(candidate.to_owned())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plurality() {
        let mut test_cases = [
            (
                vec![("alice", 1), ("bob", 1), ("charlie", 1)],
                vec!["alice", "bob", "charlie"],
            ),
            (
                vec![("alice", 1), ("bob", 2), ("charlie", 1), ("david", 0)],
                vec!["bob"],
            ),
            (
                vec![("alice", 0), ("bob", 199), ("charlie", 200)],
                vec!["charlie"],
            ),
        ];

        for (input, expected) in test_cases.iter_mut() {
            let mut election: Election = Election::new();
            for (candidate, votes) in input.iter() {
                election.entry(candidate.to_string()).or_insert(*votes);
            }
            let mut winners = plurality(election);
            let mut expected: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
            winners.sort();
            expected.sort();
            assert_eq!(winners, expected);
        }
    }
}
