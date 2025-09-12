# Structure
Structure or, `struct` is a custom data type that allows us to store different but relevant data types as a package, similar to C++'s `struct` concept. Example:
```rust
    struct User {
        uname: String,
        email: String,
        active: bool
    }
```
The attributes inside the `struct User` are called `attributes`. The above code is just the declaration of `struct User`. To use it:
```rust
    let user_1 = User {
        uname: String::from("Neelotpal"),
        active: true,
        email: string::from("ns@gmail.com"),
    };
```
The order of the fields does not matter. The above code is an immutable initialization of `struct`. We can access the individual fields using `.`. Example: `user_1.uname`. To make the same code, mutable:
```rust
    let mut user_1 = User {
        uname: String::from("User 1"),
        active: true,
        email: string::from("user1@gmail.com"),
    };

    user_1.email = String::from("another@gmail.com");
```

A more modular, advanced method to use structures is like this:
```rust
    fn create_user(uname: String, email: String) -> User {
        User {
            active: true,
            uname: uname,
            email: email,
        }
    }
```
Another method to create the above function is to use `field init shorthand syntax`. This is only possible when the field names and the parameter names are exactly the same. Example:
```rust
    fn create_user(uname: String, email: String) -> User {
        User {
            active: true,
            uname,
            email,
        }
    }
```
If we want to create another instance with the same values as the previous ones, but with some changes we can use `update struct syntax`. Example:
```rust
    let user_2 = User {
        email: String::from("another@gmail.com"),
        ..user_1
    };
```
**Note: Using the update struct syntax is like a assigment operator, and therefore after assigning the values of `user_1` to `user_2`, `user_1` can no longer be used. (Refer Ownership Rules)**
Another type of structure or a custom data type is `Tuple Struct`. Similar to normal structures, but tuple structs do not have field names associated with the values. Only the field data type is mentioned. Each tuple struct is a custom data type on its own. Example:
```rust
    struct Rgb(f32, f32, f32);
    let a = Rgb(0.2, 20.1, 11.7);
```