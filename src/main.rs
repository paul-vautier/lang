use lang::Env;

use crate::lang::syntax::parse_language;

mod lang;

fn main() {
    let res = parse_language("(2 - 3 * 2 + 3) -2").unwrap().1;

    let interp = res
        .iter()
        .map(|expr| expr.interp(&mut Env::new()).unwrap_or_else(|_| panic!()))
        .collect::<Vec<_>>();
    interp.iter().for_each(|expr| println!("{}\n", expr))
}
