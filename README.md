# kyber

Interface for file transformations, mostly code

## WIP

This project is still in progress and not usable at the moment. All of the
following is examples of how it will be used.

## Usage

```sh
cargo install kyber

kyber options src/main.rs 3:7 # let [f]oo = "foo";
# rust-refactoring/rename
# rust-refactoring/make_constant

kyber do rust-refactoring/rename src/main.rs 3:7
# Needs argument (rename to): bar
```

Before

```rs
fn main() {
  let foo = "foo";
  println!("foo is: {}", foo);
}
```

After

```rs
fn main() {
  let bar = "foo";
  println!("foo is: {}", bar);
}
```

## Editor integration

Kyber is intended primarily to be used by text editor plugins. Separating the
interface to perform the modifications from the contents of the modification
lets new editors use existing transformations, and existing editors use new
transformations.
