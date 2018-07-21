extern crate kyber_support;

use kyber_support::Action;

fn main() {
    kyber_support::main(&[&Noop]);
}

struct Noop;

impl Action for Noop {
    fn name(&self) -> &'static str {
        "noop"
    }
}
