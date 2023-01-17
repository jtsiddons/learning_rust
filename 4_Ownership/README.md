# Chapter 4 - Understanding Ownership

Ownership is one of rust's unique features, and is a very important concept which has implications
for the rest of the language. In essence, it enables rust to make memory safety guarantees without
needing a garbage collector.

## 4.1 - What is Ownership

A set of rules that govern how a program manages memory. There are three common approaches for 
managing memory:

1. Garbage collection - looks for no-longer used memory as the program runs;
2. The programmer must explicitly allocate and free the memory;
3. Manage memory through a set of rules checked by the compiler $\leftarrow$ **Rust**

This is something that I will need to learn and become experienced with in rust, as it is a key 
concept for the language.

### Stack and Heap

In most languages you aren't required to think about the *stack* or the *heap* very often, if at
all. Rust, however, is a *systems* programming language, and whether a value is on the stack or the
heap affects how the language behaves.

Both are parts of the memory that are available to a program at runtime, however they are structured
and behave differently:

#### Stack:

* Stores values in order they are received, removes in the opposite order.
* Last in, first out.
* Adding data is *pushing*, removing is *popping* (think like `push!` and `pop!` operations on 
vectors in Julia).
* Think about adding to, and removing from a stack of plates - both operations are performed to the
top of the stack.
* Data must have known, fixed size.

#### Heap:

* Less organised.
* Request certain amount of space.
* Memory allocator finds suitable empty slot \& returns a *pointer* (the address for the location).
* *Allocating* on the heap.
* Can store the *pointer* on the stack.
* Think about a restaurant. You request a table large enough for your group, the waiter finds 
(allocates) a suitable table. If a guest shows up late they can ask where you are seated, the waiter
will show them (point) to your table.

Pushing to the stack is the faster operation. Allocating to the heap requires the allocator to find
a suitable place to store the data. Whereas pushing to the stack always adds to the top of the 
stack. Similarly, accessing data on the heap is slower, processors are faster when they have to jump
around through the memory less.

Consider again the example of the waiter at the restaurant. It is much more efficient for the waiter
to collect orders from each table in full, rather than jumping between tables for each individual
patron. Similarly, a processor is more efficient if it works on data closer to other data - i.e. on 
the stack, rather than within the heap where it may be further away.

When code calls a function, the values passed as arguments (including potentially pointers), and 
local variables within the function are pushed to the stack. Once the function returns the values 
popped from the stack.

Ownership addresses the challenges of:

* What parts of the code use data on the heap,
* Minimising duplication on the heap,
* Cleaning up unused data on the heap.

### Ownership Rules

* Each value has an *owner*.
* There can only be one owner at any given time.
* When the owner goes out of scope, the value is dropped.

### Variable Scope

Consider the following example code:

```rs
// ... stuff
// s is not valid here, it has not been declared
{
    let s = "Hello there!"; // s is valid from here, in this local scope

    // ... stuff with s
}
// the local scope is now over and s is no longer valid
// ... more stuff
```

We have an outer *global* scope, where `s` is not defined. We then enter a *local* scope (initiated
with the `{`), `s` is declared within this local scope and any code below its declaration and within
local scope can use it. Once the local scope is over (terminated at `}`), `s` goes out of scope and 
is no longer valid.

* When `s` comes into scope it is valid.
* It remains valid until it leaves scope.

### The `String` Type

The data types we have discussed thus far have all been of a known size and can be pushed to and 
popped from the stack.

`String`s provide a good example of Ownership. There are essentially two forms that strings can 
take, *string literal*s and *String*s.

We've seen string literals before, this is a string that is hard-coded into our program. They are 
convenient but aren't always suitable. For instance string literals are immutable. Also, not every
string is of known size. In a previous example we took input from the user, we cannot use a string
literal here as we cannot know the length of the string and hence its size in memory. In this case 
we would use the `String` type. This type manages data allocated on the heap. We can create a String
from a literal using the `from` function:

```rs
fn main() {
    let literal_string = "hello";
    let s = String::from(literal_string);

    println!("The string is {}", s);
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ownership`
The string is hello
```

The variable is now `mutable`:

```rs
fn main() {
    let literal_string = "hello";
    let mut s = String::from(literal_string);
    
    s.push_str(", world!");

    println!("The string is {}", s);
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ownership`
The string is hello, world!
```

### Memory \& Allocation

For the case of a string-literal, we know its contents at compile time, so the text is hard-coded 
into the final executable. However, we cannot put a blob of memory into the binary for each piece of
text whose size is unknown and can change as the program runs.

With the `String` type, in order to support a mutable piece of text whose length may change we do 
not know its length at compile time, so memory must be allocated at run time. We then need a way to
return this memory when we are done with the `String`.

The first part is done for us by `String::from`, which requests the memory that it needs.

The second part is more complicated. In languages with a garbage collector, this is all handled 
automatically by the garbage collector which tracks memory that is no longer being used. In 
languages without garbage collection, we need to manually keep track of and return memory that is 
no longer being used. This is something that needs considerable care to handle freeing memory 
correctly. 

Rust takes a different approach. Memory is automatically returned when the variable who owns it goes
out of scope. Consider the earlier example:

```rs
// ... stuff
// s is not valid here, it has not been declared
{
    let s = "Hello there!"; // s is valid from here, in this local scope

    // ... stuff with s
}
// the local scope is now over and s is no longer valid
// ... more stuff
```

The variable `s` goes out of scope at the `}`. At this point rust automatically calls `drop`.

Whilst this seems simple, it can cause unexpected results in comparison to other languages.

### Variables and Data Interacting with Move

Consider the following example:

```rs
let x = 5;
let y = x;
```

Here we bind the value `5` to `x`, then make a copy and bind it to `y`. Under the bonnet this is
exactly what happens. In this case the value is an integer and we know its size, so both values of
`5` are pushed on to the stack. This is not the case for `Strings`:

```rs
let s1 = String::from("Hello");
let s2 = s1;
```

We might assume intuitively that the same thing has happened. However, that is not quite true. Let's
see what happens to a `String` under the bonnet:

![A `String` is made up of three components: `ptr` a pointer to the data, `len`, and `capacity`](https://doc.rust-lang.org/book/img/trpl04-01.svg)

A `String` is made up of three components: `ptr` a pointer to the location of the memory that holds 
the data, `len`, and `capacity`. `len` is the length of the `String`, and `capacity` is the length 
of the `String` in bytes, which is not necessarily the same as `len`.

When we assign `s1` to `s2`, we create a copy of the data assigned to `s1`, meaning we copy the
`ptr`, `len`, and `capacity` that are on the stack. Resulting in:

![`s1` and `s2` representing the data on the stack, both pointing to the same string data on the heap](https://doc.rust-lang.org/book/img/trpl04-02.svg)

We don't actually create a copy of the string itself. If we did, this could be very expensive on the
disk if the data on the heap was very large.

This is not the full story. Recall that when a variable goes out of scope then its memory is freed 
up by rust using the `drop` function. This would cause a problem, when `s1` and `s2` go out of 
scope, they will **both** try to free up the memory! This is known as *double-free* and is a memory
safety bug, and can cause memory corruption, which can lead to security vulnerabilities.

To ensure memory safety, rust assumes that `s1` has gone out of scope after the `let s2 = s1;` line.
It considers `s1` as no longer valid. This solves the problem, only `s2` is valid.

It may seem as though we are making a *shallow copy* of `s1` - where we only copy the pointer, 
length, and capacity, not the data - but because rust invalidates `s1`, we are actually performing a
*move*. We move `s1` into `s2`.

```sh
cargo run
   Compiling ownership v0.1.0 (learning_rust/4_Ownership/ownership)
warning: unused variable: `s2`
 --> src/main.rs:3:9
  |
3 |     let s2 = s1;
  |         ^^ help: if this is intentional, prefix it with an underscore: `_s2`
  |
  = note: `#[warn(unused_variables)]` on by default

error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:20
  |
2 |     let s1 = String::from("Hello!");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}", s1);
  |                    ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
warning: `ownership` (bin "ownership") generated 1 warning
error: could not compile `ownership` due to previous error; 1 warning emitted
```

The following graphic explains what is actually happening:

![`s1` is moved into `s2`. When we assign `s2 = s1` we take `s1` out of scope and it becomes invalid.](https://doc.rust-lang.org/book/img/trpl04-04.svg)

### Variables and Data Interacting with Clone

We can get a *deep* copy of the actual heap data of the `String`, we can use the `clone` method:

```rs
let s1 = String::from("Hello!");
let s2 = s1.clone();

println!("s1 = {}\ns2 = {}", s1, s2);
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ownership`
s1 = Hello!
s2 = Hello!
```

### Stack-Only Data: Copy

The earlier example with integers seems to contradict what we have just learnt. Indeed we maintain
both `x` and `y`:

```rs
let x = 5;
let y = x;

println!("x = {}\ny = {}", x, y);
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ownership`
x = 5
y = 5
```

This is because as integers, we know the size of both `x` and `y`, and they are both stored on the
stack at compile time. Actual copies are quick to make in this case. Rust has a special annotation
in this case: the `Copy` *trait*. We'll see more about traits in chapter 10. If a type implements 
the `Copy` trait, then variables that use it are copied rather than moved.

We cannot annotate a type with `Copy` if the type, or any of its parts, implement the `Drop` trait.
The `Drop` trait tells us that something special needs to happen when the variable goes out of 
scope.

Some types that implement `Copy`:

* Integers,
* Boolean,
* Floats,
* Chars,
* Tuples - if they only contain types that also implement `Copy`
    * `(i32, i32)` implements `Copy`
    * `(i32, String)` does not.

### Ownership and Functions

Passing a variable to a function will move or copy, in the same way as assignment:

```rs
fn main() {
    let s = String::from("Hello!"); // s comes into scope

    take_ownership(s); // s moves into take_ownership
                       // s no longer in scope

    let x = 5; // x into scope

    makes_copy(x); // x moves into makes_copy
                   // i32 is Copy, okay to still use x
}

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope. `drop` is called.
  // Memory is freed.

fn makes_copy(some_int: i32) { // some_int comes into scope
    println!("{}", some_int);
} // some_int goes out of scope. Nothing special happens
```

If we try to use `s` after moving it into `take_ownership`, rust will throw a compile-time error:

```rs
fn main() {
    let s = String::from("Hello!"); // s comes into scope

    take_ownership(s); // s moves into take_ownership
                       // s no longer in scope

    println!("{}", s);

    let x = 5; // x into scope

    makes_copy(x); // x moves into makes_copy
                   // i32 is Copy, okay to still use x

    print!("{}", x);
}

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope. `drop` is called.
  // Memory is freed.

fn makes_copy(some_int: i32) { // some_int comes into scope
    println!("{}", some_int);
} // some_int goes out of scope. Nothing special happens
```

```sh
$ cargo run
   Compiling ownership v0.1.0 (learning_rust/4_Ownership/ownership)
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:7:20
  |
2 |     let s = String::from("Hello!"); // s comes into scope
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |
4 |     take_ownership(s); // s moves into take_ownership
  |                    - value moved here
...
7 |     println!("{}", s);
  |                    ^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

Notice that we do not get an error when we try to print `x` after the call to `makes_copy`.

### Return Values and Scope

We can also transfer ownership using `return` values from functions:

```rs
fn main() {
    let s1 = gives_ownership(); // s1 into scope from
                                // gives_ownership

    let s2 = String::from("Hello"); // s2 into scope
    
    let s3 = takes_and_gives_back(s2); // s2 moved into takes_and_gives_back
                                       // which moves its value into s3

} // s1 and s3 out of scope and dropped. s2 was moved, nothing happens.

fn gives_ownership() -> String { // returns its value to into the scope
                                 // of the function that called it
    
    let some_string = String::from("I'm all yours!"); // some_sting into scope

    some_string // some_string is returned
                // moves into scope of calling function

}

fn takes_and_gives_back(a_string: String) -> String { // a_string into scope
    a_string // a_string is returned to scope of calling function
}
```

We can return multiple values from a function:

```rs
fn main() {
    let s1 = String::from("Hello there!");

    let (s2, len) = calculate_length(s1);

    println!("The length of \"{}\" is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ownership`
The length of "Hello there!" is 12.
```

All of this ownership management can be a bit tedious, and hard work. Fortunately Rust has a feature
that allows us to use a value without transferring ownership: *references*.

## 4.2 - References and Borrowing

It is very clunky to have to return the input string from our function, as in the previous tuple 
example. Instead, we can *reference* the `String`:

```rs
fn main() {
    let s1 = String::from("Hello there!");

    let len = calculate_length(&s1);        // Note the use of the ampersand
                                            // which references `s1`.

    println!("The length of \"{}\" is {}." s1, len);
}

fn calculate_length(s: &String) -> usize {  // Expect reference as argument
    s.len()
}
```

Results in

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/references`
The length of "Hello there!" is 12.
```

We input `&s1` as an argument to the `calculate_length` function, which expects a value of type
`&String`. The ampersand represents a *reference* to the original variable. Essentially the argument
`s` in `calculate_length`, which is type `&String`, is a pointer to the actual variable:

![`&String` `s` pointing to the `String` `s1`.](https://doc.rust-lang.org/book/img/trpl04-05.svg)

The `&s1` in the function call creates a reference to `s1`, it does not own it. This means that `s1`
is no t dropped when the reference goes out of scope.

### Mutate a *Reference*?

What happens if we try to mutate the reference instead of calculating its length?

```rs
fn main() {
    let s1 = String::from("Hello there!");

    mutate_reference(&s1);
}

fn mutate_reference(some_string: &String)  {  // Expect reference as argument
    some_string.push_str("\nGeneral Kenobi!");
}
```

We get an error when compiling: 

```sh
$ cargo run
   Compiling references v0.1.0 (learning_rust/4_Ownership/references)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn mutate_reference(some_string: &String)  {  // Expect reference as argument
  |                                  ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str("\nGeneral Kenobi!");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `references` due to previous error
```

We can mutate a reference using `mut` in the definition of the `String`, and `&mut String` as the 
type for the function argument:

```rs
fn main() {
    let mut s1 = String::from("Hello there!");

    mutate_reference(&mut s1);

    println!("s1 is:\n\"{}\"", s1);
}

fn mutate_reference(some_string: &mut String)  {  // Expect mut reference as argument
    some_string.push_str("\nGeneral Kenobi!");
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/references`
s1 is:
"Hello there!
General Kenobi!
```

Steps:

1. Change `s1` to be `mut`
2. Modify function argument to expect type of `&mut String`.
3. Create a mutable reference with `&mut s1` in the function call.

One restriction to mutable references: we can only have a single mutable reference to a variable:

```rs
fn main() {
    let mut s = String::from("Hello there!");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2)
}
```

```sh
$ cargo run
   Compiling references v0.1.0 (learning_rust/4_Ownership/references)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2)
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `references` due to previous error
```

So let's  think about what is happening here. We define `r1` as a mutable reference to `s`. We then
assign `r2` as a mutable reference to the same variable. This second assignment overwrites the 
first, to prevent multiple mutable references to `s`. However, we need `r1` to last until the 
`println!` macro. 

We can see that this is what happens by removing `r1` from the print statement: `println!("{}", r2)`
which only returns an unused variable warning for `r1`:

```sh
$ cargo run
warning: unused variable: `r1`
 --> src/main.rs:4:9
  |
4 |     let r1 = &mut s;
  |         ^^ help: if this is intentional, prefix it with an underscore: `_r1`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `references` (bin "references") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/references`
Hello there!
```

Alternatively, we can use `r1` in a local scope within curly braces:

```rs
fn main() {
    let mut s = String::from("Hello there!");

    {
        let r1 = &mut s;
    } // r1 out of scope
    let r2 = &mut s;
}
```

The above would compile with two unused variable warnings for `r1` and `r2`.

We can have multiple immutable references, however we cannot introduce a mutable reference if we
already have an immutable reference:

```rs
fn main() {
    let mut s = String::from("Hello there!");

    let immutable_ref1 = &s; // Fine
    let immutable_ref2 = &s; // Also fine
    let mutable_ref1 = &mut s; // Now we have a problem

    println!(
        "{}, {}, and {}",
        immutable_ref1, immutable_ref2, mutable_ref1
    )
}
```

```sh
$ cargo run
   Compiling references v0.1.0 (learning_rust/4_Ownership/references)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:6:24
   |
4  |     let immutable_ref1 = &s; // Fine
   |                          -- immutable borrow occurs here
5  |     let immutable_ref2 = &s; // Also fine
6  |     let mutable_ref1 = &mut s; // Now we have a problem
   |                        ^^^^^^ mutable borrow occurs here
...
10 |         immutable_ref1, immutable_ref2, mutable_ref1
   |         -------------- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `references` due to previous error
```

This is a similar error to before. The issue, as before, is that we need `immutable_ref1` after the
assignment of `mutable_ref1` (which causes a clash with `immutable_ref1`) in the print statement.
Once again, removing reference to `immutable_ref1` and `immutable_ref2` in the print statement
removes the error, both immutable references are dropped when we assign `mutable_ref1`.

```rs
fn main() {
    let mut s = String::from("Hello there!");

    let immutable_ref1 = &s; // Fine
    let immutable_ref2 = &s; // Also fine
    println!("{}, {}", immutable_ref1, immutable_ref2);
    // immutable_ref1 and immutable_ref2 now out of scope

    let mutable_ref1 = &mut s; // Now not a problem
    println!("{}", mutable_ref1)
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/references`
Hello there!, Hello there!
Hello there!
```

Borrow checking errors can be a pain, but it is one of rust's most useful features \& helps detect
potential bugs early! It also helps to enforce good management of a variable's scope.

### *Dangling* References

A `Dangling Pointer` is a pointer that references a location in memory that may have been given to
someone else - freeing some memory whilst preserving a pointer to that memory. For example a pointer
to a variable that has gone out of scope:

```rs
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("Hello there!");

    &s
}
```

```sh
$ cargo run
   Compiling references v0.1.0 (learning_rust/4_Ownership/references)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                 +++++++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `references` due to previous error
```

This error refers to the concept of a *lifetime*. We'll discuss them in chapter 10. The main 
component of the error is 

> this function's return type contains a borrowed value, but there is no value for it to be borrowed from

Essentially, after `dangle` has finished execution, the variables within that aren't returned go out
of scope and are dropped. So, `s` goes out of scope, but we are still trying to reference it! The 
solution is to return the `String` directly, rather than a reference to it.

### Rules!

* Can only have **either** one mutable reference, or any number of mutable references to a variable.
* References must always be valid.

## 4.3 - Slices

Slices allow you to reference a contiguous sequence of elements in a collection, rather than the 
collection as a whole. Like a reference, it does not have ownership. We can use the following 
notation:

```rs
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

`hello` and `there` are now references to a portion of the `String`. We create slices using a range
within brackets: `[start..end]`. `end` is one more than the last index in the slice.

`world` is essentially a variable that contains a `ptr` to the 6th index of the data pointed to by
`s`, and a `len` of 5`= end - start`. See the graphic below:

![`world` contains a pointer to the byte at index 6 with a length value of 5.](https://doc.rust-lang.org/book/img/trpl04-06.svg)

We will now see how they behave. The most important part to consider is that a slice is a reference,
so if the original is dropped, the slice is no longer valid.

* `[start..end]` syntax:
    * Can skip `start` if first index is `0`.
    * Can skip `end` if slice includes last element.
* Type of String Slice is `&str`.
* Slice is a reference and is not valid if the referenced variable does not exist (out of scope):

```rs
fn main() {
    let mut s = String::from("Hello there!");

    let word = first_word(&s);

    s.clear();

    // s no longer in scope. Reference (word) is invalid.
    println!("The first word is: {}", word);
}

// Type of String Slice is `&str`
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..]
}
```

```sh
$ cargo run
   Compiling slices v0.1.0 (learning_rust/4_Ownership/slices)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let word = first_word(&s);
  |                           -- immutable borrow occurs here
5 |
6 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first word is: {}", word);
  |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `slices` due to previous error
```

* String literals \& slices have the same Type: `&str`. We can re-write the first word function to
take `&str` as its argument.

```rs
fn main() {
    // Use string literal also &str type
    let s = "Hello there!";

    let word = first_word(&s);

    // s no longer in scope. Reference (word) is invalid.
    println!("The first word is: {}", word);
}

// Type of String Slice is `&str`
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..]
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/slices`
The first word is: Hello
```

### Other Slices

We can also take slices of arrays:

```rs
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
```

The above slice has type `&[i32]`.

We will see more about slices for arrays/vectors and other collections in chapter 8.
