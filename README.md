# Advent of Code 2020 Solutions

## Setting up

The `setup` script creates a new Rust binary application and downloads
the input for the day into a file called input.txt.

The script expects 2 arguments: the day, and the session cookie:
```
./script <day-number> <cookie>
```

I like to create a `.cookie` file (added to `.gitignore`) containing the
cookie's value, so I can do something like this:
```
cat .cookie | xargs ./setup 1
```

## Running

The solutions are written in Rust. The solutions can be
run by executing `cargo run` in the day's folder.
