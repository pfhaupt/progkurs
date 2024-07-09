# RUSTikales Rust for beginners
## Overview
This subdirectory contains resources for my Rust course aimed towards beginners.  
Each session is made out of those parts:
- original.pptx - Original PowerPoint presentation
- slides.pdf - Presentation exported as PDF
- exercises.md - Exercises for that session
- solutions.md - Solutions for that session's exercises
- Optionally: Some example projects to showcase something
  - For example, a small project to showcase loops, or the Borrow Checker
## Usage
`git clone` to get a local copy of this repository. Then regularly `git pull` to get the updated slides.  
Alternatively, you can browse it on Github, which also supports PDF and Markdown rendering.  
Exercises are meant to be self-checks, possible solutions are given in the corresponding file.
## Content
- [01. Installation](./01%20-%20Installation/)
  - Setup of the Rust compiler, the Rust toolchain, and an IDE
- [02. Variables](./02%20-%20Variables/)
  - Basic Types
    - Integers
  - Variable Declaration
    - `let` vs `let mut`
- [03. Arrays and Vectors](./03%20-%20Arrays%20and%20Vectors/)
  - Basics of Arrays and Vectors
  - Indexing, Getting and Setting elements
  - `vec![]` macro
- [04. Loops](./04%20-%20loop,%20while,%20for/)
  - Conditions
  - `if`
  - `loop`, `while`, `for`
- [05. Ownership and Borrow Checker](./05%20-%20Ownership%20and%20Borrow%20Checker/)
  - Ownership
  - References
  - Borrow Checker
- [06. Function declaration](./06%20-%20Function%20declaration/)
  - Small introduction to Lifetimes
  - Function declarations
- [07. Function usage](./07%20-%20Function%20usage/)
  - Calling functions
  - Recursion
  - Ownership and Borrow Checker for arguments
- [08. Structs](./08%20-%20Structs/)
  - Structs
  - Associated Functions
  - Methods
- [09. Traits](./09%20-%20Traits/)
  - Traits
  - Strings
  - `format!()`, `println!()`
- [10. Enums](./10%20-%20Enums/)
- [11. Generics](./11%20-%20Generics/)
- [12. Option and Result](./12%20-%20Option%20and%20Result/)
- [13. Third-party libraries](./13%20-%20Third-party%20libraries/)