# Chapter 6 - Enums and Pattern Matching

Allow us to define a type by *enumerating* its possible variants.

## 6.1 - Defining an Enum

A `struct` gives you a type that allows you to group together related fields. An `enum` gives us a way
to say a value is from a set of possible values.

Example in the book is types of IP address - V4 or V6. We define the `enum` as follows:

```rs
enum IpAddrKind {
    V4,
    V6,
}
```

We can then pick its value using the `::` notation:

```rs
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

This is a useful data type for categorical data, such as the IP address type in the example above.
We can use them in combination with `struct`s, for example we could create an IP address `struct` with
both IP standard, and the actual address:

```rs
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
```

We can use an `enum` to be more concise, we can put data into each `enum` variant:

```rs
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1");
```

Each variant can have different types of, or amounts of, data. In the IP address example, V4 
standard is 4 integers between 0 and 255, we can specify each one as a `u8` value within the `enum`
variant, and have all four:

```rs
enum IpAddr {
    V4(u8, u8, u8, u8)
    V6(String)
}

let home = IpAddr::V4(127, 0, 0, 1);
```

Furthermore, like with any other type of data, we can also use `struct`s within `enum`s. And we can
define associated functions and methods for `enum`s using `impl` as with `struct`s.

### The `Option` Enum and its Advantages Over Null Values

`Option` is an `enum` defined by the standard library. It encodes the scenario in which a value
could be either something or nothing. An example of the usefulness of `Option` is requesting the
first value from a list of items. If the list is empty you would get nothing. In the context of the
type system, the compiler can check that you have handled all the cases you need to handle. This
helps prevent bugs.

Unlike many other programs, Rust does not have the *null* feature. The main problem with `Null`
values are that they can lead to errors if you try to use a `Null` value as a non-`Null` value.

The enum `Option` is defined in the standard library as follows:

```rs
enum Option<T> {
    None,
    Some(T),
}
```

The `<T>` syntax is a generic type parameter, we can replace the `T` with a more concrete type as
required:

```rs
let absent_number: Option<i32> = None;
```

`Some` is a way of implementing an `Option` type:

```rs
let some_number = Some(5);
let some_char = Some('e');
```

The type of `some_number` is `Option<i32>`, and the type of `some_char` is `Option<char>`. Rust
infers these types because we specified a value to `Some`. However, we need to annotate the overall
`Option` type as the compiler cannot infer the type from `None`. So in the previous example we had
to annotate the type `i32` to the `Option` for `absent_number`.

It is important to note that since the type of `some_number` is `Option<i32>` and **not** `i32` the
compiler won't let us use `some_number` as thought it is a definite value of `i32`. The following
code will result in an error:

```rs
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(2);

    let sum = x + y;
}
```

```sh
$ cargo run
   Compiling option v0.1.0 (learning_rust/6_Enums_and_Pattern_Matching/option)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:33
  |
5 |     println!("The sum is {}", x + y);
  |                                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            <&'a f32 as Add<f32>>
            <&'a f64 as Add<f64>>
            <&'a i128 as Add<i128>>
            <&'a i16 as Add<i16>>
            <&'a i32 as Add<i32>>
            <&'a i64 as Add<i64>>
            <&'a i8 as Add<i8>>
            <&'a isize as Add<isize>>
          and 48 others

For more information about this error, try `rustc --explain E0277`.
error: could not compile `option` due to previous error
```

The compiler does not know how to combine a value of type `T` with an `Option<T>`. We must first
convert the `Option<T>` to a value of type `<T>` before we can perform operations with it.

This helps prevent errors caused by treating a `Null` value as non-`Null`. 

In the next section we will see how to use an option, in particular, how to extract the `T` from the
`Option<T>`.

## 6.2 - Control Flow with `match`

Think of this like `case` matching in python or bash. It allows us to compare a value against a
series of patterns, and execute code accordingly.

```rs
enum Elf {
    Dark,
    High,
    Orc,
    Wood,
}

#[derive(Debug)]
struct MagicBonus {
    alteration: u8,
    conjuration: u8,
    destruction: u8,
    illusion: u8,
}

fn set_magic_bonus(race: &Elf) -> MagicBonus {
    match race {
        Elf::Dark => MagicBonus {
            alteration: 5,
            conjuration: 0,
            destruction: 10,
            illusion: 5,
        },
        Elf::High => MagicBonus {
            alteration: 5,
            conjuration: 5,
            destruction: 5,
            illusion: 5,
        },
        Elf::Orc => MagicBonus {
            alteration: 0,
            conjuration: 0,
            destruction: 0,
            illusion: 0,
        },
        Elf::Wood => MagicBonus {
            alteration: 5,
            conjuration: 0,
            destruction: 0,
            illusion: 5,
        },
    }
}

fn main() {
    let oswald = Elf::Orc;

    let bonus = set_magic_bonus(&oswald);
    println!("{:?}", bonus);
}
```

We can use code blocks in the `match` group too, by wrapping the code block in curly
braces:

```rs
#[derive(Debug)]
enum Elf {
    Dark,
    High,
    Orc,
    Wood,
}

#[derive(Debug)]
struct MagicBonus {
    alteration: u8,
    conjuration: u8,
    destruction: u8,
    illusion: u8,
}

fn set_magic_bonus(race: &Elf) -> MagicBonus {
    match race {
        // SNIP
        Elf::Orc => {
            println!("Selected: {:?}", race);
            MagicBonus {
                alteration: 0,
                conjuration: 0,
                destruction: 0,
                illusion: 0,
            }
        },
        // SNIP
    }
}

fn main() {
    let oswald = Elf::Orc;

    let bonus = set_magic_bonus(&oswald);
    println!("{:?}", bonus);
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/elves`
Selected: Orc
MagicBonus { alteration: 0, conjuration: 0, destruction: 0, illusion: 0 }
```

### Patters that Bind to Values

We can also match to parts of the value that match the pattern. For example if we wanted to pass our
character name along with our `Elf` `enum`, we can use just the `Elf` race in the match with the
character's name as an argument.

```rs
#[derive(Debug)]
enum Elf {
    Dark(String),
    High(String),
    Orc(String),
    Wood(String),
}

#[derive(Debug)]
struct MagicBonus {
    alteration: u8,
    conjuration: u8,
    destruction: u8,
    illusion: u8,
}

fn set_magic_bonus(race: &Elf) -> MagicBonus {
    match race {
        // SNIP
        Elf::Orc(character_name) => {
            println!("Selected: {:?}", race);
            println!("Character name: {}", character_name);
            MagicBonus {
                alteration: 0,
                conjuration: 0,
                destruction: 0,
                illusion: 0,
            }
        },
        // SNIP
    }
}

fn main() {
    let character = Elf::Orc(String::from("Oswald"));

    let bonus = set_magic_bonus(&character);
    println!("{:?}", bonus);
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/elves`
Selected: Orc("Oswald")
Character name: Oswald
MagicBonus { alteration: 0, conjuration: 0, destruction: 0, illusion: 0 }
```

Each race in `Elf` can take a `String` which would be the character name. The above is not really an
example of how to create a character; it's a silly example, but it highlights the point.

### Matching with `Option<T>`

We can handle `Option<T>` to extract the inner `T` value with `match` too. The below code takes an
`Option<i32>`, and adds 1 if there is a value inside.

```rs
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Consider the `let six` line: we compare the value of `five` against each *arm* in the `match`:

1. `None`: It is not `None` so we move on
2. `Some(i)`: Yes! `i` binds to the value inside the `Some`, in this case `5`. The expression then
adds `1`, and we return `6`.

In the case of the `let none` line, we match on the `None` arm of the `match`. Consequently, we
return `None`.

### Matches Must be Exhaustive

The arms within the `match` statement must cover all possible cases. Otherwise we would get a bug:

```rs
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
    }
}
```

The above would result in an error since we haven't handled the `None` case of the `Option<i32>`.

```sh
$ cargo run
   Compiling exhaustive_match v0.1.0 (learning_rust/6_Enums_and_Pattern_Matching/exhaustive_match)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:2:11
    |
2   |     match x {
    |           ^ pattern `None` not covered
    |
note: `Option<i32>` defined here
   --> .rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:522:5
    |
518 | pub enum Option<T> {
    | ------------------
...
522 |     None,
    |     ^^^^ not covered
    = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
3   ~         Some(i) => Some(i+1),
4   ~         None => todo!(),
    |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `exhaustive_match` due to previous error
```

The compiler knows that we haven't covered the `None` case.

### Catch-all and the `_` Placeholder

With `enum`s and the `match` statement we can take special actions for a few specific values, but
take a default action otherwise:

```rs
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        other => {
            println!("{}", other);
            move_player(other);
        }
    }
}

fn move_player(num_spaces: u8) {}
fn add_hat() {}
fn remove_hat() {}
```

In the above example, we use `other` to catch values that are neither 3 nor 7. Note that `other` is
not a reserved command, this is essentially a variable that we can choose its name - `barry` would
be equally valid!

In this case, we wanted to know the value to pass to `move_player`, so we used `other` as the
argument. If we don't want to use the value in the catch-all pattern we can use the `_` placeholder.
For example we could have the arm being:

```rs
_ => reroll()
```

If we want to do nothing if the catch-all arm then we can express that with an empty tuple: `()`:

```rs
_ => ()
```

## 6.3 - Control Flow with `if let`

We can also use `if let` to handle values that match a single pattern whilst ignoring anything else.
The following two blocks behave identically:

```rs
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("Configuring maximum value to {}", max),
    _ => (),
}
```

```rs
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("Configuring the maximum value to {}", max);
}
```

`if let` takes a pattern and expression separated by an equals sign. If the pattern and expression
match then the code block will run. This is similar to the `match` statement, where the code block
within an arm will only run if the expression matches the arm's pattern.

Like the standard `if` syntax, we can include an `else` statement that will run if the pattern is
not matched.

This syntax is only really useful in cases where we wish to only match on one pattern whilst
ignoring all others (or default action for all others). Essentially, its use loses the exhaustive
checking enforced by `match`. So, it's typically only really useful for `Option<T>` cases.
