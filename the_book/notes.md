# Run After Container is Built

## access token
`export GITHUB_TOKEN=$PERSONAL_TOKEN_SECRET`

## rustlings submodule
`cd rustlings`

`cargo install --force --path .`


# Rust Key Concepts

## <div align="center">Ownership</div>
### Summary
Ownership is primarily a discipline of heap management:
 - All heap data must be owned by exactly one variable.
 - Rust deallocates heap data once its owner goes out of scope.
 - Ownership can be transferred by moves, which happen on assignments and function calls.
 - Heap data can only be accessed through its current owner, not a previous owner.

### What makes a Rust program safe/unsafe?
one way to think about safety is as the absence of undefined behavior
```rust 
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    read(x); // x isn't defined!
    let x = true;
}
```
 This example is unsafe and does not compile because `read(x)` expects `x` to have a value
 of type `bool`, but `x` doesn't have a value yet

 Rust's goal is to compile programs into efficient binaries that require as few runtime checks as possible. Therefore Rust does not check at runtime whether a variable is defined before being used. Instead, Rust checks at compile-time.

 ### What would happen if Rust allowed a rejected program to compile?
It would compile into something like the assembly code below.
`read` expects `edi` to be a boolean, which is either 0 or 1. But `edi` could be anything
at the time it is called (`2`, `100`, `0x1337BEEF`, ...) since the `mov` happens after
```asm
main:
    ; ...
    call    read
    mov     edi, 1
    ; ...
```
We have only defined behavior for `read` function when its argument is a boolean.
Some `behavior` **will happen, but it is `undefined`:
 - The code executes without crashing, and no one notices a problem.
 - The code immediately crashes due to a segmentation fault or another kind of operating system error.
 - The code executes without crashing, until a malicious actor creates the right input to delete your 
   production database, overwrite your backups, or otherwise cause problems

##
## ***A foundational goal of Rust is to ensure that your programs never have undefined behavior.***

About 70% of reported security vulnerabilities in low-level systems are caused by memory corruption, which is one form of undefined behavior.

A secondary goal of Rust is to prevent undefined behavior at compile-time instead of run-time. This goal has two motivations:
1. Catching bugs at compile-time means avoiding those bugs in production, improving the reliability of your software.
2. Catching bugs at compile-time means fewer runtime checks for those bugs, improving the performance of your software.

### Ownership as a Discipline for Memory Safety
Ownership is about safety which is the absence of undefined behavior The Rust Reference maintains a large list of ["Behavior considered undefined"](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) a main category being operations on memory.

There are many ways to think about memory - the RAM in your computer, an array of bytes, the pointers you get back from `malloc`. Those are *valid*, but not *useful* ways to think about how Rust works. 
Rust provides a particular way to think about memory - ownership is a discipline for safely using memory within that way of thinking.

### Variables Live in Frames in the Stack
A **frame** is a mapping from variables to values within a single scope, such as a function.
```Rust
fn main () {
    let n = 5;  // L1
    let y = plus_one(n);  // L3
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    // L2
    x + 1
}
```
- The frame for `main` at location L1 holds `n = 5`
- The frame for `plus_one` at L2 holds `x = 5`
- The frame for `main` at L3 holds `n = 5; y = 6`

Frames are organized into a **stack** of currently-called-functions. At L2, the frame for `main` sits above the frame for the called function `plus_one`.

![](notes_imgs/frame_stack_0.png "stack of frames")

 - In this example when the function is called, `n` gets its value copied from its slot in the `main` frame to a new slot in the `plus_one` frame.
 - At L2, the frame for `main` sits above the frame for `plus_one` and at L3, the called function's frame is gone.

After a function returns or a scope ends, Rust **deallocates** the function's or scope's frame (also called **freeing** or **dropping**).

### Boxes Live in the Heap
Copying data as in the previous example is not always ideal as it can take up a lot of memory. For example, the following copies an array with 1 million elements which causes the `main` frame to contain 2 million elements:

![](notes_imgs/frame_stack_1.png "2 million elements")

To transfer access to data without copying it, Rust uses **pointers** which is a value that describes a location in memory. One common way to make a pointer is to allocate memory in the **heap** - a separate region of memory not tied to a stack frame where data can live indefinitely:

![](notes_imgs/stack_heap_0.png "copy pointer")

`a` and `b` here are pointers and the statement `let b = a` copies the pointer from `a` to `b`. Notice `a` has been grayed out because it has been *moved*.

### Rust Does Not Permit Manual Memory Management

### A Box's Owner Manages Deallocation

### Variables Cannot Be Used After Being Moved



##
## <div align="center">References, Borrowing and Permissions</div>
### Summary


# git Miscellaneous

## Adding Fork as a Submodule and Making Commits
1. **Add Fork as a Submodule**:
   - In your main repository, add the fork as a submodule to include it as part of your project.
   - Command: `git submodule add <url-of-your-fork> path/to/submodule`

2. **Initialize and Update Submodule After Cloning**:
   - To initialize and pull changes from your fork to the submodule after cloning 
     - `git submodule init`
     - `git submodule update`

3. **Committing Changes Within the Submodule**:
   - Navigate to the submodule directory: `cd path/to/submodule`
   - Add and commit your changes: `git add .` and `git commit -m "Your message"`
   - Push changes to your fork: `git push`

4. **Updating Submodule Reference in Main Repository**:
   - After making changes in the submodule (fork), update the reference in your main repository.
   - `git add path/to/submodule`
   - `git commit -m "Update submodule reference"`
   - `git push`