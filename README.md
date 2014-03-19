FeO (Iron Oxide)
================

FeO is a basic scripting language for Rust.

Status
------

FeO is right now completely unusable.

Things already done:

* Nothing???

Things being worked on:

* Parsing

Things still remaining:

* Everything else

Examples
--------

```
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