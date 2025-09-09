# Ownership

## What is Ownership in Rust?
Ownership is a set of rules that allows Rust to govern how the program manages memory. The compiler checks this set of rules, if any of the rules are violated, the program will not compile.

## Ownership Rules
1. Each value in Rust has an owner.
2. There can be only one owner at a time.
3. When the owner goes out of scope, the value is dropped.

### Variable Scope
```rust
    {                      // s is not valid here, since it's not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```

### Example using String Data Type
```rust
    let s1 = "Hello";                   // This is a static allocation
    let mut s2 = String::from("Hello");     // This is a heap allocated String that is owned and can be growed (mutated).
    s2.push_str(", World");

    println!("{s}");                    // Output: Hello, World
```

### Memory And Allocation
```rust
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}", s1);                 // Will throw an error because since the value of s1 has been moved to s2, Rust will delete the s1 variable.
```

Since the `s1` variable will not be accessible, it is solved using `References`.

The variables `s1` and `s2` act as pointers to the actual value that points to the address in heap where the value is stored. The variables consist of address, length and capacity. Both the variables will point to a single address in heap which prevent memory wastage which happens in clones. When the scope of a variable or assignment ends, Rust calls `drop` function that helps prevent memory wastage.

```rust
    let s = String::from("Hello");
    s = String::from("Hey");            // The Rust will drop the original value of s i.e. is Hello and the new value will be assigned, preventing memory wastage
```