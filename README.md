# absh: A/B testing for shell scripts

```sh
$ absh \
  -a "test a" \
  -b "test b" \
  -A "warmup for a" \
  -B "warmup for b"
```

It continuously run `B`, `b`, `A`, `a`; ignores the numbers of the first iteration,
and then after third iteration it prints averages, and 95% confidence interval of B average/A average.

```
A: n=421 mean=61.181 std=1.701 se=0.083 min=57.687 max=66.103 med=61.218
B: n=421 mean=59.891 std=1.824 se=0.089 min=56.441 max=65.669 med=59.496
A: distr=[        ▁▁   ▃▃▃▅▂▅▆▃▄▇▆▃▅▂▁▂▅▃▁▄▄▆▇▅▄█▃▃▄▂▃▁▁ ▂ ▁  ▁        ]
B: distr=[   ▁▁▁▁▄▅▄▇▅▇█▂▃▅▃▅▃▁▁▂▃▃▂▅▂▃▅▆▂▅▃▅▁▁▃ ▂▁▁▁▁                 ]
B/A: 0.979 0.975..0.983 (95% conf)
```

## How to install

```sh
cargo install --git https://github.com/stepancheg/absh
```

Cargo is a Rust package manager and build system. It can be downloaded [from rustup.rs](https://rustup.rs/).
