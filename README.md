### 1. To Reproduce the Issue

Compile the app in debug mode with the `decaf377/u32_backend` feature:

```bash
make debug_nostd
```

Use `Valgrind` to identify where the stack overflow occurs:

```bash
valgrind --tool=memcheck target/debug/check_decaf
```

Valgrind will provide a report showing the memory error:

```plaintext
==13072== Memcheck, a memory error detector
==13072== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==13072== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==13072== Command: target/debug/check_decaf
==13072==
==13072== Stack overflow in thread #1: can't grow stack to 0x1ffe801000
==13072== Stack overflow in thread #1: can't grow stack to 0x1ffe801000
==13072==
==13072== Process terminating with default action of signal 11 (SIGSEGV)
==13072==  Access not within mapped region at address 0x1FFE801FF8
==13072== Stack overflow in thread #1: can't grow stack to 0x1ffe801000
==13072==    at 0x15F194: decaf377::min_curve::ops::<impl core::ops::arith::Mul<&decaf377::min_curve::element::Element> for &decaf377::fields::fr::u32::wrapper::Fr>::mul (ops.rs:120)
```

### 2. To Run the Code in Release Mode

```bash
make release_nostd
```

In release mode, the application doesn't crash but runs indefinitely. Use `Valgrind` to monitor memory usage or `cargo-flamegraph` to analyze time and memory consumption:

```bash
valgrind --tool=memcheck target/debug/check_decaf
flamegraph -o check_decaf.svg -- target/release/check_decaf
```

Depending on your system, you would probably need to interrupt the execution of the application to generate the graph or report from valgrind.

### Note

I reproduced this issue on 32-GB RAM, 22-core CPU.
and 4-GB swap memory, linux
