use crate::lang::syntax::parse_language;

mod lang;

fn main() {
    println!("{:?}", parse_language("\"_asdasd332_3232'\""));
}
