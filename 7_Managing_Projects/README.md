# Chapter 7: Managing Growing Projects - Packages, Crates, and Modules

This chapter will teach how to structure and manage projects. A main focus will be splitting code
into multiple modules and files. We will discuss the following features of cargo:

* **Packages** - lets you build, test, and share crates.
* **Crates** - Tree of modules that produces a library or executable.
* **Modules** - Let you control the organisation, scope, and privacy of paths.
* **Paths** - A way to name an item, such as a struct, function, or module.

These features are often referred to as the *module system*.

## 7.1 - Packages \& Crates.

A *crate* is the smallest amount of code that the Rust compiler considers at a time. Crates can
contain modules, which can be defined in other files that get compiled with the crate.

Can take two forms:

* *Binary* crate:
    * Compile to an executable that you can run.
    * **Must** have a `main` function.
* *Library* crate:
    * Define functionality intended to be shared across multiple projects.
    * For example the `rand` crate.
    * Does not have a `main` function.
    * Often use *crate* to mean *Library crate*.

The *crate root* is a source file which the Rust compiler starts from. A *package* is a bundle of
one or more crates that provide a set of functionality. Cargo itself is a crate. It contains the
binary crate for the `cargo` command as well as a library crate on which the binary depends.

A package can contain as many binary crates as you like, but at most **one** library crate.

## 7.2 - Modules - Control Scope and Privacy

We will discuss `module`s and other parts of the module system - in particular `path`s that allow us
to name items, `use` keyword to bring a path into scope, and `pub` keyword to make items public.

We'll also see the `as` keyword, external packages and the `glob` operator.

### Modules Cheat Sheet

* **Start from the crate root**: Compiler first looks in the crate root file (typically this is
`src/lib.rs` for a library crate or `src/main.rs` for a binary crate) for code to compile.
* **Declaring modules**: You declare new modules with `mod` in the crate root. For example, a "garden"
module would be declared with `mod garden`. The compiler will look for the module's code in the
following places:
    * Inline
    * `src/garden.rs`
    * `src/garden/mod.rs`
* **Declaring sub-modules**: Declared with `mod` in any file other than the crate root. For example
a "vegetable" sub-module of the "garden" module. The compiler will look in the following places:
    * Inline
    * `src/garden/vegetables.rs`
    * `src/garden/vegetables/mod.rs`
* **Paths to code in modules**: Once a module is part of the crate, you can refer to code from that
module anywhere within the crate - if privacy rules allow, using the code's path. For example, the
`Onion` type can be accessed with: `crate::garden::vegetables::Onion`.
* **Private and Public**: Code within a module is private by default and cannot be loaded. You can
set a module to be public by declaring it with `pub mod` instead of just `mod`. Make items within
the public module public by using `pub` in their declaration.
* **The `use` keyword**: Creates a shortcut to items - avoids using the full path every-time we need
to use an item. If we have `use crate::garden::vegetables::Onion` in a scope that can refer to
it then we can just use the shortcut `Onion`, rather than the full path. I'm thinking in R here,
I can refer to a function from a library without loading the library with the `library::my_func`,
but if I load the library then I can use `my_func`.

Consider the code in the `backyard` project in this directory. It has the following file tree:

```sh
$ tree
 .
├──  src
│  ├──  garden
│  │  └──  vegetables.rs
│  ├──  garden.rs
│  └──  main.rs
├──  Cargo.lock
└──  Cargo.toml
```

`main.rs` contains:

```rs
use crate::garden::vegetables::Onion;

pub mod garden;

fn main() {
    let plant = Onion {};
    println!("I'm growing {:?}!", plant);
}
```

`garden.rs` contains:

```rs
pub mod vegetables;
```

`garden/vegetables.rs` contains:

```rs
#[derive(Debug)]
pub struct Onion {}
```

Running the code results in:

```sh
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `learning_rust/7_Managing_Projects/backyard/target/debug/backyard`
I'm growing Onion!
```

### Grouping Related Code in Modules

Modules allow us to organise code within a crate for readability \& easy reuse. It also allows us to
manage the privacy of code - modules are private by default. Private items are internal
implementation details that are not available for outside use.

Consider the garden again, let's assume that we grow and harvest vegetables in our garden, these
activities are grouped together as gardening. We can imagine grouping smaller operations as part of
modules:

```rs
mod gardening {
    mod growing {
        fn sow() {}

        fn pot_on() {}

        fn feed() {}

        fn water() {}
    }

    mod harvesting {
        fn pick() {}

        fn cut() {}

        fn store() {}
    }
}
```

Note that we grouped together similar functions. We also are able to nest modules. The above code
might be placed in either `main.rs` or `lib.rs` in the `src` directory. Alternatively we could set
it up as multiple files in module directories like the example in the last sub-section.

## 7.3 - Paths for Referring to an Item in the Module Tree

Path can take two forms:

* **Absolute** path: The full path starting with the crate root. Begins with the crate name. For
code within the current crate then it starts with the literal `crate`.
* **Relative** path: Starts from the current module and uses `self` or `super`, or an identifier in
the current module.

This is somewhat similar to paths on the file-system. The separator here is `::` rather that `/` in
file-systems.

Here is an example. Contents of `lib.rs` in a `restaurant` project:

```rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Since the `add_to_waitlist` function is in the same crate as `eat_at_restaurant`, we can use the
relative path.

Note that the above code will not compile. We get an error message stating that `hosting` is
private:

```sh
$ cargo run
   Compiling restaurant v0.1.0 (learning_rust/7_Managing_Projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

We need to specify that `hosting` is public

```rs
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

The above code will still not compile. This time we get an error stating that `add_to_waitlist` is
private. We also need to make this function public:

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

The above code will now compile. Note that we did not need to make `front_of_house` public. In this
case the function `eat_at_restaurant` and module `front_of_house` are in the same module, i.e. they
are siblings. If they were defined in different modules then we would need to make `front_of_house`
public too.

### Best Practices for Packages with a Binary and a Library

A package can have both a `src/main.rs` binary crate root and a `src/lib.rs` library crate root.
Both crates will have the package name by default. Typically such packages would have just enough
code in the binary crate to start an executable that calls code from the library crate. This allows
for other projects to benefit from maximal functionality from the library crate, since its code can
be shared. 

Define the module tree in `lib.rs`. Then any public items can be used in the binary crate by
starting paths with the package's name. Essentially the binary crate is just a user of the library
crate - it can only use its public API.

### Paths with `super`

We can construct relative paths that begin in the parent module using `super`, this is similar to
`..` in file-systems.

```rs
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

In the `fix_incorrect_order` function, which is part of the `back_of_house` module, we refer to the
`deliver_order` function which is in the parent module. Rather than use an absolute path, we can use
a relative path with `super` which allows us to access items from the parent module.

### Making Structs \& Enums Public

We can make `struct`s and `enum`s public in the same way, however, we also need to make fields
public as required. We can have a mix of public and private fields.

```rs
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_breakfast() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    meal.seasonal_fruit = String::from("blueberries");
}
```

In the above code, we have a public struct: `Breakfast`, which is part of the `back_of_house`
module. It has a public field `toast` and a private field `seasonal_fruit`. The code above will not
compile because we try to access a private item `seasonal_fruit` in `eat_breakfast` function when
we try to change our breakfast choice. However, we can change our `toast` choice since that field is
public.

Notice too that we would not be able to create an instance of `Breakfast` without the public impl
function `summer` because we cannot otherwise set the value of `seasonal_fruit` which is a private
field of `Breakfast`.

In contrast, if we make an `enum` public, all of its fields are also made public. We only need to
specify `pub` when we declare the `enum`.

## 7.4 - Bringing Paths into Scope with `use`

We can use the `use` command to effectively create a symbolic link to a path, similar to how we
would do it in a file-system. This is like when we load a module in python using the `import`
command. 

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
} 

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

In the above code snippet we bring the `hosting` sub-module into scope with `use`. We can then
access the `add_to_waitlist` function with just `hosting::add_to_waitlist` rather than specify the
full path. We can utilise `use` to bring in a module, an item from a module.

This only brings the shortcut into scope where the `use` is in scope. If we move the
`eat_at_restaurant` into a different module it is no longer in the scope in which the shortcut
exists and we will no longer be able to use it:

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
} 

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

The above will fail to compile as the shortcut does not apply within the scope of the `customer`
module.

```sh
$ cargo run
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:56:9
   |
56 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` due to previous error; 1 warning emitted
```

Convention is to bring the parent module into scope rather than individual functions. This makes it
clear where a function has come from, as we need to specify its parent module. However, it is
convention to bring `struct`s and `enum`s into scope directly.

### Providing New Names with the `as` Keyword

Similar to loading modules in python, we can rename the symbolic link with the `as` keyword:

```rs`
use std::io::Result as IoResult;
```

We can now use `Result` by referring to `IoResult`. Similar to

```py
import pandas as pd
```

Where I would refer to an item within the pandas library with `pd.`. The dot notation in python is
`::` in Rust.

### Re-exporting Names with `pub use`

When we bring a name into scope with `use` it is private by default. We can make it public in the
standard way by using `pub use`. This is called *re-exporting* as we're bringing an item into scope
an allowing for other users to bring it into scope.

Without use of `pub use`, users would need to specify the full path to bring the items into scope.
However, if we have `pub use` then other uses can use the shortcut too. This is useful when the
internal structure of the code is different from how programmers calling you code would think about
the domain.

### Using External Packages

We need to add external packages to the `Cargo.toml` file, and call `use` to bring it into scope.
We saw this before in the guessing game when we added

```
rand = "0.8.5"
```

to our `Cargo.toml` file and then specified:

```rs
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

Adding the package to `Cargo.toml` tells Cargo to download the package and any dependencies from
[crates.io](crates.io), and make the package available to our project.

For packages in the `std` standard library, we do not need to add to our toml file, but we still
need to bring the package into scope with the appropriate `use`.

### Nested Paths

We can use curly braces to bring in multiple items from a path:

```rs
use std::io::{self, Write};
```

Will bring `std::io` and `std::io::Write` into scope.

### The `glob` Operator

Similar to file-systems, we can use the `*` operator to specify all items within the modules:

```rs
use std::collections::*;
```

Will bring all public items from `std::collections` into scope.

There is similar notation in python:

```py
from math import log
from os import *
```

## 7.5 - Separating Modules into Different Files

Recall the code in the `backyard` project in this directory. It has the following file tree:

```sh
$ tree
 .
├──  src
│  ├──  garden
│  │  └──  vegetables.rs
│  ├──  garden.rs
│  └──  main.rs
├──  Cargo.lock
└──  Cargo.toml
```

`main.rs` contains:

```rs
use crate::garden::vegetables::Onion;

pub mod garden;

fn main() {
    let plant = Onion {};
    println!("I'm growing {:?}!", plant);
}
```

`garden.rs` contains:

```rs
pub mod vegetables;
```

`garden/vegetables.rs` contains:

```rs
#[derive(Debug)]
pub struct Onion {}
```

Note that we only need to load a file with a `mod` declaration once in the module tree. Once the
compiler knows the file is part of the project (and where it is in the tree - from the `mod`
declaration), other files in the project should refer to the loaded code's file with a path to where
it was declared. `mod` is not an `include` operation like other languages.
