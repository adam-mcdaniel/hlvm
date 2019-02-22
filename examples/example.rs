use hlvm::function::{Fun, Value};

#[allow(dead_code, unused_variables)]
fn main() {

    let mut main = Fun::new()
        .add_list(&[
            Fun::new().add_str("Hello world!").println().as_value(),
            Fun::new().add_str("Im a function in a list!").println().as_value(),
            Fun::new().add_str("Look at me go!").println().as_value()
        ])
        .store("my_list")

        .add_fun(Fun::new()
            .load("my_list")
            .index_list()
            .call_from_stack()
        ).store("call_n")

        .add_num("0")
        .load("call_n")
        .call_from_stack()
        .add_num("1")
        .load("call_n")
        .call_from_stack()
        .add_num("2")
        .load("call_n")
        .call_from_stack();

    main.run();
}