# Dyck-words

This is a Rust package that computes the [Dyck words](https://en.wikipedia.org/wiki/Dyck_word) of a given semilength *n*
by finding the *n*th Catalan number and applying the algorithm from [this](https://arxiv.org/pdf/1602.06426.pdf) paper,
which is basically due to Donald Knuth. The algorithm is constant-space and constant-time, as described in the paper above.
There's a good chance this is the fastest algorithm possible, though I'm sure my implementation is not! :)

## Usage

Run:
```
cargo run --release -- [SEMILENGTH] > output.txt
```
and then do
```
less output.txt
```
to read the results. It prints the Catalan number of your semilength's order at the beginning.

## Possible Improvements

- [x] Check that this actually works as advertised, using tests...?
- [ ] Explore the possibility of using explicit multi-threading with `rayon` or `crossbeams`?
