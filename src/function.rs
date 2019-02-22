extern crate hlvm_runtime;

pub use hlvm_runtime::literals::*;
pub use hlvm_runtime::value::Value;
pub use hlvm_runtime::object::Object;
use hlvm_runtime::stack::StackFrame;
use hlvm_runtime::object::Instruction::*;

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

    pub fn call(&mut self, fun: Self) -> Self {
        self.add_data(fun.as_value());
        self.add_data(ins(Call));
        self.return_self()
    }

    pub fn call_from_stack(&mut self) -> Self {
        self.add_data(ins(Call));
        self.return_self()
    }

    pub fn while_function(&mut self, fun: Self) -> Self {
        self.add_data(fun.as_value());
        self.add_data(ins(While));
        self.return_self()
    }

    pub fn call_foreign_function(&mut self, fun: fn(Value) -> Value) -> Self {
        self.add_data(
            Value::from_foreign_function(fun)
        );
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
        self.println();
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

    pub fn print(&mut self) -> Self {self.call(standard::print()); self.return_self()}
    pub fn println(&mut self) -> Self {self.call(standard::println()); self.return_self()}
    pub fn add(&mut self) -> Self {self.call(standard::add()); self.return_self()}
    pub fn mul(&mut self) -> Self {self.call(standard::mul()); self.return_self()}
    pub fn sub(&mut self) -> Self {self.call(standard::sub()); self.return_self()}
    pub fn div(&mut self) -> Self {self.call(standard::div()); self.return_self()}
    pub fn append_list(&mut self) -> Self {self.call(standard::append()); self.return_self()}
    pub fn pop_list(&mut self) -> Self {self.call(standard::pop()); self.return_self()}
    pub fn index_list(&mut self) -> Self {self.call(standard::index()); self.return_self()}
}


#[allow(dead_code)]
pub mod standard {
    use super::*;
    pub fn print() -> Fun {Fun::define(ins(Print))}
    pub fn println() -> Fun {Fun::define(ins(Println))}

    pub fn add() -> Fun {Fun::define(ins(Add))}
    pub fn sub() -> Fun {Fun::define(ins(Sub))}
    pub fn mul() -> Fun {Fun::define(ins(Mul))}
    pub fn div() -> Fun {Fun::define(ins(Div))}

    pub fn append() -> Fun {Fun::define(ins(Append))}
    pub fn pop() -> Fun {Fun::define(ins(Pop))}
    pub fn index() -> Fun {Fun::define(ins(Index))}
}