mod parser;
mod vm;

use parser::lexer::Lexer;

use vm::module::Module;
use vm::context::Context;
use vm::builder::{
    Builder,
    BasicBlock
};

use vm::llvm_type::{
    VMRepresentation,
    int8_type,
    void_type,
    double_type,
};
use vm::function::Function;

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

fn function_debug() {
    let mut context = Context::new();
    let mut module = Module::new("strawberry", &context);
    let mut builder = Builder::new(&mut context);

    let function = Function::new(
        &module, "foo", &mut [
            int8_type(&context)
        ], double_type(&context),
    );

    let block: BasicBlock = function.new_basic_block("entry");
    builder.move_to_end(block);

    let summation = builder.add(
        13u8.to_representation(&context),
        37u8.to_representation(&context),
        "summation",
    );

    builder.return_value(summation);

    println!("{}", module);
}

fn main() {
    lexer_debug();
    function_debug();
}