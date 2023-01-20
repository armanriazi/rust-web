# Rust


## Commands

`time cargo run`

`cargo doc  --workspace --message-format short --no-deps --open --color always`

`cargo test --doc --workspace`

## Use-cases

> Hostile environments: In situations where **safety** is of utmost concern, Rust’s guarantees are a perfect fit.
- Concurrent
- Safe programming
- Processing
- Replacing legacy C or C++
>
> npm chose Rust to handle CPU-bound bottlenecks.

## Syntax
  
  > path`:	::
 
  > `!`: that’s known in type theory lingo as the empty type because it has no values. We prefer to call it the never type.Functions that return never are called diverging functions.

  
  > `?Sized`: T may or may not be Sized-Trait syntax with this meaning is only available for Sized, this notation overrides the default that generic types must have a known size at compile time. not any other traits.
  

## Safety

![safetay-control](./assets/images/Screenshot%20from%202022-12-14%2017-53-56.png)

> It guarantees that your program is memory-safe without imposing any runtime costs.

### Goal of Rust: Safety

> Rust programs are free from:

1. **Dangling pointers**— <u>_Live references_</u> to data that has become invalid over the course of the program (see [[ria-data-csv-bin]])

2. **Data races**—The inability to determine how a program will behave from <u>_run to run_</u> because external factors change (see [[ria-race]])

3. **Buffer overflow**—An attempt to access the 12th element of an <u>_array_</u> with only 6 elements (see listing 1.5)

4. **Iterator invalidation**—An issue caused by something that is iterated over after being <u>_altered midway_ through</u> (see listing 1.6)

## Memory model

### RAII

Rust uses [[RAII]] (resource acquisition is initialization) to keep track of when variables and all their references are in and out of scope. Once they are out of scope, memory can be released. The borrow checker will not allow references to out of scope variables, and it only allows one mutable reference or multiple immutable references, but never both.

### Explicit


### Rust programs have 3 memory 
(regions where data is stored)

#### data memory 

For data that is fixed in size and static (i.e. always available through life of program). Consider the text in your program (e.g. "Hello World!"): This text's bytes are only ever read from one place and therefore can be stored in this region. Compilers make lots of optimizations with this kind of data, and they are generally considered very fast to use since locations are known and fixed.

#### stack memory 
For data that is declared as variables within a function. The location of this memory never changes for the duration of a function call; because of this compilers can optimize code so stack data is very fast to access.

#### heap memory 
For data that is created while the application is running. Data in this region may be added, moved, removed, resized, etc. Because of its dynamic nature it's generally considered slower to use, but it allows for much more creative usages of memory. When data is added to this region we call it an allocation. When data is removed from this section we call it a deallocation. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.


> String struct is also on stack,but holds a reference to data on heap

```
  let s= Struct{x:y,z:w}  
```


## Numerous community tools 
(for improving code quality and productivity)

>rust-clippy, an advanced linter and style tool - This warns you of common mistakes and potential **code smells**. Clippy relies on **compiler plugins** that are marked as unstable, so it is available with nightly Rust only. With rustup, you can switch to nightly easily.
>
>rustfmt, an opinionated code formatter-It formats code according to conventions that are mentioned in the Rust style guide.
> rust-analyzer, full-featured IDE integration for the Rust language
> 
> **sccache**, a compiler cache for rustc


### std:prelude

To understand what is included in local scope by default(like try_into()), you should investigate the std:: [[prelude]] module. Its documentation is available online at [prelude](https://doc.rust-lang.org/std/prelude/index.html)


## Version

Internally, Cargo uses the **semver** crate for parsing the versions


## Glossery

[Rust Glossary](https://doc.rust-lang.org/nightly/reference/glossary.html)

[Rust Notation](https://doc.rust-lang.org/reference/notation.html)
