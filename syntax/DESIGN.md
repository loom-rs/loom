## 📐 Syntax examples

This document describes syntax of the `tick` programming language.

### Data types

| Data type |   Rust representation   |
|-----------|-------------------------|
| int       | `i64`                   |
| decimal   | `f64`                   |
| bool      | `boo l`                 |
| string    | `String`                |
| function  | `Rc<Function>`          |
| meta type | `Rc<Type>`              |
| instance  | `Rc<Instance>`          |
| null      | `()`                    |
| native    | `Rc<Native>`            |
| module    | `Rc<Module>`            |
| any       | `Rc<dyn std::any::Any>` |

### Variable declaration
`tick` does not support variables shadowing, so here's
a way to define variable and to reassign it.

Variable definition:
```tick
let id = value;
```

Variable assignment:
```
id = value;
```

### Binary operations
`tick` supports following binary operations:

```tick
+ - * / % && & || | ^ > < == !=
```

### Unary operations
`tick` supports following unary operations:

```
- !
```

### Compound operators
`tick` supports following compound operators:

```
id += value;
id -= value;
id *= value;
id /= value;
id %= value;
id &= value;
id |= value;
```

### Value examples
Examples of the values:

| Data type | Example of the value       |
|-----------|----------------------------|
| int       | 123                        | 
| decimal   | 123.456                    |
| bool      | true / false               |
| string    | "text"                     |
| function  | fn(x, y) {} return x + y } |
| meta type | AnyDeclaredType            |
| instance  | AnyDeclaredType()          |
| null      | null                       |
| native    | declared native            |
| module    | module                     |
| any       | any_native_value           |

### Functions example
Here's an example on how you can define function in `tick`:

```tick
fn fib(x) {
  if x <= 1 {
    return x;
  } else {
    return fib(x - 1) + fib(x - 2);
  }
}
```

Tick supports closures:

```tick
fn a() {
  let x = 1;
  fn b() {
    x += 1;
  }
  b(); // x = 2
  return b;
}

let b = a();
b(); // x = 3
b(); // x = 4
b(); // x = 5
```

### Classes or custom data types
Tick supports custom data types. Here is example:
```tick
type Dog {
  fn init() {
    self.food = 3;
    self.water = 3;
  }
  fn get_food() {
    return self.food;
  }
}
let dog = Dog();
let a = dog.get_food();
let b = dog.food;
# a == b
```

### Comments
Tick comments examples:

```
#[
Here is multiline 
comment in 
square
brackets
]#
```

```
# Here is single line comment
```

### Usings
Tick is modular:
```
use a # import `a` as `a`
use a as b # import `a` as `b`
use a for b # import `b` from `a` directly by `shallow copying` it
use a for b, c # import multiple items
```

### Loops
Tick loops examples:

For loop with range examples.
You can use any expression instead of numbers in range.
```
for i in 0..100 {
  println(i);
}

for i in 100..0 {
  println(i);
}

for i in 0..=100 {
  println(i);
}

for i in 100..=0 {
  println(i)
}
```

While loop examples. You can see, that `tick` supports `continue` and `break` keywords
```
let i = 0;
while true {
  if i == 100 {
    continue;
    i -= 200;
  }
  i += 1;
  if i == -199 {
    break;
  }
}
```

### Logical statements
If examples:
```tick
let a = scan();
if int(a) > 5 {
  ...
} else if int(a) < 5 {
  ...
} else {
  ...
}
```

### Errors raising
Bail immediately breaks execution with error:
```
bail "some text"
```

### Anonymous function
Tick supports rust-like anonymous functions:
```
let a = || 1;
let b = |a| a + 1;
let c = |a| {
    return a + 1;
};
println(a());
println(b(1));
println(c(2));
```
