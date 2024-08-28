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

 ```
 let message = "Hello, world!";  // type inferred by compiler and default immutable variable
 let message: &str = "Hello, world!";  // explicitly typed variable 
 let mut message = "Hello, world";  // mutable variable 

 fn compute_height (){
    let mut height = 120;
    height = height + 100;  // shadowing happening here, because we defined height as mutable variable 
 }
 ```

 ## Control statements in Rust 
 *loop*: A keyword used for an infinite loop, which can be exited using a break statement.

 syntax:
 ```
 loop {
    // logic
 }
 ```

 *while*: A conditional loop that continues as long as its condition is true.

 syntax:
 ```
  while i < 5 { 
    // logic
  }
 ```

 *for*: A loop that iterates through elements of a collection or range.

 *break*: A control flow keyword to exit the current innermost loop early.

 syntax:
 ```
 for i in 1..=10 {
    if i == 6 {
        break; // exits loop when i is 6
    }
 }
 ```

 *mutability*: The ability for a variable to have its value changed during runtime by marking it as mutable with the mut keyword

 syntax:
 ```
  let mut x = Some(None);
 ```

 *option*: A Rust enum type that can be either Some(value) or None, used to represent optional values.
 
 syntax:
 ```
 fn main(){
    let _target: Option<Option<()>> = Some(None); // Option in use
    println!("Hello, world");
 }
 ```

 *continue*: A control flow keyword to skip an iteration and move on to the next one in the same loop.

 syntax:
 ```
 for i in 1..=10 {
    if i%2 == 0 {
        continue;  // skip even numbers
    }
 }
 ```

 *if let*: A pattern matching construct that allows you to conditionally bind variables based on their match against a given value or pattern.

 syntax:
 ```
 let target = Some(42);
 if let Some(number)=target{
    // logic
 }
 ```

 *sum*: An enum type wrapper around Option__<T>__ which can be either Some(value) or None.

 *range*: Represents a sequence of numbers, often used in loops for iteration purposes.

 syntax:
```
    for i in 1..10 {}; // 1 to 9
    for i in 1..=10 {} // 1 to 10 inclusive range
```
 *shadowing*: A variable re-declaration with the same name but different value and/or scope within the same context.The new declaration creates a new binding. In Rust, this operation is called "shadowing" because the new variable shadows the previous variable. The old variable still exists, but you can't refer to it in this scope anymore.

 syntax:
 ```
 fn compute_height (){
    let mut height = 120;
    height = height + 100;  // shadowing happening here, because we defined height as mutable variable 
 }
 ```

 *match*: an equivalent of *switch* statement in other languages

 syntax:
 ```
 fn main(){
    let name = "Dawgg";
    
    // use of match expression to match against variable *name*
    match name {
        "Hello" => printl!("Hello, matched the name"),  // first case condition
        "Dawgg" => printl!("Dawgg, matched the name"),  // matching case condition
        _ => printl!("There were no match to the name"), // *_* is same as *default* case in other languages 
    }
 }
 ```
## Data types

### Tuples
A tuple is a grouping of values of different types collected into one compound value.

Elements in tuple can be accessed by using indexes.

syntax:
```
// Tuple of length 4
let tuple_e = ('A', 5i32, true, "Hello, world!"); // contains elements of type, *Char*, *i32*, *bool*, *&str*

// Accessing element from tuple
tuple_e.0  => A
tuple_e.3  => Hello, world!

```
### Structs
A *struct* is a type that's composed of other types. The elements in a *struct* are called fields. Like tuples, the fields in a *struct* can have different data types.

Rust supports three *struct* types: *classic structs*, *tuple structs*, and *unit structs*.

To work with structs in a Rust program, first you define the struct by name and specify the data type for each field. Then, you create an instance of the struct with another name. When you declare the instance, you provide the specific values for the fields.

Syntax:
```
// Classic struct with named fields
struct Student { name: String, level: u8, remote: bool }  // defining struct 

// Instantiate classic struct, specify fields in random order, or in specified order
let user_1 = Student { name: String::from("Hello, world!"), remote: true, level: 2 };

// Access value by field name
user_1.name => Hello, world!
user_1.remote => true
user_1.level => 2

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);

// Instantiate tuple structs, pass values in same order as types defined
let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);

// Access element from tuple by using index
mark_1.0 => A
mark_1.4 => 3.75

// Unit struct
struct Unit;

Unit structs are most commonly used as markers.

```

### Enums
Enums are types that can be any one of several variants. We use the enum keyword to create an enum type, which can have any combination of the enum variants. Like structs, enum variants can have named fields, but they can also have fields without names, or no fields at all. Like struct types, enum types are also capitalized.

syntax:
```
// Define a tuple struct
struct KeyPress(String, char);

// Define a classic struct
struct MouseClick { x: i64, y: i64 }

// Redefine the enum variants to use the data from the new structs
// Update the page Load variant to have the boolean type
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

```
#### Instantiate an enum
For each variant, we use the let keyword to make the assignment. To access the specific variant in the enum definition, we use the syntax *<enum>::<variant>* with double colons *::*.

syntax:
```
// Simple variant
let we_load = WebEvent::WELoad(true);

// Struct variant
// Instantiate a MouseClick struct and bind the coordinate values
let click = MouseClick { x: 100, y: 250 };

// Set the WEClick variant to use the data in the click struct
let we_click = WebEvent::WEClick(click);

// Tuple variant
// Instantiate a KeyPress tuple and bind the key values
let keys = KeyPress(String::from("Ctrl+"), 'N');
    
// Set the WEKeys variant to use the data in the keys tuple
let we_key = WebEvent::WEKeys(keys);

```







# Resources 
https://learn.microsoft.com/en-us/training
