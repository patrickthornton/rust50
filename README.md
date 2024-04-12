WIP: completed up to pset2, but no farther yet!

what it says on the tin; an implementation of about half of cs50's problem sets in rust.

all of these solutions come with in-file tests that duplicate a sufficiently large subset of check50's functionality such that i can be confident these are all correct. note that starting with the pset2 solutions i began to use a module i created called 'rust50' which duplicates the functionality of the 'cs50' library; from then on, i don't test the stdin/stdout interfacing as much as i did for pset1's problems.

to check my work / run it yourself, clone the repo, cd into any of the problem directories (e.g. pset1/cash, pset2/readability) and run `cargo run`. to check the tests, run `cargo test`.

at some point i will publish a 'rust50' crate proper!
