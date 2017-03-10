mod parser;
mod vm;

use parser::lexer::Lexer;

use vm::module::Module;
use vm::context::Context;

use vm::llvm_type::VMRepresentation;

fn lexer_debug() {
    let input = "
func foo(x: int, y: int) -> int
    return x + y && true
end

foo = 133.7 + \"I'm a string, hello\"
";

    println!("{}", input);

    let mut lexer = Lexer::new(input);

    while let Some(t) = lexer.next_token() {
        println!("found: {:?}", t);
    }

    println!("\n");
}

fn ir_debug() {
    let context = Context::new();
    {
        let module  = Module::new("strawberry", &context);

        println!("{}", module);
    }
    
    let val = "that rainbow kinda milk-based icecream";
    println!("{}", val.to_representation(&context));

}

fn main() {
    lexer_debug();
    ir_debug();
}