use logos::Logos;
use samarium::Token;

fn main() {
    let mut lex = Token::lexer(r"\///\/++(/\/--\//\)");
    
    println!("{:#?}", lex);

    loop {
        let token = lex.next();
        if token.is_none() {break};
        println!("{:#?}", token.unwrap());
    }
    println!();
   
}
