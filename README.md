# mmm

## Usage
Build like usual with `cargo build --release`

```
shuf -i 1-1000000 -n 1000000000 | ./mmm
mean: 500000
median: 500001
min: 1
max: 1000000
```

## Limitations
Only supports integers right now and doesn't really have error handling sorry

## Why is it mmm instead of mmmm
Well originally it was going to be mean/median/mode but I decided mode was dumb and min/max were more useful but by that time I had already created the repository
