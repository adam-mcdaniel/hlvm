use hlvm::function::Fun;

fn main() {
	Fun::new().add_fun(Fun::new().println()).store("println")
		  .add_fun(Fun::new().add()).store("add")
                  .add_fun(Fun::new().mul()).store("mul")

.add_num("2").add_num("2").load("add").call_from_stack().load("println").call_from_stack().run()
}
