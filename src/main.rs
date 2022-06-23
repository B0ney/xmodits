mod it;
mod xm;
mod s3m;

mod wav;
mod utils;
fn main() {
    println!("Hello, world!");
}
#[test]
fn test() {
    let a = [
        4, (1 * 12), 1,1,1,1,
        2, 1,  1,1,1,1,1,
        2, 1,1, (1 * 26),
        1,1,1,1,  2, (1 * 240),
        (7 + (1 * 25 * 3))*3, (1)*4    ];
    let mut offset = 0;
    for i in 0..a.len() {
        println!("0x{:04X} => ", offset);
        offset += a[i];
    }
}