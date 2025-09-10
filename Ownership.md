# Understanding Ownership

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
The above code will **move** the value of `s1` variable to `s2`, and the `s1` **variable** will be dropped by Rust and hence the error.

The variables `s1` and `s2` act as pointers to the actual value that points to the address in heap where the value is stored. The variables consist of address, length and capacity. Both the variables will point to a single address in heap which prevent memory wastage which happens in clones. When the scope of a variable or assignment ends, Rust calls `drop` function that helps prevent memory wastage.

```rust
    let s = String::from("Hello");
    s = String::from("Hey");            // The Rust will drop the original **value** of s i.e. is Hello and the new value will be assigned, preventing memory wastage
```
In the above code, the **value** will be dropped and not the variable.

- In order to not let Rust drop the `s1` variable, we can solve it by:
```rust
    fn func(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
    }

    fn main() {
        let s1 = String::from("Hello");
        let (s2, length) = func(s1);
    }
```
This can be a way to let Rust not invalidate the `s1` variable, and also copy its value to another variable `s2`.

### References
- Another way to solve the problem is by using `References`. Below is an example:
    ```rust
        fn func(a: &String) -> usize {          // Here the variable a is not a String but a reference to String value.
            a.len()
        }                                       // The scope of variable a ends here, but the value is not dropped since it is a reference to the actual value. And in Rust, references do not own values that they are pointing to. It is just a borrowed view.

        fn main() {
            let s = String::from("Hello");
            let length = func(&s);
            println!("The length of string {s} is {length}.");          // No error will be thrown, since only reference of variable s has been passed and not the variable itself.
        }
    ```

Creating a **Reference** is known as **Borrowing**.

```rust
    fn func(s: &String) {
        s.push_str(", World");
    }

    fn main() {
        let s = String::from("Hello");
        func(&s);
    }
```
The above code will throw an ownership error, because **references** are **immutable** by default.

We can use same above code by using **mutable references**. Example:
```rust
    fn func(s: &mut String) {
        s.push_str(", World");
    }

    fn main() {
        let mut s = String::from("Hello");
        func(&mut s);
    }
```
`mut` keyword allows us to modify the value the reference variable is pointing to.

**Note: In case of a mutable reference is created to point a variable, we cannot create any other mutable references to that variable.**
```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");             // This will throw an ownership error.
```
This condition is usedful to prevent race conditions - where multiple processes occur on a shared resource, and the output varies due to the execution sequence and timing. But the below given code will:
```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{r1}");
    }                                   // Since the reference r1 is dropped after this scope ends, therefore no error will be thrown in the case where we create another mutable reference r2.
    let r2 = &mut s;

    println!("{r2}");
```