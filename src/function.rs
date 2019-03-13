extern crate hlvm_runtime;

#[macro_use]
pub use hlvm_runtime::literals::*;
pub use hlvm_runtime::value::Value;
pub use hlvm_runtime::object::Object;
use hlvm_runtime::stack::StackFrame;
use hlvm_runtime::object::Instruction::*;


pub type ForeignFunction = fn(Value) -> Value;


#[derive(Clone, Debug)]
pub struct Fun {
    body: Vec<Value>
}

impl Fun {
    pub fn new() -> Self {
        Self {
            body: vec![]
        }
    }

    pub fn return_self(&self) -> Self {self.clone()}

    pub fn define(instruction: Value) -> Self {
        Self {
            body: vec![instruction]
        }
    }

    pub fn add_data(&mut self, data: Value) -> Self {
        self.body.push(data);
        self.return_self()
    }

    pub fn add_fun(&mut self, fun: Self) -> Self {
        self.add_data(fun.as_value());
        self.return_self()
    }

    pub fn add_foreign_fun(&mut self, fun: ForeignFunction) -> Self {
        self.add_fun(
            Fun::new()
                .add_data(foreign_function(fun))
                .call_foreign_function()
        );
        self.return_self()
    }

    pub fn add_str(&mut self, s: &str) -> Self {
        self.add_data(string(s));
        self.return_self()
    }

    pub fn add_num(&mut self, n: &str) -> Self {
        self.add_data(num(n));
        self.return_self()
    }

    pub fn add_list(&mut self, l: &[Value]) -> Self {
        self.add_data(list(l));
        self.return_self()
    }

    pub fn add_obj(&mut self) -> Self {
        self.add_data(empty_obj());
        self.return_self()
    }

    pub fn call(&mut self, fun: Self) -> Self {
        self.add_data(fun.as_value());
        self.add_data(ins(Call));
        self.return_self()
    }

    pub fn call_from_stack(&mut self) -> Self {
        self.add_data(ins(Call));
        self.return_self()
    }

    pub fn call_foreign_function(&mut self) -> Self {
        self.add_data(ins(Execute));
        self.return_self()
    }

    pub fn store(&mut self, name: &str) -> Self {
        self.body.push(string(name));
        self.add_data(ins(Store));
        self.return_self()
    }

    pub fn get_parameter(&mut self, name: &str) -> Self {self.store(name)}
    pub fn store_variable(&mut self, name: &str) -> Self {self.store(name)}


    pub fn load(&mut self, name: &str) -> Self {
        self.body.push(string(name));
        self.add_data(ins(Load));
        self.return_self()
    }
    
    pub fn load_variable(&mut self, name: &str) -> Self {
        self.load(name)
    }

    pub fn disassemble(&mut self) -> Self {
        self.add_data(self.as_value());
        self.return_self()
    }

    pub fn as_value(&self) -> Value {
        let mut body = list(&[]);
        for expr in self.body.clone() {
            body.list_push(expr);
        }

        body
    }

    pub fn run(&mut self) {
        StackFrame::from_instructions(
            self.as_value()
        ).run();
    }

    pub fn debug(&mut self) {
        println!("===| DEBUG |===> main {}\n===[ START ]===>", self.as_value());
        StackFrame::from_instructions(
            self.as_value()
        ).run();
    }

    // pub fn print(&mut self) -> Self {self.call(Fun::define(ins(Print))); self.return_self()}
    // pub fn println(&mut self) -> Self {self.call(Fun::define(ins(Println))); self.return_self()}
    // pub fn add(&mut self) -> Self {self.call(Fun::define(ins(Add))); self.return_self()}
    // pub fn mul(&mut self) -> Self {self.call(Fun::define(ins(Mul))); self.return_self()}
    // pub fn sub(&mut self) -> Self {self.call(Fun::define(ins(Sub))); self.return_self()}
    // pub fn div(&mut self) -> Self {self.call(Fun::define(ins(Div))); self.return_self()}
    // pub fn not(&mut self) -> Self {self.call(Fun::define(ins(Not))); self.return_self()}
    // pub fn greater(&mut self) -> Self {self.call(Fun::define(ins(Greater))); self.return_self()}
    // pub fn less(&mut self) -> Self {self.call(Fun::define(ins(Less))); self.return_self()}
    // pub fn append_list(&mut self) -> Self {self.call(Fun::define(ins(Append))); self.return_self()}
    // pub fn pop_list(&mut self) -> Self {self.call(Fun::define(ins(Pop))); self.return_self()}

    pub fn print(&mut self) -> Self {
        self.add_data(ins(Print));
        self.return_self()
    }
    pub fn println(&mut self) -> Self {
        self.add_data(ins(Println));
        self.return_self()
    }

    pub fn add(&mut self) -> Self {
        self.add_data(ins(Add));
        self.return_self()
    }
    pub fn mul(&mut self) -> Self {
        self.add_data(ins(Mul));
        self.return_self()
    }
    pub fn sub(&mut self) -> Self {
        self.add_data(ins(Sub));
        self.return_self()
    }
    pub fn div(&mut self) -> Self {
        self.add_data(ins(Div));
        self.return_self()
    }
    pub fn not(&mut self) -> Self {
        self.add_data(ins(Not));
        self.return_self()
    }

    pub fn greater(&mut self) -> Self {
        self.add_data(ins(Greater));
        self.return_self()
    }
    pub fn less(&mut self) -> Self {
        self.add_data(ins(Less));
        self.return_self()
    }

    pub fn append_list(&mut self) -> Self {
        self.add_data(ins(Append));
        self.return_self()
    }
    pub fn pop_list(&mut self) -> Self {
        self.add_data(ins(Pop));
        self.return_self()
    }

    pub fn eq(&mut self) -> Self {
        self.add_data(ins(Equal));
        self.return_self()
    }

    pub fn index(&mut self) -> Self {
        self.add_data(ins(Index));
        self.return_self()
    }


    pub fn get_attr(&mut self) -> Self {
        self.add_data(ins(GetAttr));
        self.return_self()
    }
    pub fn set_attr(&mut self) -> Self {
        self.add_data(ins(SetAttr));
        self.return_self()
    }


    pub fn while_function(&mut self) -> Self {
        self.add_data(ins(While));
        self.return_self()
    }

    pub fn if_function(&mut self) -> Self {
        self.add_data(ins(If));
        self.return_self()
    }

    // pub fn if_function(&mut self) -> Self {self.call(Fun::define(ins(If))); self.return_self()}
    // pub fn if_function(&mut self) -> Self {self.call(Fun::define(ins(If))); self.return_self()}

    // pub fn if_function(&mut self) -> Self {
    //     let if_fun = Fun::new()
    //         .get_parameter("c")
    //         .get_parameter("a")
    //         .get_parameter("b")

    //         .load("c").store("d")
    //         .add_fun(
    //             Fun::new()
    //                 .load("a")
    //                 .add_num("0").store("d")
    //         )
    //         .add_fun(
    //             Fun::new()
    //                 .load("d")
    //         ).while_function()

    //         .load("c").store("d")
    //         .add_fun(
    //             Fun::new()
    //                 .load("b")
    //                 .add_num("1").store("d")
    //         )
    //         .add_fun(
    //             Fun::new()
    //                 .load("d")
    //                 .not()
    //         ).while_function()
    //         ;

    //     self.add_fun(if_fun);
    //     self.call_from_stack();
    //     self.return_self()
    // }
}
