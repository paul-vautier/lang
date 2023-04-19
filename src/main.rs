use crate::lang::syntax::parse_language;

mod lang;

fn main() {
    println!("{:?}", parse_language("3 :> 3 :: 4"));
}
