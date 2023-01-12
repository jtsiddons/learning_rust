# Chapter 2 - Guessing Game

 We start with a simple exercise to program a simple guessing game. The program will generate a random
 integer between 1 and 100. It will then ask (prompt) the user to input a guess. After making the guess
 the program will then tell us whether we were too high, too low, or bang on the money.

 The aims of this little exercise are to learn basic concepts, and how to apply them in Rust. This 
 includes

 * `let`
 * `match`
 * methods
 * associated functions
 * external crates

 These concepts will then be built upon over the next few chapters. 

 Let's get started!

## Set-up

Start by initialising the project as before. In this case I am using version control for the entire set
of book notes, rather than for individual chapters or mini projects so I will run:

```sh
$ cargo new guessing_game --vcs none
```

As before, this sets up a project directory and initialises a 'Hello, world!' script.

## Stage 1

### Taking and Processing User Input

We start with asking the user for input. The code will then need to process the input and verify that
it is in the expected form. To take input we need to load the `io` library from the standard library. 
This is done with the `use` command.

```rs
use std::io;
```

We can load libraries from the standard library without any additional dependencies. The list of 
libraries and their items can be found [here](https://doc.rust-lang.org/std/).

The first stage of the exercise is taking the input from the user. The code for this stage is below.

```rs
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");
}
```

Let's analyse the code.

We now know about bringing items, such as `io`, from the standard library into scope with `use`.

We are also familiar with the `println!` macro.

### Variables

Consider now the `let` line. Here we are defining the variable, and its type, for the guess. We are also 
setting the variable to a `mut`. What does that mean?

By default variables are **immutable**, or constant. Consider the following line

```rs
let apples = 5;
```

Here we have defined a variable `apples` and bound it to the value `5`. Since we have not declared
`apples` to be `mut` this value will not change.

```rs
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

In the above example we have now assigned the variable `bananas` to `5`. In this case we declared
it as `mut` so we can change its value later on. We will learn more about variables in chapter 3.

In the main example we are assigning the mutable variable `guess` to the value `String::new()`. The 
first component `String` is informing the type of the variable. The `String` type is provided by the 
`std` library and is a *growable*, UTF-8 encoded bit of text.

The second component `::new()` is more complicated. In simple terms it is creating a new, empty 
`String`, but we should break it down a bit more. The `::` section tells us that we are using a function *associated* with the `String` type. `new()` is that function.

```rs
let mut guess = String::new();
```

Creates a new, empty, mutable variable of type `String`.

### Receiving Input from the User

Now consider the next three lines of our stage 1 code:

```rs
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line.");
```

To be clear this is actually a single line of code, the `.` character tells us that we are applying a 
`method` to the previous step. Notice that each of the lines does not end with a `;` suggesting that the 
command continues to the next line.

To get user input using the `io` module we use the function `stdin()` from that library. We can actually 
skip the earlier call of `use std::io;` with a full call to the `io` module and function - n.b. similar 
to how I would do it in R - 

```rs
shd::io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line.");
```

`stdin()` returns an instance of `std::io::Stdin` which is a type to handle standard input from the
terminal. 

The next line `.read_line(&mut guess)` calls the `read_line` method on the `Stdin` handle to get the 
user's input. The `&mut guess` argument tells the function where to store the input. `read_line` 
appends to the end of the `String` argument, and the argument needs to be mutable.

The `&` indicates that the argument is a `reference`. This gives us a way to let multiple parts of the
code access one piece of data without needing to copy it into memory multiple times. By default, 
references are immutable, so we needed to use `&mut guess` as opposed to `&guess`. We'll see more on 
references in Chapter 4.

### Handling Potential Failure

The final line of this section of the code

```rs
    .expect("Failed to read line.");
```

The previous line, `readline...` returns a `Result` value. `Result` is an `enumeration` (or `enum`) 
which is a type that can be in one of multiple possible states, each possible state is a *variant*.
We'll see more about  `enum` in Chapter 6.

`Result`'s variants are `Ok` and `Err`. As one would expect, the `Ok` variant tells us that the 
operation was a success and the `Err` variant tells us it failed and provides details as to why it
may have failed. Like with any Type, we can apply methods to `Result`.

One method that can be applied to an instance of `Result` is the `expect` method which allows us to 
handle errors. If `Result` is an `Err` value the program will error out and crash, displaying the error.
We can instead pass this to `expect` and instead display the message passed as an argument, i.e. 
`"Failed to read line."`. 

If we exclude the `expect` method our code will compile but we will get a warning.

```sh
$ cargo check
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 | /     io::stdin()
11 | |         .read_line(&mut guess);
   | |_______________________________^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```

Best practice is to write error-handling code, but in this simple example we just wish to crash the
program if we are unable to read the user input, so we just use `expect`. We'll learn more about 
error-handling in Chapter 9.

### Printing variable output

We can use the `println!` macro to print out the guess using `{guess}`. The `{}` acts as a place-holder.
For printing a variable we can simply put the variable name within the curly braces. If we wish to 
print output of a function, we need to use empty braces and the expression as an additional argument to
`println!`. For example:

```rs
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

### Testing Stage 1

We can test our code using `cargo run`:

```sh
$ cargo run
   Compiling guessing_game v0.1.0 (/home/joe/learning_rust/2_Guessing_Game/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
15
You guessed: 15
```

## Stage 2

The next part of the exercise is to generate a random number. There is no random number functionality
within the rust standard libraries, so we need to add an external `crate`. We can use the `rand` 
crate.

### Crates

A *crate* is a collection of rust source code files. The project builds a *binary crate* which is an 
executable. The `rand` crate is a library crate, designed to be used in other programs and cannot
be executed on its own. 

To use `rand` we need to add it to our `Cargo.toml` file. 

```toml
[dependencies]
rand = "0.8.5"
```

Cargo understands *Semantic Versioning* which is a standard for writing version numbers. The 
specifier `0.8.5` is short for `^0.8.5` which tells Cargo to use any version that is at least
version `0.8.5`.

Adding this tells Cargo to ship this crate. We can now build our code and we will see it loading 
the necessary libraries.

```sh
cargo build
    Updating crates.io index
  Downloaded cfg-if v1.0.0
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.4
  Downloaded getrandom v0.2.8
  Downloaded ppv-lite86 v0.2.17
  Downloaded rand v0.8.5
  Downloaded libc v0.2.139
  Downloaded 7 crates (824.7 KB) in 0.39s
   Compiling libc v0.2.139
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.8
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (/home/joe/learning_rust/2_Guessing_Game/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 37.97s
```

Cargo will only need to download crates once, so re-running `build` results in the standard 
output:

```sh
cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
```

On first build Cargo creates a `Cargo.lock` file. This is to handle upstream changes in crates. 
Let's say that `rand` released version `0.8.6` which fixes a bug but also contains a regression
that may break our code. Since we have the lock file, cargo will only use `0.8.5` as requested.
Cargo will use the lock file when building. This allows for reproducible builds.

We can update versions using `cargo update`. Here cargo will update to any new version in the
`0.8.x` release, and ignore `0.9.x`. If we want to upgrade to `0.9.0` we need to manually change 
the entry in the `toml` file.

### Generating a Random Number

We now update our code to include the components for stage 2:

```rs
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");
}
```

We have loaded the `rand` library with the `Rng` *trait*. This defines methods that random
number generators implement, and we need it in scope to use these methods.

We won't usually know which traits we need to bring into scope, so we will often have to 
consult the docs. Which can be done from the command line. `cargo doc --open` will load the
documentation for all dependencies and open in a browser.

We load a random number generator, in this case the `rand::thread_rng()` generator. To 
generate an integer between 1 and 100 we apply the `gen_range()` method to it, with the argument
`1..=100`. This argument takes the form `start..=end`.

Note here that we did not specify `mut` indicating that our `secret_number` is constant and
immutable.

We also added a print line for debugging.

Let's now build and run the code.

```sh
$ cargo run
   Compiling guessing_game v0.1.0 (/home/joe/learning_rust/2_Guessing_Game/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is 31
Please input your guess.
22
You guessed: 22
```

If we run it again we get a different secret number.

```sh./target/debug/guessing_game
Guess the number!
The secret number is 13
Please input your guess.
99
You guessed: 99
```

## Stage 3

In this stage we now compare our guess with the secret number.

Let's update the code and analyse it bit-by-bit.

```rs
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

### Comparing the Guess to the Secret Number

To compare two numbers we need to load another type into scope. We load `std::cmp::Ordering`
which is another `enum` with variants:

* `Less`
* `Greater`
* `Equal`

The three options when comparing two numbers.

We added some lines to compare the guess to our secret number. We use a `match` expression
to decide what to do based on each variant of `Ordering` that was returned from `cmp`, which 
compared `guess` to `secret_number`. Match is similar to a `cases` clause in python. It is
made up of *arms*, which consist of a *pattern* to match against, and the code to be run if
the pattern is matched. 

Notice too that I added a `// ` to the line that printed the secret number. This commented
out that line so that it will not be run.

When I attempt to build the code - or in my case save the file and `rust-analyzer` is run 
(which presumably implements `cargo check`) - I get an error notice.

```sh
$ cargo check
    Checking guessing_game v0.1.0 (/home/joe/learning_rust/2_Guessing_Game/guessing_game)
error[E0308]: mismatched types
   --> src/main.rs:23:21
    |
23  |     match guess.cmp(&secret_number) {
    |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
    |                 |
    |                 arguments to this function are incorrect
    |
    = note: expected reference `&String`
               found reference `&{integer}`
note: associated function defined here
   --> /home/joe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:780:8
    |
780 |     fn cmp(&self, other: &Self) -> Ordering;
    |        ^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error
```

This error is because we are trying to compare `guess` which is of type `String` to 
`secret_number` which is an `integer`. The cause of this issue is the line

```rs
let mut guess = String::new();
```

Fixing the guess to a `String`. We need to convert this to an integer value. We add a new
line of code to achieve this.

```rs
let guess: u32 = guess
    .trim()
    .parse()
    .expect("Please type a number!");
```

Here we are creating a new variable named `guess`, which happens to be the same as the 
previous declared variable. Fortunately, rust allows us to *shadow* the previous value og
`guess`. This allows us to re-use the variable name `guess`.

We assign it to a type of `u32` which is an unsigned 32bit integer. We give it the 
previous value of `guess` to which we apply the methods `trim` followed by `parse` and
then `expect`.

`trim` takes the `String` value and removes leading or trailing white-space. This keeps
the value as a `String`.

The `parse` method on `String`s converts the `String` to another type, in this case 
specified by the earlier `u32` declaration. This will only work on characters that can
logically be changed to the desired type, and so returns a `Result`. 

We handle the `Result` `enum` in the same way as before, with the `expect` method.

Let's now run the code.

```sh
git:(main*)cargo run
   Compiling guessing_game v0.1.0 (/home/joe/learning_rust/2_Guessing_Game/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
33
You guessed: 33
You win!
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
11
You guessed: 11
Too Small!
```

I ran the program twice, when I first ran the program I just so happened to get the
guess correct, I wanted to be sure that my code was correct, so I ran it again and
got a different result. 

## Additional Stuff

It's time to increase the complexity of the code. Currently we only get a single guess,
we can introduce a loop to allow is to have multiple guesses.

```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

This will allow for multiple guesses, however, if we guess correctly the code will not 
terminate and continue to ask for guesses. We add a line to break out of the loop if we
are correct:

```rs
Ordering::Equal => {
    println!("You win!");
    break;
},
```

This allows for termination after guessing correctly. However if we input a non-number
we just error out

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
34
You guessed: 34
Too Small!
Please input your guess.
68
You guessed: 68
Too Small!
Please input your guess.
100
You guessed: 100
Too Big!
Please input your guess.
help
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:21:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Let's handle that with proper error handling.

```rs
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

We use a match statement, with the arms on `Ok` and `Err` and proceed accordingly; if guess
can be parsed as a number then we can proceed to the next steps, if not then we continue to
the next loop cycle.

Finally I'll add some comments to the code for clarification.

The final code is:

```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Initialise random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read input from user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Parse input -> number. Continue and ask again if Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
```

We can run the result:

```sh
$ cargo run
   Compiling guessing_game v0.1.0 (/home/joe/learning_rust/2_Guessing_Game/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
14
You guessed: 14
Too Big!
Please input your guess.
7
You guessed: 7
Too Small!
Please input your guess.
11
You guessed: 11
Too Big!
Please input your guess.
9
You guessed: 9
Too Small!
Please input your guess.
10
You guessed: 10
You win!
```
