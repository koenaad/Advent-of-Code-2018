# Advent of Code 2018

Advent of code: https://adventofcode.com/2018

Don't know what to expect? Check out the previous years: [2017](https://adventofcode.com/2017), [2016](https://adventofcode.com/2016), [2015](https://adventofcode.com/2015).

#### How does it work

- New puzzles are added each day at midnight EST (that is 06:00 CET)
- Solve the puzzle and post the solution to earn a star :star:,​ two stars can be earned each day
- The quicker you are the more points you get
- Document and share your frustrations, eureka's and black magic in #AoC2018

#### How to join

- Create an account / login: https://adventofcode.com/2018/auth/login
- Join the Kunlabora leaderbord [on this link](https://adventofcode.com/2018/leaderboard/private) with this code: `239979-3c68168f`
- Fork this repository and start hacking

#### Log

To improve knowledge sharing and experimentation, please document how to build and run your code. It can also be nice to keep a log here:

##### 30/11

I am planning to complete the Advent of Code using [Rust](https://www.rust-lang.org/). Rust is still fairly new for me, so we'll see how this goes...

To run my code:
- install Rust toolchain: https://www.rust-lang.org/install.html
- make sure `cargo` is in your path
- `cd` into directory
- compile and run the program: `$ cargo run`
- run the tests: `$ cargo test`

##### 01/12

This was a fairly easy challenge to start with: mostly reading files, iterating over lists...

I learned some more about error handling (though I kept it simple) and command line input.

Refactored the code using iterators and closures.

##### 02/12

Today I learned about all types of iterators over strings. It was messy.

##### 03/12 - 04/12 (Day 03)

I had a lot of problems getting started with this puzzle: I had never done any regex-like parsing on Rust, so I had to learn this from scratch. I ended up using [`nom`](https://crates.io/crates/nom), a streaming parser. `nom` is pretty cool, but also kind of complicated. So it took a while to simply parse a claim...

Also, tests are good.

##### 04/12 - 05/12 (Day 04)

This challenge was tough. Spent way too long getting the parsing good. Then spent way too long finding ways to convert these events into duration data.

Using existing libraries / packages is good.

##### 05/12 (Day 05)

This challenge was a lot easier than Day 4 (or maybe I got a lot smarter?). Simply had to write an algorithm to apply the reactions to the polymer and do some more iterating over various combinations.

Execution time is now about 7 seconds (with `--release`), I wonder if this could be further optimized...

##### 06/12 - 07/12 (Day 06)

This challenge was mostly difficult to wrap your head around. I ended up assuming I could discard all areas which leave the convex hull defined by the given coordinates. This allowed me to create a kind of brute force algorithm since the grid is then limited in size (and brute forcing isn't an issue in Rust). There might be more elegant solutions out there...

##### 07/12 - 08/12 (Day 07)

This was one difficult to get started, but once I started thinking in steps to do, steps in progress and steps done it clicked.

##### 08/12 (Day 08)

This was fun. Recursive parsing in `nom` is nice.

##### 09/12 - 10/12 (Day 09)

Ran into massive performance issues here. Finding the solution to puzzle 2 took 2 hours 18 minutes... Should probably spend some time implementing a linked list in Rust.

##### 10/12 (Day 10)

Had to make a small cli. Was interesting.

##### 11/12 (Day 11)

Learned a lot about making libraries and using generic types. Also `rayon` is the real MVP, instant parallel iterations. Reduces calculation time of puzzle 2 from 89 seconds to 16 seconds!

##### 14/12 (Day 14)

Mostly had difficulty understanding what was asked here...

##### 15/12 (Day 12)

Found a solution, but it doesn't scale well over a couple of 1000's of generations. Tried to parallelize the pattern matching, but that was obviously not sufficient. There is probably another solution...

##### 16/12 (Day 13)

This was hard to get structured. It works, but my domain is spread all over. Should probably spend some time refactoring this... some day.
