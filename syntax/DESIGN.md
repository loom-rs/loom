## 🪭 Syntax examples

This document describes syntax of the `Geko` programming language.

### 🎨 Data types
| Data type | Description                                                               |   Rust representation            |
|-----------|---------------------------------------------------------------------------|----------------------------------|
| int       | integer number                                                            | `i64`                            |
| float     | floating-point number                                                     | `f64`                            |
| bool      | logical (bool) type: `true` or `false`                                    | `bool`                           |
| string    | text data                                                                 | `String`                         |
| callable  | represents reference to  any callable: function, native, bound, etc.      | `Rc<Function>`                   |
| class     | represents reference to the class.                                        | `Rc<Class>`                      |
| enum      | represents reference to enumeration.                                      | `Rc<Enum>`                       |
| instance  | represents reference to instance of the type.                             | `Rc<RefCell<Instance>>`          |
| null      | represents null value or `nothing`.                                       | `()`                             |
| module    | represents reference to the module.                                       | `Rc<RefCell<Module>>`            |
| any       | represents internal rusts `std::Any` variable                             | `Rc<RefCell<dyn std::any::Any>>` |

### Variable declaration
`Geko` does not support variables shadowing, so here's
a way to define variable and to reassign it.

Variable definition:
```geko
id := value
```

Variable assignment:
```
id = value
```

### Binary operations
`Geko` supports following binary operations:

```geko
+ - * / % && & || | ^ > < == != >: >!
```

### Unary operations
`Geko` supports following unary operations:

```
- !
```

### Compound operators
`Geko` supports following compound operators:

```
id += value
id -= value
id *= value
id /= value
id %= value
id &= value
id |= value
```

### Value examples
Examples of the values:

| Data type | Example of the value        |
|-----------|-----------------------------|
| int       | 123                         | 
| float     | 123.456                     |
| bool      | true / false                |
| string    | "text"                      |
| function  | fun(x, y) { return x + y } |
| class     | AnyDeclaredClass            |
| enum      | AnyDeclaredEnum             |
| instance  | AnyDeclaredClass()          |
| null      | null                        |
| native    | declared native             |
| module    | module                      |
| any       | any_native_value            |

### Functions example
Here's an example on how you can define function in `Geko`:

```geko
fun fib(x) {
  if x <= 1 {
    return x
  } else {
    return fib(x - 1) + fib(x - 2)
  }
}
```

Geko supports closures:

```geko
fun a() {
  x := 1
  fun b() {
    x += 1
  }
  b() # x = 2
  return b
}

b := a()
b() # x = 3
b() # x = 4
b() # x = 5
```

### Classes or custom data types
`Geko` supports custom data types. Here is example:

```geko
class Dog {
  fun init() {
    self.food := 3
    self.water := 3
  }
  fun get_food() {
    return self.food
  }
}

dog := Dog()
a := dog.get_food()
b := dog.food
# a == b
```

### Comments
`Geko` comments examples:

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
`Geko` is modular:

```
use "a" # import `a` as `a`
use "a" as b # import `a` as `b`
use "a" pick b # import `b` from `a` directly by `shallow copying` it
use "a" pick b, c # import multiple items
```

`Geko` supports relative imports with `@/` prefix:
```
use "@/a/b/c" # imports `a/b/c` relative to current file as `c`
```

### Loops
`Geko` loops examples:

For loop with range examples.
You can use any expression instead of numbers in range.
```
for i in 0..100 {
  putln(i)
}

for i in 100..0 {
  putln(i)
}

for i in 0..=100 {
  putln(i)
}

for i in 100..=0 {
  putln(i)
}
```

While loop examples. You can see, that `Geko` supports `continue` and `break` keywords
```
i := 0
while true {
  if i == 100 {
    continue
    i -= 200
  }
  i += 1
  if i == -199 {
    break
  }
}
```

### Logical statements
If examples:

```geko
use convert

a := readln()
if convert.int(a) > 5 {
  ...
} else if convert.int(a) < 5 {
  ...
} else {
  ...
}
```

### Errors raising
Bail immediately breaks execution with error:

```geko
bail "some text"
```

### Anonymous function
`Geko` supports anonymous functions:

```geko
a := fun() -> 1
b := fun(a) -> a + 1
c := fun(a) {
  return a + 1
}
putln(a())
putln(b(1))
putln(c(2))
```

### Enumerations
`Geko` supports enumerations. Every variant is just an int variable inside `Dog`.

```geko
enum Dog {
  Poodle,  # 0
  Bulldog, # 1
  Beagle,
  Husky
}

dog := Dog.Poodle
putln(dog == 0) # true
putln(dog == Dog.Beagle) # false
```

### Traits
`Geko` supports traits. Trait represents a behaviour description
that classes can implemenet.

```geko
trait Pet {
  fun feed(self, amount)
}

class Cat {
  # Cat has method `feed`
  fun feed(self, amount) {
    self.food = amount;
  }
}

class Toad {
  # Toad doesn't have method `feed`, but `stroke`
  fun stroke(self) {
    println("Croak! Croak!");
  }
}

cat := Cat();
if cat >: Pet {
  println("`Cat` impls `Pet`");
}

toad := Toad();
if toad >! Pet {
  println("`Toad` doesn't impl `Pet`");
}
```
