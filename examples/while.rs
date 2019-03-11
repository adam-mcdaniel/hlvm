#[allow(unused_imports)]
use hlvm::function::{Fun, string, num, list};


#[allow(dead_code, unused_variables)]
fn main() {


    let mut main = Fun::new()
        .add_num("0")
        .store("a")
        .add_fun(
            Fun::new()
                .load("a")
                .add_num("1")
                .add().store("a")

                .add_str("#").print()
                .load("a").println()
        )
        .add_fun(
            Fun::new()
                .add_num("1000")
                .load("a")
                .less()
        )
        .while_function()
        .add_str("a is ")
        .print()
        .load("a")
        .println()
        ;

    main.run();
}