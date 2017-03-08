mod parser;

use parser::lexer::Lexer;

fn main() {
    let input = "
func foo(x: int, y: int) -> int
    return x + y
end
";
    let mut lexer = Lexer::new(input);

    while let Some(t) = lexer.next_token() {
        println!("found: {:?}", t);
    }
}