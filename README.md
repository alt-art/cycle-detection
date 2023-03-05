# Cycle detection in linked lists

This is a simple implementation of a cycle detection algorithm for linked lists. It uses the Floyd's cycle-finding algorithm.

## Running the typescript version

```bash
yarn install
yarn start
```

## Running the rust version

```bash
cd rust-version
cargo run
```

Initialy I wrote the rust version, but I realized that the rust version will be much more verbose than the typescript version. So I decided to write the typescript version to see how I could make the rust version more concise.

## Reference

- [Floyd's cycle-finding algorithm](https://en.wikipedia.org/wiki/Cycle_detection#Tortoise_and_hare)
