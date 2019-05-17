# mmm

## Usage
Build like usual with `cargo build --release`

```
shuf -i 1-1000000 -n 1000000000 | ./mmm
mean: 500000.5
median: 500001.0
min: 1.0
max: 1000000.0
sum: 500000500000.0

```

## Limitations
None, this program is perfect as long as you pass it valid input
