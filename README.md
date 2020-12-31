# Advent of Code 2020, in Rust

_Elia Robyn Speer_

I had a great time using Advent of Code to level up my Rust skills from beginner to intermediate. Here, I'll highlight some of my favorite solutions.


# Day 23: Crab Cups

[The task](https://adventofcode.com/2020/day/23) / [my solution]((./blob/main/advent23/src/main.rs))

This problem started by defining a simple game of moving around 9 numbered cups in a circle, with an operation that moves 3 cups at a time to a new spot after the cup with a given number. With 9 cups, this was easy enough to implement with a double-ended queue, which I'd simply search linearly to find the place to move the cups to.

The second step, however, discourages linear search: instead of 10 moves on 9 cups, it asks you to do 10 million moves on a million cups.

It turned out that, when my code was compiled with optimizations on, it didn't take _that_ long to do the approximately 5 trillion operations that linear search required. I initally got the answer by running the same code and waiting an hour. But it was clear that I should look for a more satisfying solution.

The problem seems like it was designed to encourage making a circular linked list, plus an index of references into the linked list keeping track of where each numbered element was. This is the kind of thing that absolutely demands reference counting, and wouldn't fit at all into the statically-analyzed lifetimes that Rust data structures have by default, so I made much heavier use of `Rc` and `RefCell` than I ever had before.

I ended up with a fairly satisfying data structure I called `SpinnyList`. I was surprised to find that, even though the common pattern of reference-counted smart pointers is a type of the form `Rc<RefCell<_>>`, each cell of the linked list actually wanted to be a `RefCell<Option<Rc<Self>>>`. Once I figured out the right types, the rest of the code fell into place.

Once I had this data structuer, it was extremely satisfying to see the same million-cup monte run in under a minute and get the right answer.


# Day 17: Conway Cubes

[The task](https://adventofcode.com/2020/day/17) / [my solution]((./blob/main/advent17/src/main.rs))

This problem asks you to implement Conway's Game of Life in 3 dimensions, then in 4 dimensions. The live cells have to be stored sparsely, as a dense 4-dimensional array consumes way too much memory as it expands in every dimension.

I was able to abstract my code to require its cells to have only a trait called `HasNeighbors`. Theoretically, this would allow it to work on any graph structure, though I only needed to implement `HasNeighbors` for 3D and 4D coordinates.


# Day 24: Lobby Layout

[The task](https://adventofcode.com/2020/day/24) / [my solution]((./blob/main/advent24/src/main.rs))

Okay, actually, the generality of my solution on day 17 _did_ help! It turned out that I could reuse this code on day 24, which was another Life-like automaton, on a hex grid this time. I just needed to add some parameters to allow changing the number of neighbors required for survival or birth.

I was glad that I remembered the existence of [this article by Red Blob Games](https://www.redblobgames.com/grids/hexagons/) on hex-grid coordinate systems.

This is the only day that I ended up using unstable instead of stable Rust. Hex coordinates in this task were given as strings of undelimited directions, such as `weswnww` which breaks down into `[w, e, sw, nw, w]`.

Because each valid hex direction ends in either `w` or `e`, I found that I wanted to use the `str::split_inclusive` feature. It's not stabilized yet, but overriding my Rust environment to use unstable for just this day worked fine.


# Day 22: Crab Combat

[The task](https://adventofcode.com/2020/day/22) / [my solution]((./blob/main/advent22/src/main.rs))

Part 1 asks for an implementation of a simple game of War, and then part 2 makes it overwhelmingly, painfully recursive. Implementing the rules of both versions of the game went fine, but I was perplexed to find that my code for step 2 would loop infinitely.

I spent a while trying to debug this, and trying to find out where I was recursing or looping infinitely. Things only got more perplexing, because I _wasn't_ recursing or looping infinitely in my code. Meanwhile, the infinite loops started happening inside the debugging statements!

It turned out that this task led me directly into a soundness bug in Rust. The recently-added method `VecDeque::make_contiguous`, which allows getting a contiguous slice from a VecDeque, was introduced in Rust 1.48, but its code is incorrect. When the VecDeque falls on a particular position in the buffer, `make_contiguous` breaks its invariants. I found this [minimal example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=05fffbc27131e985d52b745672882a95) that causes the bug.

The result for me was that the VecDeque ended up overwriting the empty cell that was supposed to mark its end, and its ring buffer became an infinitely long cycle.

As I went to see if I should report the bug in Rust, I found that it was [already reported](https://github.com/rust-lang/rust/issues/80338), by someone who encountered it for the same reason! This unfortunate coder wasn't getting infinite loops, they were getting wrong values.

The bug is fixed in unstable versions of Rust, such as the current candidate for Rust 1.51.


# Day 4: Passport Processing

[The task](https://adventofcode.com/2020/day/4) / [my solution]((./blob/main/advent4/src/main.rs))

This was an extremely messy parsing task to appear so early in the Advent of Code, asking us to parse and validate an ad-hoc "passport" format with examples such as:

```
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm
```

The parser for Rust I'd heard of most is [nom](https://docs.rs/nom/6.0.1/nom/), so I learned to use it.

It was a very frustrating learning experience.

nom was designed as a parser for byte streams built using macros, but it's grown to suggest using higher-order functions instead of its (nearly-undebuggable) macros, and it's also generalized to work on `&str` data instead of arbitrary bytes.

Unfortunately, most of the documentation and examples of nom still refer to the macros-and-bytes way of doing things. By trial and error and sometimes asking for help, I eventually got the hang of it as I reused nom on days 7, 8, 14, 18, and 19.

Even so, I ended up continually irritated by the design of nom. Its design seems influenced by the Haskell parser `Parsec`, where everything is made of abstract, curried, higher-order functions, which are a very natural way to do things in Haskell because all Haskell functions are like that.

In Rust, where currying is basically not a thing and where you have to worry about the low-level details of your functions, Haskell-like patterns aren't nearly as comfy.


# Day 21: Allergen Assessment

[The task](https://adventofcode.com/2020/day/21) / [my solution]((./blob/main/advent21/src/main.rs))

On day 21, exhausted by the design of nom, I learned a different parsing tool, [pest](https://pest.rs/book/). Writing [a grammar](./blob/main/advent21/src/grammar.pest) in pest was much easier, though the ergonomics of how to turn a parse tree into a semantic value weren't as desirable.

Once the awkward ad-hoc format was parsed, what remained was a cute little logic puzzle where the constraints were easy to propagate.


# Day 19: Monster Messages

[The task](https://adventofcode.com/2020/day/19) / [my solution]((./blob/main/advent19/src/main.rs))

I spent a while on this one, because I wanted to do it right. Parsing context-free grammars is the kind of thing that's been my bread and butter for decades, but that means it had been maybe 14 years since I'd actually implemented the [Earley algorithm](https://en.wikipedia.org/wiki/Earley_parser).

I found myself wishing I had looked at the Earley code I'd worked on for Python's NLTK to refresh my memory, instead of the terribly vague pseudocode descriptions of the algorithm that people give, but instead of doing that code archaeology I eventually figured out the pseudocode.

Once I had the details right, the algorithm coped admirably with this messy context-free grammar.