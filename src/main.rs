use std::fmt::Debug;

mod quiz1;
mod quiz2;
mod quiz3;

trait Loggable {
    fn log(&self) {}
}

impl<T: Debug> Loggable for T {
    fn log(&self) {
        println!("[LOG] {self:?}");
    }
}

fn main() {
    // You can optionally experiment here.

    vec![1, 2, 3].log();
}
