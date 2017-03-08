mod parser;

use parser::lexer::Lexer;

fn main() {
    let input = "
func main.foo(x: int, y: int) -> int
    return x + y && true
end

foo_ref = *foo
foo = 133.3
";
    let mut lexer = Lexer::new(input);

    while let Some(t) = lexer.next_token() {
        println!("found: {:?}", t);
    }
}