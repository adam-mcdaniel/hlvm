use hlvm::function::Fun;

fn main() {

    let mut main = Fun::new()
        .add_str("hello world!")
        .println();

    main.run();
}