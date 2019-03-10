use hlvm::function::Fun;

fn main() {
    let mut main = Fun::new()
        .add_num("1")
        .add_str("a")
        .add_fun(
            Fun::new()
                .load("x").store("x")
                .load("y").store("y")
                .load("x")
                .load("y")
                .println()
                .println()
        )
        .call_from_stack();

    main.run();
}