---
title:
    - Rust Presentation
author:
    - Marcel Guzik
theme:
    - Frankfurt
colortheme:
    - beaver
fontsize: 9pt

pandoc-latex-fontsize:
    - classes: [rust]
      size: scriptsize
    - classes: [plain]
      size: scriptsize
---

# Why Rust

-   **Both safe and performant. No tradeoffs.**

-   Zero cost abstractions!

    Really?

    [Really!](https://pkolaczk.github.io/overhead-of-optional/)

-   Both low-level and high-level

    Write mostly high-level code, go low-level when you need it!

---

-   Memory safety

    Eliminate entire classes of bugs at compile time! You **can't** corrupt memory when using safe Rust!

    ![](img/memorysafety.jpg)

---

-   Good tooling and helpful compiler

    ![](img/it_compiles_it_works.png){ width=50% }

---

Rust's focus on safety famously makes implementing classical data structures
more difficult, eg. writing a linked list is challenging for a beginner.

https://rust-unofficial.github.io/too-many-lists/

## Borrow checking

### Move semantics

Move semantics in Rust are better than in C++. Why?

https://www.thecodedmessage.com/posts/cpp-move/

Short version:

In Rust, if the object is moved, it can't be accessed anymore

In C++, the moved object is still accessible, but is "empty", you need to
explicitly handle that case in the destructor, therefore move semantics are not
zero cost

## Rich type system

-   Algebraic Data Types
-   Generics
-   Traits

### Traits

![traits](img/traits.jpg){ width=50% }

---

Traits are like interfaces from Java or Go, but better.

-   https://softwareengineering.stackexchange.com/questions/247298/how-are-rust-traits-different-from-go-interfaces#247313
-   https://stackoverflow.com/questions/69477460/is-rust-trait-the-same-as-java-interface

---

In short:

-   it gives you a choice between static and dynamic dispatch (static dispatch
    means bigger code size but faster generics)

    ```rust
    // fast, bigger code size
    fn static_dispatch<T: MyTrait>(arg: T) { }

    // slow, less code size, uses Vtable
    fn dynamic_dispatch(arg: Box<dyn MyTrait>) { }
    ```

---

-   object definition / method implementation is decoupled (you implement in
    impl blocks)

    ```rust
    #[derive(Debug, Clone, Copy)]
    struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    impl Add for Vec3 {
        type Output = Vec3;

        fn add(self, rhs: Self) -> Self::Output {
            Vec3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl Add<f32> for Vec3 {
        type Output = Vec3;

        fn add(self, rhs: f32) -> Self::Output {
            Vec3 {
                x: self.x + rhs,
                y: self.y + rhs,
                z: self.z + rhs,
            }
        }
    }
    ```

---

-   you can conditionally implement a trait for a type

    ```rust
    impl<T> Clone for Vec<T> where T: Clone {...}
    ```

---

-   associated types, fuctions, values

    ```rust
    trait Iterator {
        type Item;
    }

    struct Iter<T>;
    impl Iterator for Iter<T> {
        type Item = &T;
    }

    struct IterMut<T>;
    impl Iterator for IterMut<T> {
        type Item = &mut T;
    }

    struct IntoIter<T>;
    impl Iterator for IntoIter<T> {
        type Item = T;
    }
    ```

---

#### Most important standard library traits:

-   Debug: Debug print formatting
-   Copy (requires Clone): Types that can be implicitly and trivially copied via
    bitwise copy
-   Clone: Types that can be explicitly cloned by calling `.clone()` on them.
-   Send: The type can be safely sent between threads
-   Sync: The type can be safely accessed via references from different threads.
    If `&T` is `Send`, then `Sync` is derived automatically

---

Comparing values:

-   PartialEq: For types that have partial equality
-   Eq: For types that have full equality
-   PartialOrd: For types with partial ordering (type can be compared if its
    less, greater, or equal)
-   Ord: For types with total ordering (can be sorted)

---

Also:

-   `Sized`: The size of this type is known at compile time. If the type has
    known size, it can be used as fields in structs or placed on the stack.
    `?Sized` (maybe sized) means size of type is not known at compile time.

    Unlike previous traits, this is assumed for all types, and only unsized
    types implement `!Sized` (not sized). Example:

    -   https://doc.rust-lang.org/stable/std/primitive.slice.html#impl-Sized
    -   https://doc.rust-lang.org/stable/std/primitive.str.html#impl-Sized

    Slices and string slices are not `Sized`, but the references to them are.

---

**[We use rich type systems to design APIs that are flexible and simple, but
most importantly,
correct.](https://fasterthanli.me/articles/aiming-for-correctness-with-types)**

## Algebraic data types

What is algebraic data type?

> In computer programming, especially functional programming and type theory, an
> algebraic data type is a kind of composite type, i.e., a type formed by
> combining other types.

We can combine types in two ways:

-   Sum types
-   Product types

In other languages, structs/classes are like a product type, but there is no
proper sum type.

---

In Rust, enums are sum types. Enums can contain values.

Example: standard library `Option`/`Result` types.

```rust
enum Option<T> {
    Some(T),
    None
}
```

```rust
let some_int: Option<i32> = Some(5);
let no_int: Option<i32> = None;
```

```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

```rust
// Returns string on success. Returns error code on failure.
fn op_that_can_fail -> Result<String, i32> {
    // ...
}

let result = op_that_can_fail();

match result {
    Ok(text) => println!("success: {text}"),
    Err(err_code) => println!("error! code: {err_code}")
}
```

---

It is impossible to not error check in Rust, because you need to handle the
error to access the success value:

```rust
let text: String = std::fs::read_to_string("file.txt");
println!("{text}");
```

```plain
error[E0308]: mismatched types
 --> examples/result.rs:2:24
  |
2 |     let text: String = std::fs::read_to_string("file.txt");
  |               ------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `String`, found enum `Result`
  |               |
  |               expected due to this
  |
  = note: expected struct `String`
               found enum `Result<String, std::io::Error>`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rust-demo` due to previous error
```

---

To unwrap the value on success, but exit the program on failure, use `.unwrap()`
or `.expect("your message")`.

```rust
let text: String = std::fs::read_to_string("file.txt").unwrap();
println!("{text}");
```

On failure (eg. when `file.txt` does not exist):

```plain
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', examples/result.rs:2:60
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Generics

Trait based generics

## Fixing a billion dollar mistake

What do we usually use null pointers for in other languages?

-   to allocate data on the heap
-   to signify the presence/absence of a value

These separate concerns are coupled, so it's not possible to express in the type
system:

-   An optional value that's on the heap
-   A heap-allocated value that's non-optional, always valid

---

**That's why Rust doesn't have null.**

Short version: for optional values, we use `Option<T>`, for heap allocation, we
use `Box<T>`. If we want an optinal heap-allocated value, use `Option<Box<T>>`,
[which is optimized to use only as much memory as
`Option<T>`](https://doc.rust-lang.org/std/option/#representation).

## Owned vs borrowed types

![strings](img/strings.jpg)

"Wow, what a mess, it's too complex! Better to use language like Go, which is
simpler."

**You can't eliminate complexity. If it's not exposed, it's hidden and may have
unexpected consequences.**

-   https://fasterthanli.me/articles/i-want-off-mr-golangs-wild-ride
-   https://viralinstruction.com/posts/defense/

also lol no generics

---

How to make sense of this?

It's a common pattern that types in Rust are divided into "owned" types and
"borrowed" types.

---

Owned types:

-   `String` - Owned, Rust native, UTF-8 encoded, explicitly sized string
-   `CString` - Owned C-compatible null-terminated string
-   `OsString` - Owned, platform-native strings (so on Unix UTF-8, on Windows
    UTF-16, etc.)
-   `PathBuf` - Wrapper around `OsString`, with logic to manage path according
    to the platform (so on Unix separator is `/`, on Windows it's `\`, etc.)
-   `Vec<u8>` - Owned vector of unsigned bytes

---

Borrowed types:

-   `&str`
-   `&' static str`
-   `CStr`
-   `OsStr`
-   `Path`
-   `&[u8]`
-   `&[u8; N]`
-   `&u8`

---

"But you told me borrowing in Rust is done with `&`, so why do some don't have
that? Also, if to borrow we just add `&`, then why is borrowed string not just
`&String`? What's the difference?"

### Strings

To show the difference we'll look into just `&str` and `String`.

First, like any respectable programmer, let's turn for help to Stack Overflow:

https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str

![String vs str](img/string_str.png)

---

`String`:

-   Mutable
-   Manages memory
-   Heap-allocated

`&str`:

-   Immutable, "view" of the string
-   A reference to memory managed by somebody else - Is a "slice" so it can
    point to any portion of the string
-   Can be on heap, on stack, static, etc.

---

How they look on the inside?

In _Rust pseudo-code_:

String:

```rust
struct String {
    data: *mut u8,      // pointer to heap allocated data - 8 bytes
    length: usize,      // length of the string - 8 bytes
    capacity: usize,    // capacity of the string to grow, size of the current allocation - 8 bytes
}

dbg!(std::size_of::<String>()); // 24
```

Str:

```rust
struct &str {
    data: *const u8 // pointer to string - 8 bytes
    length: usize   // length of the string - 8 bytes
}

dbg!(std::mem::size_of::<&str>()); // 16
```

---

So internally they're quite different, `&str` is smaller, and they do different
things, that's why they are different types. The same goes for the rest of
types.

The following are analogous to `String` and `&str`:

-   `CString` and `CStr`
-   `OsString` and `OsStr`
-   `PathBuf` and `Path`

More about strings: https://fasterthanli.me/articles/working-with-strings-in-rust

---

### Vecs and slices

What about `Vec<u8>`, `[u8; N]`, `&[u8; N]`, `&[u8]`?

`Vec<u8>` and `[u8; N]` are arrays of `u8`; former is growable and
heap-allocated, latter is constant size and may be on the stack.

`&[u8; N]` - a reference to array of type `u8` of size `N`

`&[u8]` - a slice of type `u8` (so, a "view" into an array of type `u8`, either
`Vec<u8>` or `[u8; N]`)

---

"Ok, what does that mean for me, which should i use?"

In methods, use least restricive, most "generic" type:

Instead of:

```rust
fn read_bytes(bytes: &Vec<u8>)
fn read_string(text: &String)
```

do:

```rust
fn read_bytes(bytes: &[u8])
fn read_string(text: &str)
```

---

But dont overthink it for now:

![API](img/api.jpg)

## Big stdlib

Rust stdlib has two stdlibs:

-   `core`, which is a subset of `std`, targets embedded, doesnt support
    allocation and shit
-   `std`, which is bigger, targets programs running on OSes that provide APIs
    for memory allocation, file operations, system calls, etc.

## Tooling (build system, package manager, rustfmt, clippy)

# Getting started

## How install?

Use rustup.rs. It lets you install multiple versions of rust. Usually you'll use
stable but sometimes you might want to use features that are still unstable and
available only on nightly. Also clippy and rustfmt are parts of the toolchain.

## Linux

Install via your package manager or https://rustup.rs/ if it's not in your
distro's repositories. The website installer will automatically prompt you to
install the stable toolchain. If you installed rustup via package manager,
install stable toolchain: `rustup toolchain install stable`.

## Windows

Install via https://rustup.rs. To use MSVC backend, which is recommended, you'll
need to have installed either Visual Studio 2015+ C++ workload or VS C++ build
tools standalone if you don't use visual studio.

You can also use MinGW, but it won't be covered here.

## IDE setup

I personally recommend VS Code with rust-analyzer, but feel free to use
something you're comfortable with if it's supported.

List of Rust IDEs/plugis available at: https://areweideyet.com

![](img/ides.png)

## VS Code + rust-analyzer

What does rust-analyzer do?

![](img/ra1.png)

-   type hinitng
-   autocomplete
-   jump to declaration/definition
-   Autoapply suggestions

After you have Rust stable toolchain installed, just install the VS Code
rust-analyzer extension. In case of difficulties, refer to the
[manual](https://rust-analyzer.github.io/manual.html#vs-code).

## Troubleshooting

The extension works if the root directory of Rust project is opened in VS Code
(the folder that contains `Cargo.toml`). If you have opened a directory with
multiple Rust projects, you'll have to manually specify paths for rust-analyzer.

# Learing Rust

## Basics

Basics of Rust

## Fearless concurrency

Sharing data between different threads

## Crates

Crates

## Other good sources

-   [I am a Java, C#, C or C++ developer, time to do some
    Rust](https://fasterthanli.me/articles/i-am-a-java-csharp-c-or-cplusplus-dev-time-to-do-some-rust)

    Comprehensive introduction to Rust for developers of other Object Oriented
    languages

-   [Declarative memory
    management](https://fasterthanli.me/articles/declarative-memory-management)

    How Rust memory management differs from C or C++

-   [Learn Rust in Y minutes](https://learnxinyminutes.com/docs/rust/)
-   [Rust Book](https://doc.rust-lang.org/book/)

# Other tips

-   Use clone
-   Use clippy

# Sources

-   https://fasterthanli.me
-   https://www.youtube.com/c/fasterthanlime
-   https://www.youtube.com/c/JonGjengset
-   https://pkolaczk.github.io
-   https://www.reddit.com/r/rustjerk
