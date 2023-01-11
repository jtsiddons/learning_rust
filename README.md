# Learning Rust

This is my repo for following the [rust book](https://doc.rust-lang.org/book/title-page.html) as I learn the rust programming language.

## What is rust?

> The Rust programming language helps you write faster, more reliable software. High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.

## Installation

I can run the install script. This provides `rustc` compiler, `cargo` package manager, `rustup` installer/upgrade utility.
```sh
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

Alternatively, since I am running on Arch Linux I can install via the distribution package manager:

```sh
sudo pacman -S rust rustup
```

Although this does not allow me to utilise `rustup` to upgrade rust, it still allows me to access other versions.

## Directory Structure

Each directory will refer to a chapter in the book. These will contain all the code files, and the notes markdown file for my notes on the chapter. If a chapter is large then it may get broken further into sub-directories for sections of the chapter.
