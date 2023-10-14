/* Traits mods */
pub mod trait_;

/* Functions mods */
pub mod fn_;

/* Struct mods */
pub mod struct_rectangle;
pub mod struct_circle;

/* Impl mods */
pub mod impl_rectangle;
pub mod impl_circle;

/* Use trait */
pub use self::trait_::Drawable;

/* Use fn */
pub use self::fn_::_draw_figures;

/* Use struct */
pub use self::struct_rectangle::Rectangle;
pub use self::struct_circle::Circle;