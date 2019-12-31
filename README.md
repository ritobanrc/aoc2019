# Advent of Code 2019

These are my solutions to Advent of Code 2019. https://adventofcode.com/


This repository uses [`cargo-aoc`](https://github.com/gobanos/cargo-aoc). Follow the instructions to install `cargo-aoc`, and then use `cargo aoc` to run the most recent day. Use `cargo run` to run all of them (though you should probably use `cargo run --release`, because some of the solutions are extremely slow when not in release mode). 

So far, I have only finished up to Day 22 Part 1, though I may complete the last couple days if I have time. 

I have set up unit tests for some of the days. These are just the examples provided in the problem, not my actual input, and are only present on days where I needed them. 

This year was notable because it involved the Intcode computer. Unlike many participants, I did not take the time to refactor my IntCode VM (though I may have to for day 23). As such, the code for that is present in `src/day9.rs`.

My solutions probably aren't very good. I am quite new to Rust at the time of writing this, and there are MANY things I do that are not recommened. My code is written very procedurally, and is far from idiomatic, maintainable, scalable, or readable. Please go find someone else's repository if you're interested in a good example of Rust code. 

Finally, Thank you to Eric Wastl for the wonderful experience and such high quality problems. 
