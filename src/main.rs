mod config;
mod exp;

fn main() {
    config::execute();

    let i = 32;
    exp::fizz_buzz::list(&i);
}
