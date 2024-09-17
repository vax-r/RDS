# Rust-Implemented Linux-style DataStructure
## Introduction
This crate implements multiple data structures using Rust programming language as the foundation,
wishing to demonstrate the power and the potential of Rust to serve as the infrastructure of multiple system.

## Main Features
Currently the following data structure are supported.
* Circular-Doubly Linked List

Also the corresponding algorithms are implemented
* `list_sort()`

More features will be released in the future !

## Prerequites
Before executing the program, you should have the following packages/tools
within your system
* cargo

## How to Build
After cloning the crate
```
$ git clone git@github.com:vax-r/RDS.git
```
One can simply type in the following command to run the Rust program
```
$ cargo run
```
You should be able to see the following sorted output
```
...
847137800 -> 869262287 -> 915084488 -> 916282423 -> 1093356507 -> 1188308514 -> 1214434924 -> 1236516448 -> 1270212226 -> 1270740816 -> 1276692194 -> 1415143921 -> 1453979203 -> 1456426835 -> 1478023237 -> 1501459823 -> 1525164894 -> 1556311866 -> 1697635167 -> 1722871447 -> 1728399490 -> 1759217622 -> 1836648801 -> 1850575906 -> 1898450328 -> 1939348964 -> 2090854285 -> 2132106684 -> Finished
```
Or you can build from the source and execute the executable under `./target`
```
$ cargo build
$ ./target/debug/RDS
```

### Test
Unit tests are written to make sure the implementation or refactor of
the method within `ListHead` is correct.
Hit the following command can execute the unit tests.
```
$ cargo test
```

## Reference
* [/include/linux/list.h](https://elixir.bootlin.com/linux/v6.10.10/source/include/linux/list.h)
* [/lib/list_sort.c](https://elixir.bootlin.com/linux/v6.10.10/source/lib/list_sort.c)
* [/lib/list-test.c](https://elixir.bootlin.com/linux/v6.10.10/source/lib/list-test.c)
