# Rust

Got some inspiration, improved the other rust solution:

* Read from STDIN, less code.
* Use map and collect
* Sort and limit abort if > 2020 in the 2nd part

## Part 1

```
$ rustc part1.rs && time cat ../input | ./part1
123456

real    0m0,002s
user    0m0,003s
sys     0m0,000s
```

## Part 2

```
$ rustc part2.rs && time cat ../input | ./part2
12345678

real    0m0,002s
user    0m0,002s
sys     0m0,000s
```
