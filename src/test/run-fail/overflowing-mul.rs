// error-pattern:thread 'main' panicked at 'attempt to multiply with overflow'
// compile-flags: -C debug-assertions

#![allow(overflow)]

fn main() {
    let x = 200u8 * 4;
}
