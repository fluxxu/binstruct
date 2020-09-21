pub mod serialize;
pub mod error;

pub use binstruct_codegen::*;

pub fn dump_hex<T>(data: T)
where
  T: AsRef<[u8]>,
{
  use pretty_hex::*;
  println!("{:?}", data.as_ref().hex_dump());
}