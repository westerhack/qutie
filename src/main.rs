#![allow(dead_code, unused)]

#[macro_use]
mod macros;

mod parsing;
mod objects;

fn main() {
   let inp = "12 + 3 + 4";
   let mut stream = parsing::stream::Stream::from(inp);
   let mut frame = parsing::frame::Frame::new(stream);
   frame = frame.exec();
   println!("frame: {:?}", frame);
}