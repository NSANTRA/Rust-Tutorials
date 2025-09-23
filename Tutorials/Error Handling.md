# Error Handling
Rust has mainly 2 types of errors:
1. Recoverable Errors: It is handled using `Result<T, E>`.
2. Unrecoverable Errors: It is handled using `panic!` macro.

- By default, when `panic!` occurs the programs starts `unwinding`. 
