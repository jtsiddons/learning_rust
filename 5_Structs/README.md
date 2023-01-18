# Chapter 5 - Using Structs to Structure Related Data

Can think of *Structs* as a custom data type. Similar to an object or class in an object oriented
programming language, for example a `Class` in python, or a `Struct` in Julia (although, these are 
not mutable objects in Julia). Structs and Enums are the building blocks for creating new data types 
in a program's domain. We'll see more about Structs in Chapter 6.

"hell"

## 5.1 - Defining and Instantiating Structs

### Structs

We define a Structs as follows:

```rs
struct User {
    active: bool,
    username: String,
    email: String,
    age: i8,
    sign_in_count: i64,
}
```

In this example, we define a User `struct` containing information about a user of our service. We 
can define a `User` as follows:

```rs
fn main() {
    let user1 = User {
        email: String::from("mycooluser@example.com"),
        username: String::from("mycoolusername"),
        active: true,
        sign_in_count: 23,
        age: 41,
    }

    println!("The users email address is {}", user1.email);
}
```

Note that we can also access values from the `Struct` with dot notation, for example in the above we
accessed the user's email address with `user1.email`. If we set `user1` to be mutable, we can change
their email address:

```rs
fn main() {
    let mut user1 = User {
        email: String::from("mycooluser@example.com"),
        username: String::from("mycoolusername"),
        active: true,
        sign_in_count: 23,
        age: 41,
    };

    println!("Email address is: {}", user1.email);

    user1.email = String::from("mynewemail@example.com");
    user1.sign_in_count += 1;

    println!("Email address is now: {}", user1.email);
    println!("User has signed on: {} times", user1.sign_in_count);
}
```

Returns:

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/defining`
Email address is: mycooluser@example.com
Email address is now: mynewemail@example.com
User has signed on: 24 times
```

Note that I get a warning related to the definition of the `User` struct, stating 
`fields 'active', 'username', and 'age' are never read` - this is because we never use these in the
above code example - the warning does not include the two variables that get modified and printed.

Rust does not allow us to make only some fields mutable. The entire instance must be mutable.

We can use functions to generate instances, for example:

```rs
fn new_user(username: String, email: String, age: i8) -> User {
    User {
        username: username,
        email: email,
        age: age,
        active: true,
        sign_in_count: 0
    }
}
```

Here, the function creates a new `User`, with some pre-defined values for `active` and 
`sign_in_count` - essentially acting as an initialisation function for new `User`s.

We can also create an instance of a Struct using values from another instance:

```rs
fn main() {
    let user1 = User {
        email: String::from("mycooluser@example.com"),
        username: String::from("mycoolusername"),
        active: true,
        sign_in_count: 23,
        age: 41,
    };

    let user2 = User {
        email: String::from("my_new_email@example.com"),
        ..user1
    };

    println!("{}", user2.email);
}
```

We user the `..` syntax, this is known as the update syntax. In the above I created a new user, who
is essentially `user1` but has a different email address. This could be an example of creating a new
user account for a user who lost their credentials - although not necessarily a good way of doing 
it. The `..user1` must come last, to specify any remaining values.

### Tuple Structs

We can also define Structs that look very similar to Tuples. Essentially, this acts as a way of 
providing a struct name to a Tuple. We don't have field names. An example could be RGB colours:

```rs
struct RGB(i8, i8, i8);

fn main() {
    let white = RGB(255, 255, 255);
    let blue = RGB(0, 0, 255);
}
```

### Unit Like Structs

These are Structs that have no fields. They behave similarly to `()`, an empty Tuple. Although they 
initially seem useless, they can be useful for implementing a trait on some type but don't have any
data you want to store in that type itself. We'll see more about traits in Chapter 10.

```rs
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

### Structs and Ownership

In the above examples, the `User` struct utilised `String` as opposed to `&str`. We want each 
instance of `User` to own all its data, and for the data to be valid whilst the entire struct is 
valid.

We can use `&str` and other forms of references in structs, however, we need to use *lifetimes*, 
which we'll see in Chapter 10.

```rs
struct User {
    username: &str,
    email: &str,
    age: i8,
    active: bool,
    sign_in_count: i64,
}

fn main() {
    let user1 = User {
        email: "mycooluser@example.com",
        username: "mycoolusername",
        active: true,
        sign_in_count: 23,
        age: 41,
    };
}
```

Does not compile and we get `missing lifetime specifier` errors:

```sh
$ cargo run
   Compiling defining v0.1.0 (learning_rust/5_Structs/defining)
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:15
  |
2 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:3:12
  |
3 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     username: &str,
3 ~     email: &'a str,
  |

Some errors have detailed explanations: E0106.
For more information about an error, try `rustc --explain E0106`.
error: could not compile `defining` due to 4 previous errors
```

## 5.2 - A Program with Structs

An example of refactoring code from simple variables through to utilising structs. This example is
computing the magnitude and angle from the origin of a 2d point with `x` and `y` components.

The project is stored in `./pythagoras/`.

### Initial code

Using `x`and `y` as variables:

```rs
fn main() {
    let x1 = 20.0;
    let y1 = 15.0;

    println!("Angle of point from origin is {}", angle(x1, y1));
    println!("Distance of point from origin is {}", length(x1, y1));
}

fn angle(x: f32, y: f32) -> f32 {
    return (y/x).atan()
}

fn length(x: f32, y: f32) -> f32 {
    return (x.powi(2) + y.powi(2)).sqrt()
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/pythagoras`
Angle of point from origin is 0.6435011
Distance of point from origin is 25
```


### Refactor to Tuple

Let's instead write our point as a Tuple.

```rs
fn main() {
    let point1 = (20.0, 15.0);

    println!("Distance of point from origin is {}", length(point1));
    println!("Angle of point from origin is {}", angle(point1));
}

fn angle(point: (f32, f32)) -> f32 {
    return (point.1/point.0).atan()
}

fn length(point: (f32, f32)) -> f32 {
    return (point.0.powi(2) +point.1.powi(2)).sqrt()
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/pythagoras`
Distance of point from origin is 25
Angle of point from origin is 0.6435011 
```

### Refactor using Struct

We can refactor using a `struct` for our point. Note that when we use functions on structs we need
to use a reference - i.e. if we don't use a reference then the function will take ownership of it
and it will go out of scope once the function has completed its execution.

```rs
struct Point2 {
    x: f32,
    y: f32,
}

fn main() {
    let point1 = Point2 {
        x: 20.0, 
        y: 15.0,
    };

    println!("Distance of point from origin is {}", length(&point1));
    println!("Angle of point from origin is {}", angle(&point1));
}

// Borrow the struct - don't own it!
fn angle(point: &Point2) -> f32 {
    return (point.y/point.x).atan()
}

fn length(point: &Point2) -> f32 {
    return (point.x.powi(2) + point.y.powi(2)).sqrt()
}
```

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/pythagoras`
Distance of point from origin is 25
Angle of point from origin is 0.6435011 
```

### Traits

If we try to use the `println!` macro on our struct we get an error:

```txt
`Point2` doesn't implement `std::fmt::Display`
```

That is because the `println!` macro uses the `Display` formatting output. Primitive types 
implement `Display` by default (such as float, int). However, because there are multiple possible
ways we could print our struct, `println!` has no formatting to use, and it does not attempt to 
guess. In fact, during my earlier experimentation refactoring the example to use Tuples, I saw that 
Tuples also do not implement the `Display` trait, and cannot be printed by `println!`.

The error suggests using `{:?}` to print our struct:

```rs
struct Point2 {
    x: f32,
    y: f32,
}

fn main() {
    let point1 = Point2 {
        x: 20.0, 
        y: 15.0,
    };

    println!("{:?}", point1);
}
```

Which results in the following error

```txt
`Point2` doesn't implement `Debug` the trait `Debug` is not implemented for `Point2` add `#[derive(Debug)]` to `Point2` or manually `impl Debug for Point2`
```

If we follow the advice and add `#[derive(Debug)]` before our `struct` definition:

```rs
#[derive(Debug)]
struct Point2 {
    x: f32,
    y: f32,
}

fn main() {
    let point1 = Point2 {
        x: 20.0, 
        y: 15.0,
    };

    println!("{:?}", point1);
}
```

We get:

```sh
$ cargo run
   Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/pythagoras`
Point2 { x: 20.0, y: 15.0 }

```

With the `#[derive(Debug)]` we can use the `dbg!` macro to print the output to `stderr` (rather than
`stdout`).

```rs
#[derive(Debug)]
struct Point2 {
    x: f32,
    y: f32,
}

fn main() {
    let scale = 3.0;
    let point1 = Point2 {
        x: 20.0, 
        y: dbg!(15.0 * scale),
    };

    dbg!(&point1);
    println!("{:?}", point1);
}
```

Results in the following output:

```sh
$ cargo new
   Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/pythagoras`
[src/main.rs:11] 15.0 * scale = 45.0
[src/main.rs:14] &point1 = Point2 {
    x: 20.0,
    y: 45.0,
}
Point2 { x: 20.0, y: 45.0 }
```

Notice how the output shows the computation made to calculate `y`. `dbg!` returns ownership of an 
expression, so the `y` field gets the same value regardless of the presence of `dbg!`. However, we
don't want `dbg!` to take ownership of `point`, so we provide a reference.

For a full list of traits that are available to use with the `derive` attribute see 
[here](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

## 3.3 - Structs and Methods

It would be more beneficial if we could tie our length and angle calculations more closely to the
`Point2` struct, since it is only related to that type. We can define *implementation* methods for our
structs using the `impl` block.

### Defining Methods

`impl` is short for *implementation*.

```rs
struct Point2 {
    x: f32,
    y: f32,
}

impl Point2 {
    fn angle(&self) -> f32 {
        return (self.y/self.x).atan();
    }    

    fn length(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt()
    }    
}

fn main() {
    let point1 = Point2 {
        x: 20.0,
        y: 15.0,
    };

    println!("The point is {} units from the origin.", point1.length());
    println!("The angle of the point to the origin is {}", point1.angle());
}
```

We add our methods as functions inside the `impl` block. Through experimentation, I discovered that
I can use multiple `impl` blocks, do define each method in a separate block if I wanted to:

```rs
struct Point2 {
    x: f32,
    y: f32,
}

impl Point2 {
    fn angle(&self) -> f32 {
        return (self.y/self.x).atan();
    }    
}

impl Point2 {
    fn length(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt()
    }    
}

fn main() {
    let point1 = Point2 {
        x: 20.0,
        y: 15.0,7
    };

    println!("The point is {} units from the origin.", point1.length());
    println!("The angle of the point to the origin is {}", point1.angle());
}
```

However, it is most effective to use a single `impl` block for each struct. This is for good 
organisation. This is also why we want to use methods as opposed to functions.

For each method definition argument we need to use `&self` - this is a reference to the instance of
the struct. We use a reference as we do not want the method to take ownership of the struct.

We can also use the same name for a method as one of the fields. We can still distinguish the use of
the method and access to the field itself, as the method requires the parentheses. The main use of 
this is as a *getter*, which simply returns the value of the field with which it shares its name.
Getters are not implemented automatically by rust.

We can also add other parameters to the arguments of a method.

### Associated Functions

All functions defined with an `impl` block are called *associated functions* - they are associated
with a type. We can also define functions in an `impl` block without a `self` as first param - and
hence are not methods, and don't need an instance of the struct to work with. One such example is 
the `String::from` function, which does not require a `String` as an argument.

Such associated functions are often used as *constructors*, these are often called `new`, but that 
is not required. Let's see an example:

```rs
struct: Point2 {
    x: f32,
    y: f32,
}

impl Point2 {
    fn equalPoint(x: f32) -> Self {
        x: x,
        y: x,
    }
}
```

So the `equalPoint` function creates a new `Point2` instance where the `x` and `y` fields are the
same. To call this function, we need to use the `::` notation with the type:

```rs
let point2 = Point2::equalPoint(11.7);
```
