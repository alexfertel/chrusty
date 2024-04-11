use parser::Parser;

mod dom;
mod parser;

fn main() {
    let mut a = Parser::new(
        "<div id=\"123\" data-src=\"abc\"     data-id=\"dđd\">
        zdfasdfsdf
        <div>
            List 2
        </div>
        <p>
            List 3 
        </p>
    </div>",
    );
    print!("{} {:?}", a.parse(), a)
}
