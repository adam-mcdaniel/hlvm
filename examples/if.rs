use hlvm::function::Fun;

fn main() {
    let mut main = Fun::new()
        .add_str("y")
        .add_str("x")
        .add_num("0") // change me to one or zero!
        .if_function()
        .println()
        ;
        
    main.run();
}