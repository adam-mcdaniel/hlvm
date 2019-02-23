use hlvm::function::Fun;

fn main() {
    let mut main = Fun::new()
        .add_fun(
            Fun::new()
                .load("f")
                .call_from_stack()
        ).store("f")

        .load("f")
        .call_from_stack();

    main.run();
}