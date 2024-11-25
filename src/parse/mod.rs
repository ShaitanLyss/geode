pub mod quantity;
pub mod error;
pub mod range;


trait RawRepr {
   fn raw(&self) -> &str;
}
