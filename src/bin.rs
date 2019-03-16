extern crate hlvm_runtime;
use hlvm_runtime::literals::*;
use hlvm_runtime::value::Value;

use hlvm::function::Fun;

#[allow(dead_code, unused_variables)]
fn main() {
    fn hello_world(v: Value) -> Value {
        println!("hello foreign functions!");
        none()
    }
    
    
    let mut decompose_list = Fun::new()
        .add_fun(
            Fun::new()
                .add_num("1")
                .add_str("test")
        )
        .run();

    // let mut main = Fun::new()
    //     .add_fun(Fun::new().print()).store("print")
    //     .add_fun(Fun::new().println()).store("println")
    //     .add_fun(Fun::new().add()).store("add")
    //     .add_fun(Fun::new().sub()).store("sub")
    //     .add_fun(Fun::new().mul()).store("mul")
    //     .add_fun(Fun::new().div()).store("div")
    //     .add_fun(Fun::new().less()).store("less")
    //     .add_fun(Fun::new().greater()).store("greater")
    //     .add_fun(Fun::new().eq()).store("eq")
    //     .add_fun(Fun::new().eq().not()).store("noteq")
    //     .add_fun(Fun::new().not()).store("not")
        
    //     .add_data(string("16 squared is ")).print()
    //     .add_data(num("16"))
    //     .call(square.clone())
    //     .println()

    //     .add_data(string("8 squared is ")).print()
    //     .add_data(num("8"))
    //     .call(square.clone()).println()

    //    	.add_data(none())
	// .add_foreign_fun(hello_world)
    //     .call_foreign_function()

    //     .add_data(string("the best language is ")).print()
        
    //     .add_data(string("python"))
    //     .add_data(string("rust"))
    //     .add_fun(
    //         Fun::new()
    //             .get_parameter("a")
    //             .call(
    //                 Fun::new()
    //                     .get_parameter("b")
    //                     .load("a")
    //             )
    //     ).store("true")
    //     .load("true").call_from_stack()
    //     .println();
    // main.run();


}
