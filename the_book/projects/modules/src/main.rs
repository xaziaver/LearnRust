/* 
 - When compiling a crate, the compiler first looks in the crate root file usually,
   src/main.rs (binary crate) and src/lib.rs (library crate) for code to compile

 - In the crate root file, you can declare modules with `mod {module_name};`
   the compiler will look for the code in three places:
   1. Inline in curly braces, directly follwing the {module_name} before the semi-colon
   2. In the file src/{module_name}.rs
   3. In the file src/{module_name}/mod.rs

 - In any file other than the crate root, you can declare submodules with 
   the same syntax, `mod {submodule_name};` and the compiler will look:
   1. Inline
   2. In the file src/{module_name}/{submodule_name}.rs
   3. In the file src/{module_name}/{submodule_name}/mod.rs

 - Once a module is part of a crate, you can refer to code in that module from anywhere else
   in the same crate, as long as privacy rules allow, using the path to the code.
   ( e.g. crate::{module_name}::{submodule_name}::SomeType )

 - Code within a module is private from its parent modules by default, you must make it public
   by declaring with `pub mod` instead of `mod`. Items within a public module can be made public
   as well by using `pub` before their declarations.

 - Within a scope, the `use` keyword creates shortcuts to items. For example, if we write
   `use crate::{module_name}::{submodule_name}::SomeType;' then we only need to
   write `SomeType` to make use of that type.
*/

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
