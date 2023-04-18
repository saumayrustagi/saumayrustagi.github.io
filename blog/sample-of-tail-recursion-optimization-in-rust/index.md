title: Sample of tail recursion optimization in Rust
link: sample-of-tail-recursion-optimization-in-rust
published_date: 2023-01-08
meta_description: Showing optimization by overflowing the stack
tags: rust, optimization
___

## Breakdown

#### Tail Recursive Procedure

For our purposes, a recursive procedure whose last procedure call is itself.

#### Optimization

Compilers can detect whether a recursively defined procedure requires space with linear growth (recursive in nature) or constant space (iterative in nature) and in turn, produce a suitably optimized bytecode.

#### Rust

The Rust compiler can output both un-optimized and optimized binaries of the code compiled.

## Code

The code is a recursive definition of the Peano method to add 2 positive integers:

```rust
fn add(a: u32, b: u32) -> u32 {
    if a == 0 {
        b
    } else {
        add(a - 1, b + 1)
    }
}
```

As mentioned the last call of `add(a,b)` is `add(a-1,b+1)`, making the procedure definition tail-recursive. Now, the process can vary in 2 ways:

* The process of `add(2,2)` is recursive, (space required grows linearly)
  > `add(2,2) -> return add(1,3) -> return (return add(0,4))`  
  > `-> return (return (return 4))`
  or

* The process of `add(2,2)` is iterative. (space required is constant)
  > `add(2,2) -> add(1,3) -> add(0,4) -> return 4`

## Result

The soft stack limit set using `ulimit` is 2GB.

```text
$ cargo run
Enter 2 positive integers to be added: 
100000000
1

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
Aborted (core dumped)
```

The above code resulted in un-optimized code where the process was recursive in nature. The resulting stack overflow shows that the space required by the process was affected by the size of its inputs.

```text
$ cargo run --release
Enter 2 positive integers to be added: 
100000000
1
100000001
```

The above code resulted in optimized code since the flag `--release` was passed in, resulting in an iterative process.

*This was just a little test to check if the Rust compiler supports tail-recursion optimization. A similar comparison can be made with passing in the flags `-O0` and `-O2` when compiling such a procedure in C using `gcc`.*

**Kindly communicate improvements via [e-mail](mailto:saumay03pro@gmail.com).**
