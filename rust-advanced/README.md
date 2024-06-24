# RUSTikales Rust for advanced coders
## Overview
This subdirectory contains resources for my Rust course aimed towards advanced programmers.  
Each session is made out of those parts:
- original.pptx - Original PowerPoint presentation
- slides.pdf - Presentation exported as PDF
- Optionally: Some example projects to showcase something
  - For example, a small project to showcase macros
## Usage
`git clone` to get a local copy of this repository. Then regularly `git pull` to get the updated slides.  
Alternatively, you can browse it on Github, which also supports PDF and Markdown rendering.  
## Content
- [01. Introduction](./01%20-%20Introduction/)
  - Roadmap and Recap of Rust Basics
- [02. Lifetimes](./02%20-%20Lifetimes/)
  - What are Lifetimes?
  - Lifetime Elision
- [03. Slices](./03%20-%20Slices/)
  - What are Slices?
  - Efficient tokenization of Strings
- [04. Smart Pointers](./04%20-%20Smart%20Pointers/)
  - What are Smart Pointers?
  - `Box<T>`, `Rc<T>`, `RefCell<T>`
- [05. Declarative Macros](./05%20-%20Declarative%20Macros/)
  - `Weak<T>`
  - What are Macros?
  - `macro_rules!`
- [06. Procedural Macros](./06%20-%20Procedural%20Macros/)
  - tt-muncher
  - Introduction to `proc_macro`, `proc_macro_derive`, `proc_macro_attribute`
- [07. syn + quote](./07%20-%20syn%20+%20quote/)
  - Showcase of the Rust crates `syn` and `quote` for procedural macros