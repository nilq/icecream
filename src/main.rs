mod parser;
mod vm;

use parser::lexer::Lexer;

fn main() {
    let input = "
func main.foo(x: int, y: int) -> int
    return x + y && true
end

foo = 133.7 + \"I'm a string \"
";
    let mut lexer = Lexer::new(input);

    while let Some(t) = lexer.next_token() {
        println!("found: {:?}", t);
    }
}