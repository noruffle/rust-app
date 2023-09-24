/* 
    1. Data Types
  1.1. Scalar Types
1.1.1. Integer Types: i32, u32 - number without a fractional component (without 1/2, 2/4 etc).
  Each variant can be either signed or insigned and has an explicit size.
  Signed and unsigned refer to whether it's possible for the number to be negative -
  - in other words, whether the number needs to have a sign with it (signed) or whether
  it weill only ever be positive and can therefore be represented without a sign (unsigned).
  It's like writing numbers on paper: when the sign matters, a number is shown with a plus sign
  or a minus sign; however, when it's safe to assume the number is positive, it's shown with no sign.
1.1.2. Floating-Point Types: f32, f64;
1.1.3. Numeric Operations: +, -, *, /, %;
1.1.4. The Boolean Type: bool;
1.1.5. The Character Type: char;

  1.2. Compound Types - groups multiple values into one type.
  Two primitive types: tuples and arrays.
1.2.1. The Tuple Type - is general way of grouping together a number of values with a variety
  of types into one compound type. Tuples have a fixed lengh: once declared, they cannot grow
  or shrink in size.

  We create a tuple by writing a comma-separated list of values inside parentheses. Each position
  in the tuple has a type, and the types of the different values in the tuple don't have to be the
  same. We're added optional type annotations. //1

1.2.2. The Array Type - a = [];
*/
fn main() {
  // 1
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  let tuple = (500, 5.5, 2);
  let (_x, _y, _z) = tuple;

  let _five_hundred = tup.0;
  let _six_point_four = tup.1;
  let _one = tup.2;

  /* You write an array’s type using square brackets with the type of each element, 
  a semicolon, and then the number of elements in the array, like so: */
  let a: [i32; 5] = [1, 2, 3, 4, 5];

  let _two = a[1];
}