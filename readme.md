# Script for testing safe fast math effect

Tested only Ubuntu 20 with Python3, Clang and Rust.

#### Versions

Ubuntu clang version 11.0.0-2~ubuntu20.04.1
Target: x86_64-pc-linux-gnu
Thread model: posix
InstalledDir: /usr/bin

rustc 1.51.0 (2fd73fabe 2021-03-23)

#### How to test

Tweak constants in compile.py (you need specify your CPU, linker, llvm tools, etc)
And just run `python3 compile.py`

#### What it does

It generated LLVM IR code for dot product dynamic library in Rust.
Then adds some faster and imprecise (but still _SAFE_ and __without__ UB!) flags to `fadd` and `fmul` instructions in one of implementations.
Runs LLVM optimizations then link it to dynamic library.
Then builds and run Rust project to show difference between IEEE-754 compilant and fast arithmetics.

#### Results on my PC

```
Running IEEE-754 floating math
Elapsed 177 microseconds
Result: 2666686666700000

Running fast floating math
Elapsed 40 microseconds
Result: 2666686666700000
```
