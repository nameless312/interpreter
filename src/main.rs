use lexer::repl::Repl;

mod lexer;
fn main() {
    Repl::run().unwrap();
}
