## rust50

what it says on the tin; an implementation of about half of cs50's problem sets in rust. includes `hello world`, `hello, it's me`, `mario_less`, `mario_more`, `cash`, `credit`, `scrabble`, `readability`, `caesar`, `substitution`, `plurality`, `runoff`, `tideman`, `volume`, `recover`, and `speller`.

all of these solutions come with in-file tests that duplicate a sufficiently large subset of check50's functionality such that i can be confident these are all correct. [perhaps with the exception of `recover`, but that problem sets' functionality is so specific that evaluating correctness is pretty self-explanatory; i settled with seeing if it spits out the 50 jpgs from `card.raw`, which i'm happy to report it does!] note that starting with the pset2 solutions i began to use a module i created called `rust50` that duplicates the functionality of the `cs50.h` library; from then on, i don't test the stdin/stdout interfacing as much as i did for pset1's problems.

to check my work / run it yourself, clone the repo, cd into any of the problem directories (e.g. pset1/cash, pset2/readability) and run `cargo run`, or `cargo run --release` if you're in a hurry. to check the tests, run `cargo test`.

### speller

there are some implementations here that are definitely more interesting than others; `tideman`, for instance, shows off a number of cool things about rust (the `dfs()` function ended up being really pleasant to look at). `speller`, though, is the only problem set in cs50 that takes a nontrivial amount of time to execute, and so ends up being a place where rust can show off its potential for, uh, performantness.

this binary matches cs50's speller50 binary's functionality exactly, which was a bit of a pain, since speller50's precise functionality is often a consequence of its very C-ish implementation (i.e., a string like `rust50's` is 'ignored' since it includes digits, but the spellchecker stops ignoring characters when it hits the apostrophe, so the `s` after the apostrophe has to be rescued and spellchecked; weird, right?) the tests for the file just confirm that its output is identical to that of speller50.

one snag; the provided test files `xueqin2.txt` and `holmes.txt` aren't valid UTF-8! to work with them, i would've had to rewrite this to deal with byte arrays (`[u8]`) instead of strings (`String`), since rust strings are necessarily UTF-8. hence, the tests exclude them; kick up the times below by like ~20% when comparing against the times-to-spellcheck-all-files for speller50 in C.

so, extremely loose benchmarks: going to `pset5/speller` and running the following [the ``-- --nocapture`` flag just prints out each text as the program spellchecks it]

```
cargo test test_spellcheck_load_once --release -- --nocapture
```

completes successfully in `~0.91s` on my computer. this test loads the dictionary once and then fully spellchecks the 26 test files that are valid UTF-8, from `cat.txt` ("A cat is not a caterpillar.") to `shakespeare.txt` (his complete works), comparing this program's outputs against the expected outputs from speller50. running

```
cargo test test_spellcheck_load_every_time --release -- --nocapture
```

completes successfully in `~1.34s` on my computer. this test does the same things as the previous one, but reloads the dictionary for every text it spellchecks; this is more in line with how `check50` runs your code. given the best score on the big board for `speller` is something around `~5s`, i'm happy with this result!

on `nushell` on my computer, going to `pset5/speller` and running

```
timeit ./target/release/speller dictionaries/large texts/shakespeare.txt
```

returns in about `~.31s`. doing the same for speller50 on my computer after recompiling it myself;

```
timeit ./speller50 dictionaries/large texts/shakespeare.txt
```

returns in about `~1.65s`.

#### todo

- some of these implementations could benefit from a second pass, of course; an ancillary purpose of this was to learn a bit more about rust, so you can see my understanding of it develop as i go along. also, of course of course, i'm constrained for time!
- at some point i ought to publish a `rust50` crate proper!
- `pset4/filter` isn't here yet; my sincere apologies.
