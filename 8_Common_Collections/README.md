# Chapter 8 - Common Collections

This chapter discusses a number of useful data structures in `std`, these structures are called
*collections*. Collections are a data type that can contain more than a single value. They differ
from array and tuple types in that they are stored on the heap. This means that the amount of data
stored in the collection does not need to be known at compile-time, and can change during the time
the program runs. The three common collections that will be discussed in this chapter are:

1. *vector*
2. *string*
3. *hash map*

There are other forms of collection in `std`, details on these can be found in the documentation.

## 8.1 - Vectors

### Creating Vectors

We can create a new, empty vector with:

```rs
let v: Vec<i32> = Vec::new();
```

In this case we need to annotate the type, since rust does not know what kind of elements we will be
storing. If we wish to create a vector with initial values we can use the `vec!` macro:

```rs
let v = vec![1, 2, 3];
```

The above will create a new instance of `Vec<i32>` - we did not need to annotate the type as rust
can infer the data type. Recall that `i32` is the default integer type in rust.

### Updating Vectors

We can add to a vector with `push`:

```rs
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
```

In this case rust is able to infer the data type, since the values we are adding to the vector are
hard-coded. We don't need to annotate the `Vec<i32>`.

### Reading Values

There are two approaches to reading an element from a vector. The first is the standard index
approach, common to most languages:

```rs
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third value is {third}");
```

There is an issue that can occur here, if we try to access an element that does not exist - i.e. the
index is out-of-bounds, the program will panic \& throw an error. The second approach for accessing
values from a vector is more secure in this regard, since it returns an `Option`. This is the `get`
method:

```rs
let v = vec![1, 2, 3, 4, 5];

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element.");
}
```

Recall that rust is 0-indexed, so the third element has index value of 2. The benefit of the second
example is in the case of user input, if the user requests an element that does not exist we would
prefer the program to handle it, giving the user another attempt, rather than crash as a consequence
of a typo.

### Vectors and the Borrow Checker

The following code will not compile:

```rs
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {first}");
```

When a program has a valid reference it must obey the rules outlined by the borrow checker. The rule
states that we cannot have mutable \& immutable references (to the same object) in the same scope.
In this code we hold an immutable reference to the first element in vector, and then try to add an
element to the end of that vector. 

This may seem like behaviour we do not want, but it is safe. Recall that collections such as a
vector are stored on the heap. The reference to the first value is stored on the stack and points
to its location on the heap. When we change the vector we may need to change its location on the
heap - there may not be space to append to it in its current position. Rust stores elements of a
vector next to each other. This would mean that the pointer is no longer valid.

```sh
$ cargo test
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element of v is {first}");
  |                                          ----- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `vector` due to previous error
```

### Iteration

```rs
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```

The above loops through elements of vector to get immutable references to each element \& print
them. We can also loop over elements and make changes to them. For this we can use the *deference*
operator `*`, which follows the pointer to the value. We'll learn more about this operator in
chapter 15. Here is an example:

```rs
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

We need to use `*` before we can use `+=`.

Iterating over a vector is safe because of the borrow checker's rules. We would get a compile error
if we try to modify the whole vector in a loop.

### Vectors with Multiple Types with Enums

Vectors can only take values of the same type. If we want to store multiple types in the same vector
we can use an Enum:

```rs
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("Blue")),
    SpreadsheetCell::Float(3.14),
];
```

This works because the Enum `SpreadsheetCell` is its own type, and we can create a vector of that
type. If we do not know all the types of data that we wish to store in a vector then the Enum
approach will not work and we will need to use a trait object which we will see in Chapter 17.

## 8.2 - Strings

Strings in rust are more complicated than one would typically assume, at least in comparison to some
other languages. In rust, a string is implemented as a collection of bytes. The string type is
different from other collections, because of differences between how computers and people interpret
string data, particularly in the context of multiple languages - especially those of non-latin
origin. This means that indexing strings is somewhat complicated.

### `str` and `String`

In the core language, rust only has `str`, which is the *slice string* that is usually seen in
borrowed form `&str`. *String literals* are stored in the binary, and are therefore of type `str`.

`String` type is in `std`, rather than the core language, and is growable, mutable, owned, UTF-8
encoded. This section is largely about the `String` type.

### Creating a String

Since a `String` is implemented as a wrapper around a vector of bytes, many of the same operations
available to vectors can be utilised with `String`s.

```rs
let mut s = String::new();
```

Which creates an empty string, which we can load data into. If we have some initial data we wish to
initialise a string with we can use the `to_string` method:

```rs
let data = "let's get started";

let s = data.to_string();

// Or with a literal directly:
let s = "let's get started".to_string();
```

We can also use

```rs
let s = String::from("Let's get started");
```

As we have seen before.

### Updating Strings

Just like a Vector, a String can grow and change as the program runs. We can push to the end of a
String with `push_str` method, concatenate Strings with `+`, or use the `format!` macro:

```rs
let mut s = String::from("foo");
s.push_str("bar");
```

`push_str` takes a `str` as input, as we don't necessarily want to take ownership of the parameter,
for example we may want to use `s2` in the following example after appending it to `s1`:

```rs
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");
```

The `push` function for Strings can be used to push a single character to the String:

```rs
let mut s = String::from("lo");
s.push('l');
```

To join Strings together we can use the `+` operator to concatenate them:

```rs
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;
```

Note that we use a reference to `s2` - this is because the `+` operator uses the `add` method, the
signature of this method looks like:

```rs
fn add(self, s: &str) -> String {
    ...
}
```

This also means that `s1` is no longer valid, since it has been moved to `s3`, this is `self`
argument does not have a `&`, so the method takes ownership of `s1`. The method requires that the
argument(s) is(are) of type `&str`, we can only add a `str` to a `String`. However `s2` is a String,
not `str`. The compiler can *coerce* the `&String` into a `&str`, this is called a *deref coercion*
and transforms `&s2` into `&s2[..]` - a string slice. We'll see more about coercion in Chapter 15.

Furthermore, since the argument is `&str`, the method does not take ownership of `s2`, so we can use
it later in the program.

Concatenating multiple strings can become unwieldy:

```rs
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

An alternative, cleaner, and more readable approach is to use the `format!` macro, which functions
like the `println!` macro:

```rs
let s = format!("{s1}-{s2}-{s3}");
```

### Indexing Strings

Indexing of Strings in rust is more complicated than in many other languages. This is partially
because of UTF-8 encoding. This means that many characters are actually made up of multiple bytes,
which causes difficulties with the implementation of Strings as a Vector of bytes.

Consider:

```rs
let s1 = String::from("hello");
let h = s1[0];
```

This code will result in an error.

> `String` cannot be indexed by `{integer}`

Rust does not support indexing of Strings. Let's see why that is the case:

A String is a wrapper over `Vec<u8>`. If we ask rust of the length of `"Hola"` we get 4, as we would
expect, however if we ask for the length of `"Здравствуйте"` we actually get 24, when we would
intuitively expect it to return 12. In UTF-8 each Unicode scalar value is actually composed of two
bytes. Subsequently, indexing into the String's bytes will not return what we might be expecting and
won't necessarily correspond to the Unicode scalar value.

If we try to access the first element of `"Здравствуйте"` the result would not be `"З"` which is not
`3`. In UTF-8 the first byte is 208 and the second is 151. One would then expect to get 208, but
this does not represent a valid character. This is likely not want the user would want. To avoid
this, rust does not allow for indexing Strings by integer.

### Slicing Strings

Whilst we cannot index a String, we can attempt to extract a slice from a String:

```rs
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Will extract the first 4 bytes from the string. Each of these characters is 2 bytes, so `s` will be
`"Зд"`. If we were to try `&hello[0..1]` to extract only the first byte, rust would panic at runtime
just like if we were to index a String. The result would not be part of a valid character.

Using ranges to create slices of a String should be used with caution.

### Iteration of Strings

We can iterate over the bytes or the characters of a String:

```rs
for c in "Здравствуйте".chars() {
    println("{c}");
}

for b in "Здравствуйте".bytes() {
    println!("{b}");
}
```

This does not work with grapheme clusters from strings, for example from Devanagari script, since
this is much more complex. However there are crates for this functionality.

## 8.3 - Hash Maps (Dictionary)

This type of data structure is a key-value mapping. It stores a mapping of keys to values, the
syntax in rust is `HashMap<K, V>`, which maps keys of type `K` to values of type `V`. These are
useful for when you want to look up data by a key instead of an index. 

### Creating a HashMap

We can create an empty hash map with `new` and insert a key-value pair with `insert`:

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Note that we need to load the `HashMap` from `collections` into scope. This type is much less common
than vectors and strings and is not included in features automatically loaded into scope. They also
have less support from `std` - for example there is no macro to construct them. The example above is
an example of a `HashMap<String, i32>`. A hash map must have all keys of the same type, and all the
values must also all have the same type.

### Accessing Values

We can access a value from a hash map with `get`, which returns an `Option`.

```rs
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

In the above example, `get` attempts to get the value stored in `scores` with the key `team_name`.
This returns an `Option<&i32>`; if the team's name is not found it will return `None`. To get an
`Option<i32>` we use `copied()`, the final part `unwrap_or(0)` sets the score to 0 if `None` is
returned from the previous code.

We can loop through elements:

```rs
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

Note that a hash map is unordered, so the order in which the pairs are printed is arbitrary.

### Hash Maps and Ownership

If the type implements the `Copy` trait then the value will be copied into the hash map, however,
owned values, such as String, values are moved into the hash map.

```
use std::collections::HashMap;

let field_name = String::from("Favourite Colour");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
```

We can no longer use the variables `field_name` or `field_value` since they have been moved into
`map`. If we instead insert references to values into the hash map, the values won't be moved into
the hash map. The values that the references point to must remain valid for at least as long as the
hash map.

### Updating a Hash Map

#### Overwriting

Overwriting a value is simple. This is done with the `insert` method as before:

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

Will print `{"Blue": 25}`. The original value is overwritten.

#### Adding a Pair If and Only If a Key isn't Present

We can use the `or_insert` method after using the `entry` method. `entry` returns an `Option`, the
`or_insert` method then adds the key-value pair if `entry` returns `None`.

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

Will print `{"Yellow": 50, "Blue": 10}`. Since key `"Yellow"` does not exist it is added by
`or_insert(50)`. The key `"Blue"` already exists so `or_insert(50)` does not change its value.

#### Updating Based on Previous Value

This is a use case for hash maps, for example counting how many instances of each value in a list.
As with vectors we use the *deference* operator `*` for this:

```rs
use std::collections::HashMap;

let text = "Hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

Will print `{"world": 2, "hello": 1, "wonderful": 1}`. The `or_insert` method returns a mutable
reference to the value for the specified key, which we store in count. We can then update it's
value.


### Hashing Function

`HashMap` implements the `SipHash` algorithm. This isn't the fasted hashing algorithm, but provides
resistance to DoS attacks that involve hash tables. You can change the algorithm by specifying a
different hasher - a type that implements the `BuildHasher` trait.
