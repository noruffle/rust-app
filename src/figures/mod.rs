/* Traits mods */
mod trait_;

/* Struct mods */
mod struct_rectangle;
mod struct_circle;

/* Impl mods */
mod impl_rectangle;
mod impl_circle;

/* Use trait */
pub use {
  trait_::Area,
  struct_circle::Circle,
  struct_rectangle::Rectangle,
};