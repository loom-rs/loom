### 🏜️ Geko design document
This document describes syntax and semantics of the `Geko` programming language.

### 🦎 Overview
Geko is a dynamic-typed, friendly, lightweight programming language 
for math and games, designed to be simple, readable and easy-to-learn. 

### Goals
- Simple and lightweight
- Minimal and readable syntax
- Easy to learn and use

### Non-goals
- High-performance systems programming
- Complex features
- Unpredictable, implicit behaviour

### Reserved words
Best way to get a quick feel for a language's style is to see what words it uses. 
Here’s what Geko has:
```
for while in use class enum if else
return continue brrak as fun trait pick
```

### Comments
Geko supports block and line comments. Here is an example:
```
# Line comment
#[
  Block comment
]#
```

### Identifiers
Naming rules are simple. Identifiers should start with letter or underscore 
and may contain letters, digits and underscores.

Here is some examples:
```
abc123
abc
_abc
```

### Blocks
Block represents series of statements, like this:
```
{
  a := 5
  b := 2
  return a + b
}
```
Geko uses curly braces `{` and `}` to define blocks.
You can use blocks in control flow statements, functions and classes.

### Values
A value is the smallest unit of information that can be obtained 
by evaluating expressions or using literals. 

Every value has a type. Here is a table of value types:
| Data type | Description                                                               |   Rust representation            |
|-----------|---------------------------------------------------------------------------|----------------------------------|
| int       | integer number                                                            | `i64`                            |
| float     | floating-point number                                                     | `f64`                            |
| bool      | logical (bool) type: `true` or `false`                                    | `bool`                           |
| string    | text data                                                                 | `String`                         |
| callable  | represents reference to  any callable: function, native, bound, etc.      | `Rc<Function>`                   |
| class     | represents reference to class.                                            | `Rc<Class>`                      |
| enum      | represents reference to enum.                                             | `Rc<Enum>`                       |
| trait     | represents reference to trait.                                            | `Rc<Trait>`                      |
| instance  | represents reference to instance of the class.                             | `Rc<RefCell<Instance>>`          |
| null      | represents null value or `nothing`.                                       | `()`                             |
| module    | represents reference to the module.                                       | `Rc<RefCell<Module>>`            |
| any       | represents internal rusts `std::Any` variable                             | `Rc<RefCell<dyn std::any::Any>>` |

### Variables
Variables are mutable memory cells, where you can store any value you need.
Here is example how to define, assign variables:
```
a := 5 # define variable
a = 5 # assign variable
a := true # redefine variable
```
Geko supports variables shadowing, so you can redefine your variables.

### Operations
`Geko` supports following binary operations:
```geko
+ - * / % && & || | ^ > < == != >: >!
```

And following unary operations:
```
- !
```

### Control flow statements
Geko supports conditional logic like any other language.
The simplest branching statement is `if`. Here is an example:
```
if a > 5 {
  ...
} else if a < 5 {
  ...
} else {
  ...
}
```

Geko supports different kinds of loops. Loop executes code repeatedly.
Here is an example for the `while` loop:
```
while i < 1000 {
  ...
}
```
While loop repeats code until condition becomes falsey.

And examples for the `for` loop:
```
for i in 0..100 {
  ...
}

for i in list {
  ...
}
```
For loop iterates over list and repeats code every iteration.

### Functions
It's hard to write good code without thinking about breaking it down 
into smaller, reusable pieces. We call it functions.
Here is an example to define function in Geko:
```
fun hello(name) {
  putln("Hello, " + name)
}
```
Function parameters are enclosed in parens `(` and `)`.

Sometimes you need to pass the context of the outer scope to the inner one,
but doing this using params is a bad case. To solve this, Geko supports closures:
```
fun outer() {
  x := 0
  fun inner() {
    x += 1 # function can see outer scope!
  }
  inner() # x = 1
  return inner
}

inner := outer()
inner() # x = 2
```

It is also useful to pass a function as an argument to another function, 
or to store a function in a variable with a name different from the function itself.
In Geko, functions are first-class values, so you can do the following:
```
fun sum(a, b) {
  return a + b
}
a := sum
a(1, 2) # works fine!
```

But doing this every time creating a named function is just not practical.
Here is an example how to use anonymous functions:
```
fun do_stuff(a, f) {
  f(a)
}

do_stuff(1, fun(x) -> x + 1)
do_stuff(2, fun(x) {
  return x + 1
})
```

### Classes
Sooner or later, every developer will want to create their own data structure.
In Geko, you can use classes for this:
```
class Battery {
  fun init(self, energy) {
    self.energy := energy
  }
  fun charge(self, power) {
    if self.energy + power > 100 {
      self.energy = 100
    } else {
      self.energy += power
    }
  }
}

battery := Battery(90)
battery.energy = 70 # you can assign fields
battery.voltage := 2 # you can declare new fields
battery.charge(20) # you can call methods
battery.charge = fun(self, x) {
  self.x += 1
} # methods are also fields
```
A class is a blueprint for creating an object with the fields and methods you need.
You can see `self` param in methods. This parameter passes instance of class to your method.

### Enums
When writing programs, there are times when you need to store a kind of something.
Enums are well-suited for this purpose. Enum is a container that stores mapping: `Name` -> `Int`
Here is an example:
```
enum Direction {
  North, # 1
  South, # 2
  West,  # 3
  East   # 4
}

direction := Direction.North
direction = Direction.South

putln(direction) # prints `2`
```

### Traits
At some point, there are simply too many classes, and it becomes necessary 
to validate input values based on specific criteria. This is where traits come in.
Trait is a behaviour description that can be used to validate values.
Here is an example:
```
trait Dog {
  fun bark(self)
  fun feed(self, amount)
}

class Dalmatian {
  fun init() {
    self.food := 20
  }
  fun bark(self) {
    putln("Dalmatian barks!")
  }
  fun feed(self, amount) {
    self.food += amount
  }
}

dalmatian := Dalmatian()
if dalmatian >: Dog {
  putln("Dalmatian impls Dog!")
}
if 123 >! Dog {
  putln("Int doesn't impl Dog!")
}
```

### Usings
It's hard to maintain a code without breaking file into small modules. 
`Geko` is modular, every file is a module.
Here is an example how to use one file from another:
```
use "a" # import `a` as `a`
use "a" as b # import `a` as `b`
use "a" pick b # import `b` from `a` directly by `shallow copying` it
use "a" pick b, c # import multiple items
```

By default, import resolves module relative to current directory.
`Geko` also supports imports relative to current file with `@/` prefix:
```
use "@/a/b/c" # imports `a/b/c` relative to current file as `c`
```
