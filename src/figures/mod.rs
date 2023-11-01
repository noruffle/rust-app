/* Traits mods */
mod trait_;


/* Struct mods */
mod struct_rectangle;
mod struct_circle;

/* Impl mods */
mod impl_rectangle;
mod impl_circle;

/* Use trait */
pub use self::trait_::Area;


/* Use struct */
pub use self::struct_rectangle::Rectangle;
pub use self::struct_circle::Circle;
