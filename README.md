FeO (Iron Oxide)
================

FeO is a basic scripting language for Rust.

Structure
---------

The FeO JIT compiler is planned to have 3 parts:

* `parse`—this is where the parsing and tokenisation happens. Basically, source code is turned into an AST.
* `compile`—this is where the AST is turned into FeO bytecode.
* `execute`—this is where FeO bytecode is evaluated through the JIT.
* `feo`—this is the command-line tool used for compiling FeO programs.

Status
------

FeO is right now completely unusable.

Things already done:

* Nothing???

Things being worked on:

* Parsing

Things still remaining:

* `feo!(…)` macro
* Bytecode
* JIT execution

Examples
--------

```rust
let list = ["hello", 2, "dynamically-typed", 3.4, "world"];

fn concat(list) {
    let result = "";
    for item in list.iter() {
        result += item as str;
    }
}

print(concat(list)); // prints `hello2dynamically-typed3.4world`

class Cat {
    let colour;
    let miaow;

    fn new(self.colour, self.miaow) {}

    fn purr(self) {
        print("purr", self.miaow, "purr");
    }
}

let mr_fluffles = Cat::new("brown", "maaaaaow");
mr_fluffles.purr(); // prints `purr maaaaaow purr`
```