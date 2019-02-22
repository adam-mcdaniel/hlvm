extern crate runtime;
use hlvm_runtime::literals::*;
use hlvm_runtime::value::Value;

use hlvm::function::Fun;

#[allow(dead_code, unused_variables)]
fn main() {
    fn hello_world(v: Value) -> Value {
        println!("hello foreign functions!");
        none()
    }
    
    
    let square = Fun::new()
        .get_parameter("a")
        .load("a")
        .load("a").mul();

    
    let main = &mut Fun::new()
        .add_data(string("16 squared is ")).print()
        .add_data(num("16"))
        .call(square.clone())
        .println()

        .add_data(string("8 squared is ")).print()
        .add_data(num("8"))
        .call(square.clone()).println()

        .add_data(none())
        .call_foreign_function(hello_world)

        .add_data(string("the best language is ")).print()
        
        .add_data(string("python"))
        .add_data(string("rust"))
        .add_fun(
            Fun::new()
                .get_parameter("a")
                .call(
                    Fun::new()
                        .get_parameter("b")
                        .load("a")
                )
        ).store("true")
        .load("true").call_from_stack()
        .println();


    main.run();
}