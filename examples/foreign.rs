use hlvm::function::{Fun, Value};

fn main() {

        fn square(v: Value) -> Value {
                println!("squared");
                v.clone() * v.clone()
        }


        Fun::new()
                .add_num("2")
                .add_foreign_fun(square)
                .call_from_stack()
                .println()
                .run()
}