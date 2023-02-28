# Chapter 9 - Error Handling

Rust has a number of features for handing errors, allowing us to take action on our code in order to
get it to compile. This helps our program be more robust.

There are two major categories of error: *recoverable* and *unrecoverable* errors. A recoverable
error could be something like *file not found*, wherein we would want to report the problem to the
user and give them an opportunity to try again. Unrecoverable errors are symptoms of bugs, for
example trying to access an index outside of an array's bounds, in which case we would want to
immediately stop the program.

Most languages allow the programmer to handle errors such as those above with *try-except* patterns.
Rust does not have exceptions, instead relying on the `Result<T, E>` type for recoverable errors and
the `panic!` macro to stop execution on unrecoverable errors.

## 9.1 - The `panic!` Macro and Unrecoverable Errors

In practice there are two ways to cause a panic:

1. Taking action that causes the code to panic, for example trying to access an index that is not in
the bounds of an array

```rs
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

2. Calling the `panic!` macro directly

```rs
fn main() {
    panic!("I've errored for no reason!");
}
```

When a panic occurs, the default action is that the code starts to unwind - Rust walks back up the
stack and cleans up the data from each function. This clean-up action takes a lot of work \& can
contribute to the size of the program's binary. 

If your binary needs to be as small as possible, then Rust allows you to choose the alternative
option of just aborting on panic. This action terminates the program but does not clean up. This
memory left behind by the application must then be cleaned up by the operating system. To change to
this *abort* only behaviour you add

```toml
[profile.release]
panic = 'abort'
```

To the *Cargo.toml* file.

### Stacktrace

By default, the walking up the stack is done quietly \& output from the process is not displayed to
the user. However, we may want to see this information as it can be used to diagnose the problem in
our code. To do this we can use the environment variable `RUST_BACKTRACE=1`. Let's see this in
action. The following code

```rs
fn main() {
    let v = vec![1, 2, 3];

    v[99];
```

Results in the following error when we run `cargo run`:

```sh
$ cargo run
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

If we set the necessary environment variable, we get:

```sh
$ RUST_BACKTRACE=1 cargo run
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/panicking.rs:64:14
   2: core::panicking::panic_bounds_check
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/panicking.rs:147:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/slice/index.rs:260:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/mod.rs:2727:9
   6: panic::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/ops/function.rs:507:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

This is effectively similar to the *stacktrace* that we get in Julia when we error out and is very
useful for diagnosing errors.

We will see more about the `panic!` macro later in this chapter, when we discuss "To `panic!` or not
to `panic!`".

## 9.2 - The `Result` Type and Recoverable Errors

You won't always want the program to panic when a function fails for a reason that you might expect.
In this case rust has the `Result<T, E>` type.

An example might be that you wish to open a file. Instead of panicking if the file does not exist,
we may wish to create that file instead. Or, if it's from a user input, then we may want to ask the
user to try again rather than panic due to a typo.

The `Result` type is an `enum` with two variants:

```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` and `E` are *generic* type parameters - these will be discussed in the next chapter. Essentially
`T` is the type of the value that would be returned in the case that the operation was successful
and is returned in the `Ok` variant. `E` is the type of error that is returned if the operation is
not successful.

Let's look at an example - in this case loading a file:

```rs
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

The return type of `File::open` is a `Result<T, E>`, where `T` is filled by the implementation of
`File::open` - i.e. a file handle of type `std::fs::File` - or simply `File` since this has been
brought into scope by the `use` earlier.

If `File::open` is successful, the return value in `greeting_file_result` is an instance of
`Ok(std::fs::File)`, otherwise it is an instance of `Err(E)` where `E` specifies the type of error.
We can handle both cases with a `match` statement:

```rs
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

### Handling Specific Errors

We can *nest* match statements in order to handle different kinds of error. For example, if the
above operation fails because the file does not exist, we may wish to create the file:

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

The error type associated with `File::open` is `io::Error`, this is a struct provided by `std`. This
struct has a `kind` method, which returns an `io::ErrorKind` value. This is an enum which
has variants for the different kinds of error that may result from an io operation.

If the `File::open` operation fails because the file does not exist, then the code attempts to
create a file. This also returns a `Result`, so we need to handle it appropriately. If `File::open`
fails from another error we print it out - this is the `other_error` arm. I am unsure if this is a
reserved keyword or if I can put any name here. The code runs if I change its name, however I don't
know if that is because what I did was valid, or because that arm does not activate. Given this is
rust, I assume the compiler would catch if it was invalid, but I don't know for certain.

### Alternatives to `match` and `Result<T, E>`

Nesting multiple `match` statements can get messy very quickly, and can easily become hard to read.
Whilst the `match` statement can be incredibly powerful, it isn't necessarily the best option for
clean, easy to read code. Another approach for the above code could be to use the `unwrap_or_else`
method:

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

In my opinion, the above is harder to read. However it serves as an alternative to using multiple
`match` statements. The use of the `|` around `error` in the above example is called a *closure*.
This is a function that can capture the enclosing environment. This appears to be similar to an
anonymous function or lambda function? We will see more about closures in chapter 13.

### `unwrap` \& `expect`

Another alternative to `match` is to use the `unwrap` method. This is a shortcut that returns the
value inside the `Ok` variant, or panics if the `Err` variant is returned.

```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

The above will run the `panic!` macro if the file does not exist. Similarly, we can use `expect` to
display a message with the `panic!` macro:

```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in the project");
}
```

The message in the `expect` method is displayed by the `panic!` call. Typically, `expect` is more
useful than `unwrap` since you can give context as to why the operation is expected to be
successful. This would not be useful if we were asking the user to input the file-name however. If
we are including files within our package though, `expect` is useful as we would expect that a file
is part of the package - if it is missing we would want to give that information to the user so they
can obtain the file.

### Propagating Errors

This is the process of passing errors that occur within a function back to the code that calls the
function, so that the original code can handle the error, as opposed to handling the error within
the function. This gives more control to the calling code, where there may be additional context
that dictates how specific errors should be handled.

```rs
use std::fs::File;
use std::io::{self, Read);

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

The above function tries to read a username from the file `hello.txt`. If the file does not exist,
or cannot be read then the errors are returned to the calling code.

The function returns a instance of `Result<T, E>`, where in this case `T` is a `String` and the
`E` variant is an instance of `io.Error` - an error from io operations.

If the function is successful, the `Ok` variant is returned, containing a `String` which is the
username - the contents of `hello.txt`. Otherwise, the calling code receives an `io.Error` in the
`Err` variant of the `Result` - which can then be handled appropriately by the calling code. This
type of error is what we get if either `File:::open` or `read_to_string` fail.

This usage pattern is very common in rust - so common in fact that there is a shortcut for
propagating errors: `?`

### The `?` Shortcut

A `?` placed after a `Result` value is defined works almost the same way as the `match` statement in
the example above. If the value is the `Err` variant then it is returned to the calling code.
Otherwise, if the value is the `Ok` variant, then it is unwrapped into the variable for use in the
function. The code below has the same functionality as the above example:

```rs

use std::fs::File;
use std::io::{self, Read);

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

The difference between `match` and `?` is that the `?` operator passes error values through the
`from` function (from the `From` trait). This function is used to convert one type to another. In
this case it converts the error received into the error type defined by the return type of the
function - `io::Error`. This is useful when a function returns a single error type that represents
all the possible ways in which the function may fail - even if parts could fail for a variety of
different reasons (or underlying error types).

If we have a custom error type for our function, let's say `UserNameError`, we would need to define
an `impl` `From` for this type of error: `impl From<io::Error> for UserNameError`. This allows the
`?` operator to pass the `io::Error` through `from` to our custom `UserNameError`.

An additional difference is that we needed to add `Ok(username)` at the end of the above example.
This is because `?` also unwraps the `Ok` variant, but our function needs to return a `Result` type
which needs an `Ok` variant. Therefore, we need to convert the result `username` to the `Ok`
variant.

We can chain the `?` operator, so that we can shorten the above example to:

```rs
use std::fs::File;
use std::io::{self, Read);

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

We can also shorten this code by using `fs::read_to_string` instead of using the method on the file
handle. In this case, we call the `fs::read_to_string` directly on the filename. This function
returns the same `Result<String, io::Error>` type that we want:

```rs
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

#### When can we use `?`?

Since the `?` operator performs an early return out of a function, its output must be compatible
with the function's return type, i.e. there must be an appropriate `from` function that `?` can
pass the result through so that it matches the function's return type.

For example:

```rs
use sd::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

Will fail because the return type of `main` is `()`, not `Result`. The compiler would tell us that
we can only use the `?` operator on types that implement `FromResidual`, this includes `Result` and
`Option`, and we could create a type that implements `FromResidual`.

If we use the `?` on an `Option` in a function that returns `Option`, the behaviour is similar to
the case with `Result`, namely that if it receives `None` then it returns that value early.

The `?` operator cannot convert between a `Result` and an `Option` (or vice-versa), so we cannot
mix and match its use - we need to do those conversions explicitly.

`main` function is special since it is the entry and exit point of an executable \& has restrictions
on its return type. Normally its return type is `()`, but we can have it return `Result<(), E>`,
allowing us to use `?` in that case:

```rs
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

`Box<dyn Error>` is a *trait object*, which will be discussed in chapter 17. It essentially means
"any kind of error". In combination with `?` it allows any `Err` value to be returned early. If
`main` returns with `Ok(())`, it will exit with a value of `0`, if it errors it will exit with a
non-zero value.

The `main` may have any return type that implements the `std::process::Termination` trait. This
contains a `report` function which returns an `ExitCode`.

## 9.3 - To `panic!` or Not to `panic!`?

In simple terms, when using `panic!` there is no way to recover, essentially you're making the
decision that the situation is unrecoverable - even if it might actually be recoverable. Although,
this may actually be the appropriate case.

For examples, tests, or prototypes it is often more useful to `panic!` rather than return a
`Result`. 

For examples, including robust error-handling may make the example unclear. In this case it is
typically understood that use of `unwrap`, which could panic, is used in place of error-handling.

`unwrap` and `expect` are useful when prototyping - before you've decided how you want to handle
these errors. They can be temporary markers, indicating to you where you need to handle errors
appropriately.

When testing, if a method call fails then you want the whole test to fail too - even if that is not
the functionality under test.

It can also be appropriate to use `unwrap` or `expect` when there is some logic that guarantees the
result will have an `Ok` value, but the compiler won't be able to determine that. You will still
have a `Result` that you'll need to handle. For example:

```rs
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

In the above example, we create an instance of `IpAddr` by hardcoding a string value. We know for
definite that this is a valid instance, but the compiler does not. We can use `expect` here, and
the argument explains why it's valid - the hardcoded IP address should be valid. This would not be
an appropriate choice if we were to request an IP address from the user though.

### General Guidelines for Error Handling

Advisable to panic when your code could end up in a bad state. i.e. when some assumption, guarantee,
contract, or invariant has been broken, e.g. invalid values, contradictory values, missing values
are passed to code plus one or more of:

* Bad state is something unexpected, not something that could happen occasionally, for example a
user using the wrong date format.
* Code after this point relies on not being in a bad state, rather than explicitly checking for
problem at every stage.
* Cannot encode the information in types. See chapter 17.

It's better to return an error (if possible) if a user passes values that don't make sense to your
function. This allows the user the choice on how they wish to proceed. However, if continuing would
be insecure or harmful it is better to `panic!`, informing the user that there may be a bug in their
code. It is also appropriate to use `panic!` if you are relying on external code that you do not
control.

If failure is expected, returning a `Result` is more appropriate.

If the code can put the user at risk if it is called with invalid values then your code should first
verify the validity of the values, and panic if they are not. This is primarily for security
reasons, for example, it could be dangerous for a user to access data that is out of bounds - i.e.
overflow. Functions can often have a *contract* - behaviour is only guaranteed if the inputs meet
particular requirements - for example some parameter that is bounded by 0 and 1. Your code should
verify that the inputs satisfy the contract before continuing.

However, frequent error checking throughout the code can become cumbersome and involves a lot of
work. An alternative approach is to use custom types for variables that have some restrictions on
them, and perform the restriction check on creation of the instance.

For example, if we know that a particular argument for a function is bounded by ±1 we could define a
type and methods for that type \& require that type as input to the function.

```rs
pub struct Alpha {
    value: f32,
}

impl Alpha {
    pub fn new(value: f32) -> Alpha {
        if value < -1 || value > 1 {
            panic!("Alpha must be between -1 and 1 (inclusively). Found: {value}");
        }

        return Alpha { value }
    }

    pub fn value(&self) -> f32 {
        return self.value
    }
}
```

We first define the struct for the type - `Alpha`, which has a `value` field that is a float. We
then implement an associated function - `new` on the `Alpha` struct that takes a float and creates a
new instance of `Alpha`. This function performs the necessary check that the value is valid, i.e.
that it is constrained by ±1. If it does not pass the test then it panics, alerting the user that
they have a bug that requires fixing.

Finally, we have the *getter* function, named `value` which borrows `self` and returns the value
field of the `Alpha` instance. This is required because the `value` field is private. Being private
is important, it prevents code using the struct from changing or setting its value directly - code
outside the module is required to use `Alpha::new` to create a new instance, this guarantees that
there is no way for an `Alpha` value to be invalid. One should ensure that requirements for structs
are part of the public documentation.
