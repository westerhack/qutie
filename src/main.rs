#![allow(dead_code, unused)]

#[macro_use]
mod macros;

mod parsing;
mod objects;

fn main() {
   let inp = "1+2";
   let mut stream = parsing::stream::Stream::from(inp);
   let mut frame = parsing::frame::new_root(&mut stream);
   frame.exec();
   println!("frame: {:?}", frame);
}