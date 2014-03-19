#[crate_id = "feo"];
#[crate_type = "lib"];

#[feature(globs, macro_rules)];

/*!
FeO — a fast scripting language for Rust

# Example

~~~ignore
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
~~~
*/

pub mod parser;

