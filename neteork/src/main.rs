/**
 * Rustのファイルシステムについて
 * まず、以下がルートファイルとなる
 * ・src/main.rs
 * ・src/lib.rs
 *
 * コンパイルしたいファイルはこのルートファイルにmod宣言する必要がある。
 *
 * 参考: https://zenn.dev/newgyu/articles/3b4677b4086768
 */
mod interface_sample;
mod sample;

use crate::{interface_sample::InterfaceSample, sample::Sample};

fn main() {
    println!("Hi!");
    let p: Box<i32> = Box::new(1);
    let ptr: Box<Sample> = Box::new(Sample {
        size: 1,
        pointer: p,
    });
    ptr.intMethod(1);
}
