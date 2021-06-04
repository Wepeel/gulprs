#![feature(async_stream)]
mod stream;
use std::stream::Stream;
use stream::GulpStream;

fn main() {
    let s = GulpStream::<usize>::new();
}
