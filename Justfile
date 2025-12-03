# Build a day in debug mode
build DAY:
    cd day-{{DAY}} && \
    cargo build

# Build a day in release mode
release DAY:
    cd day-{{DAY}} && \
    cargo build --release

# Run a single script in debug mode
run DAY PART:
    cd day-{{DAY}} && \
    cargo run --bin part{{PART}}

# Measure using hyperfine
time DAY PART:
    cd day-{{DAY}} && \
    cargo build --release 2>/dev/null && \
    hyperfine --warmup 5 -N target/release/part{{PART}}

# Test a day
test DAY:
    cd day-{{DAY}} && \
    cargo test

# Test a day every time code is changed
keeptesting DAY:
    cd day-{{DAY}} && \
    bacon nextest

# Test the shared utils
testutils:
    cd aoc-utils && \
    cargo watch -x test

# Create a new day template
create DAY:
    cargo new day-{{DAY}} --vcs none
    cd day-{{DAY}}/src && \
    mkdir bin data && \
    mkdir bin/shared && \
    touch bin/shared/mod.rs && \
    touch data/sample_input.txt && \
    touch data/puzzle_input.txt && \
    touch bin/part1.rs && \
    cargo add test-case --dev && \
    cargo add nom && \
    cargo add --path ../../aoc-utils && \
    rm main.rs
    cp templates/shared.rs day-{{DAY}}/src/bin/shared/mod.rs
    cp templates/main.rs day-{{DAY}}/src/bin/part1.rs
    cp templates/main.rs day-{{DAY}}/src/bin/part2.rs

# Nuke a day
delete DAY:
    rm -rf day-{{DAY}}
