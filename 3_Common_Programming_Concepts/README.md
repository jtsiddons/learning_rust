# Chapter 3 - Common Programming Concepts

The concepts discussed in this chapter are not unique to rust, they are common across most, if not
all, programming languages. We'll discuss these standard concepts in the context of rust, how they 
are applied for example.

## Outline

* [3.1 - Variables](#3.1)
* [3.2 - Data Types](#3.2)
* [3.3 - Functions](#3.3)
* [3.4 - Comments](#3.4)
* [3.5 - Control Flow](#3.5)

## [3.1 - Variables and Mutability]{#3.1}

### Mutability

Have seen before that by default variables are *immutable* in rust. This means that if a variable 
has been set and is immutable, its value cannot be changed.

For example, the following code will fail to comply because we attempt to reassign the immutable 
variable `x`

```rs
fn main() {
    let x = 5;
    x = 6;
}
```

Indeed this shows an error in `rust-analyzer`: `cannot assign twice to immutable variable 'x'`.

If however, we assign the variable to be *mutable* with the `mut` tag, the variable's value can be 
changed as required:

```rs
fn main() {
    let mut x = 5;
    x = 6;
}
```

### Constants

Constants are similar to immutable variables, in that they are values bound to a name that are not
allowed to change, however there are a few differences.

* We cannot use `mut` with constants. They are **always** immutable.
* We declare a constant with `const` instead of `let`.
* The type of the constant **must** be annotated.
* Can be declared in any scope - including the global scope, which makes them useful for values that
are required by multiple parts of the code.
* Can only be set to a constant expression, not the result of a value that can only be computed on
runtime.

An example of a constant declaration:

```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Convention is that constants are named using capital snake case. 

The compiler can process a limited set of operations for constants at run time. This allows us to 
write the value `10,800` in a easier to understand form: `60 * 60 * 3`.

Constants are valid for the entire time the program runs, within the scope they are declared.

### Shadowing

As part of the guessing game in chapter 2 we were able to declare a new variable with the same name
as a previous variable. We would say that the first value is *shadowed* by the second. The second
variable is what will be seen by the compiler when use the name of the variable. The second variable
*overshadows* the first.

An example of shadowing is below

```rs
fn main() {
    let x = 5;
    println!("The first value of x is: {x}");

    // Shadow x
    let x = x + 1;

    {
        // Shadow x in a local scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/variables`
The first value of x is: 5
The value of x in the inner scope is: 12
The value of x is: 6
```

Let's have a look at what has happened:

1. The program first binds `x` to a value of 5. 
2. It creates a new variable `x` by repeating the `let x =` and adding 1 to its original value. `x` 
is now 6.
3. It then creates an inner scope wherein it creates a variable `x`, doubling its previous value.
Giving `x` a value of 12.
4. It leaves the inner scope, the inner shadowing ends and `x` returns to 6.

The most complicated component here is that there is a local scope.

Shadowing is different to `mut`. We will get a compile-time error if we try to reassign the value 
without the `let` keyword. We are effectively creating a new variable with `let`, whereas with a 
`mut` we are changing the variable in place.

With `let` and shadowing, we can also change the type of a variable:

```rs
fn main() {
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is {spaces}");
}
``````

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/variables`
The value of spaces is 4
```

However, if we try to do this using `mut`, we get an error

```rs
fn main() {
    let mut spaces = "    ";
    spaces = spaces.len();
    println!("The value of spaces is {spaces}");
}
```

```sh
$ cargo run
   Compiling variables v0.1.0 (learning_rust/3_Common_Programming_Concepts/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "    ";
  |                      ------ expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```

## [3.2 - Data Types]{#3.2}

Rust is a *statically* typed language. This means that it must know the type of all variables at 
compile time. The compiler can often infer the type we want based on the value and its use. However,
in the case where many types are possible, for example when we used `parse` to convert a `String` to
a numeric type in the guessing game, we must add a type annotation.

```rs
let guess: u32 = "32".parse().expect("Not a number!");
```

If we do not add the typing (`: u32`) then we will get an error - the compiler needs more information and
doesn't know what type to use.

### Scalars

This represents a single value. Four primary types:

1. Integers.
2. Floating-point numbers.
3. Booleans.
4. Characters.

#### Integers

Integer types in rust:

| Length    | Signed    | Unsigned  |
| ---       | ---       | ---       |
| 8-bit     | `i8`      | `u8`      |
| 16-bit    | `i16`     | `u16`     |
| 32-bit    | `i32`     | `u32`     |
| 64-bit    | `i64`     | `u64`     |
| 128-bit   | `i128`    | `u128`    |
| arch      | `isize`   | `usize`   |

* Signed integers range from $-2^{n-1}$ to $2^{n-1} - 1$
* Unsigned integers range from $0$ to $2^n -1$
* *arch* corresponds to the system architecture - 32-bit systems will use `u32` for example.

We can also specify integers in a number of forms

| Number literal            | Example   |
| ---                       | ---       |
| Decimal                   | `12_124`  |
| Binary                    | `0b1100`  |
| Hex (Base 16)             | `0xf2`    |
| Octal (Base 8)            | `0o14`    |
| Byte (Base 64, `u8` only) | `b'A'`    |

##### Integer Overflow

Consider the case where we have a `u8` variable, which can hold values between $0$ and $255$. If we
try to change the value to a value outside of that range, say $256$, then *integer overflow* will
occur. There are two possible behaviours:

* In debug mode, the compiler will detect the integer overflow and cause a *panic* at runtime. This
means that the program exits with an error.
* If we compile in release mode, rust does not check for integer overflow. If overflow does occur
within the program it will apply *two's complement wrapping*. Essentially values greater than the
maximum range will wrap around. For example in `u8` the value $256$ will become $0$, $257$ will be
$1$ and so on. There will be no panic. 
[More about two's complement.](https://en.wikipedia.org/wiki/Two%27s_complement)
One should not rely on the integer overflow wrapping. We may get a value that we may not expect.

There are methods that we can use to explicitly handle overflow:

* Wrap in all modes with `wrapping_*` methods - e.g. `wrapping_add`.
* Return `None` if overflow with `checked_*` methods.
* Return the value and a boolean indicating overflow with `overflowing_*` methods.
* Saturate at the value's minimum or maximum with `saturating_*` methods.

#### Floats

Two primitive types: `f32` and `f64`. The default is `f64` since it is approximately as fast as 
`f32` on modern CPUs, but has higher precision.

```rs
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

`f32` is often referred to as *single-precision*, whilst `f64` is *double-precision* according to 
the IEEE-754 standard.

#### Operations on Numeric Types

We can apply common mathematical operations to numeric types:

```rs
fn main() {
    // addition & subtraction
    let sum_int = 5 + 10;
    let sum_float = 5.1 + 10.2;
    let diff = 95.4 - 7.1;

    // multiplication
    let product_int = 5 * 10;
    let product_float = 5.1 * 10.2;

    // division
    let quotient = 56.7 / 32.2; // floats
    let truncated = -5 / 3;     // Results in -1
    let remainder = 41 % 5;     // Results in 1
}
```

We cannot combine types here. If we want to add an integer to a float we need to convert the 
integer.

#### Booleans

Values that are true or false.

```rs
fn main() {
    let t = true;

    let f: bool = false;
}
```

#### Characters

Individual characters or symbols

```rs
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

Just like in Julia we specify a character with single quotes and a string with double quotes.

Rust uses 4 bytes for `char`s and represents a Unicode Scalar Value. We'll see more about characters
and strings in chapter 8.

### Compound Types

Tuples and Arrays.

#### Tuples

A way of grouping a number of values with different types into a single compound type. Have a fixed 
size, once declared they can neither grow nor shrink. Optionally annotate with types. For example:

```rs
fn main() {
    let tup: (i32, f64, char) = (500, 4.12, 'u');
}
```

We can extract values with pattern matching (we can skip saving a particular value using `_`), or using a period to access an indexed value
(zero-indexed):

```rs
fn main() {
    let tup: (i32, f64, char) = (500, 4.12, 'u');

    let (i, y, c) = tup;
    // ignore second value
    let (j, _, d) = tup;

    println!("The value of y is {y}");

    let five_hundred = tup.0;
    let u = tup.2;
}
```

#### Arrays

Every element of an array must have the same type. Arrays in rust also have a fixed size - unlike
other languages.

```rs
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

An array is allocated on the *stack* rather than the *heap* - we'll discuss these in chapter 4. An 
array is not as flexible as a *vector*, which can vary in size. 

Arrays are also useful if we know the number of elements will not change, for example an array
containing the 12 months.

We can define an array and annotate its type and size as follows:

```rs
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

We can also create an array containing only the same value repeated:

```rs
fn main() {
    let threes = [3; 5]; // [3, 3, 3, 3, 3]
}
```

We can access elements of an array in a standard way, with square braces and the index. Note that
rust is zero-indexed.

```rs
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let third = a[2];
}
```

If we attempt to access an index that is out of range we get a panic/runtime error.

## [3.3 - Functions]{#3.3}

We use `snake_case` as convention for function names in rust. We declare functions by with `fn`, the
function name and a set of parentheses `()`. We add curly braces `{}` to tell the compiler where the
function body begins and ends.

```rs
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function!");
}
```

### Parameters

We can define functions to have arguments. We can annotate these arguments with type.

```rs
fn main() {
    println!("Hello, world!");

    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}
```

In the above example if we provided the value of $1.23$ to `another_function`, we would get an error
as we are expecting the argument to be of type `i32`.

We must declare the type of the argument in the function signature.

### Statements and Expressions

Functions are a series of statements, optionally ending in an expression.

* **Statement**: instructions that perform an action and do not return a value. End in a `;`.
* **Expressions**: Evaluate to a resultant value. Do not end in `;`.

`let y = 6;` is a statement, it does not return a value. We cannot assign a `let` statement to 
another variable: 

```rs
fn main() {
    let x = (let y = 6);
}
```

Results in the following error on compilation:

```sh
$ cargo run
   Compiling functions v0.1.0 (learning_rust/3_Common_Programming_Concepts/functions)
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^

error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  |

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 3 previous errors; 1 warning emitted
```

Here, the `let y = 6` statement has no return value, there is nothing to set `x` to.

Expressions evaluate to a value & are most of the code we write in rust. Expressions can be part of 
a statement. `5 + 7` is an expression. Calling a function or a macro is an expression.

```rs
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

```rs
{
    let x = 3;
    x + 1
}
```
 is an expression - it returns the value `4`. In the above function, the expression's value is bound
 to the variable y:

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/functions`
The value of y is: 4
```

### Return values

Functions can return values. We don't name the return value, but must specify its type. The return
value is synonymous with the value of the final expression in the function. We can return early with 
the `return` keyword, and can specify a value.

```rs
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
}
```

This function takes no input arguments, but declares a return value of 5, with type `i32`. It is 
equivalent to `let x = 5;`. Consider the following example:

```rs
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

The above code will print `"The value of x is 6"`. The `x + 1` line is an expression. If instead we
append a semi-colon to this expression:

```rs
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

We get an error

```sh
$ cargo run
   Compiling functions v0.1.0 (learning_rust/3_Common_Programming_Concepts/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` due to previous error
```

The compiler treats `x + 1;` as a statement that has no return value. The function is expected to
return an `i32`, so we get a `mismatched types`  error.

## [3.4 - Comments]{#3.4}

We can comment code, or add comment lines with `//`. This can be done at the beginning of a line,
removing that line from the program, or it can be done within the line, removing anything to its 
right. Useful for adding notes or clarifications.

## [3.5 - Control Flow]{#3.5}

The most common ways of controlling the flow of code are `if` statements and loops.

### `if` statements

Allow you to branch code depending on conditions. For example:

```rs
fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}
```

Will display `"Condition was true"`. Whereas:

```rs
fn main() {
    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}
```

Will display `"Condition was false"`. 

In the above examples, the condition was `number < 5`. The condition is an expression that must 
return a boolean value. For example, the following will generate a `mismatched types` error:

```rs
fn main() {
    let number = 3;

    if number {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}
```

Since the condition `number` will return `3`, which is not a boolean.

We can handle multiple conditions with `if else`:

```rs
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/control_flow`
Number is divisible by 3
```

Notice that our input number, `6`, is divisible by both 3 and 2. However it only displayed the 
message about 3. This is because once a condition has been met and the relevant block is executed
the if statement is complete, it does not check any further conditions.

#### `if` in a `let` statement

We can declare a value using a conditional - this is a ternary operator.

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

Will display `"The value of number is: 5"`. We cannot mix types in a ternary operator:

```rs
let number = if condition { 5 } else { "six" };
```

Will return a `if ane else incompatible types` error.

### Repetition - Loops

We have already seen the most simple form of loops in the guessing game example. Simple loops take
the form:

```rs
fn main() {
    loop {
        println!("Again!");
    }
}
```

This will print `"Again!"` continuously. We can manually stop it with ctrl-c keyboard shortcut. In
the guessing game example, we added a `break` clause once we had guessed correctly. This stops the 
loop.

We can assign a value to the output from a loop:

```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is: {result}");
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/control_flow`
The result is: 20
```

#### Loop labels

If we have nested loops, or loop-ception, when we use `break` or `continue` these act on the 
innermost loop **at that point**. We can optionally label a loop and specify that we wish to break
out of the labelled loop as follows:

```rs
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

The first `break` has no label and so will exit the inner loop when `remaining` is `9`. The second
`break` is labelled `counting_up`, and so will exit the outer loop, which is also labelled 
`counting_up` when `count` is `2`:

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/control_flow`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

### Conditional Loops

We can run loops until a condition is met with a `while` loop. We can implement this by combining
a `loop`, `if`, `else`, and `break`. However a `while` loop is a much shorter approach:

```rs
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("Lift Off!");
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/control_flow`
3!
2!
1!
Lift Off!
```

### Looping through a collection

We can loop through a collection, such as an array or vector with `for`.

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("Current element is {element}");
    };
}
```
