## Create a new rust project using cargo
using *cargo init .* ---> initializes rust project using current directory as project name
or by the following command:
```
cargo new your_project_name

```
## Short notes

Rust uses keyword *fn* to define a function.

```
fn main(){
    println("Hello, world!");
}
```
## Variable declaration, scope , mutability 
 Variables are declared using keyword *let*.
 Variable can have type or leave it to compiler to infer the type.
 By default variables are immutable, we can make a variable mutable by adding *mut* keyword during declaration.
 shadowing, reassigning a variable to other value is possible only when a variable is mutable.

 ``
 let message = "Hello, world!";  // type inferred by compiler and default immutable variable
 let message: &str = "Hello, world!";  // explicitly typed variable 
 let mut message = "Hello, world";  // mutable variable 

 fn compute_height (){
    let mut height = 120;
    height = height + 100;  // shadowing happening here, because we defined height as mutable variable 
 }
 ``

 ## Control statements in Rust 
 *loop*: A keyword used for an infinite loop, which can be exited using a break statement.

 *while*: A conditional loop that continues as long as its condition is true.

 *for*: A loop that iterates through elements of a collection or range.

 *break*: A control flow keyword to exit the current innermost loop early.

 *mutability*: The ability for a variable to have its value changed during runtime by marking it as mutable with the mut keyword

 *option*: A Rust enum type that can be either Some(value) or None, used to represent optional values.

 *continue*: A control flow keyword to skip an iteration and move on to the next one in the same loop.

 *if let*: A pattern matching construct that allows you to conditionally bind variables based on their match against a given value or pattern.

 *sum*: An enum type wrapper around Option__<T>__ which can be either Some(value) or None.

 *range*: Represents a sequence of numbers, often used in loops for iteration purposes.

 *shadowing*: A variable re-declaration with the same name but different value and/or scope within the same context.





