use pepser::traits::ParseResult;

use crate::lang::syntax::parse_language;

mod lang;

fn main() {
    let res = parse_language("3- 3 * 2 + 3; !3 :> 3 :: (3 - 3)*4")
        .unwrap()
        .1;

    res.iter().for_each(|expr| println!("{}\n", expr))
}
