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
