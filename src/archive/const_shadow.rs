/* CONSTANTS

  1. The "const" is immutable variables, constants are values that are bound to a name
    and are not allowed to change, but there are a few differences between constants and variables.

  1.1. Constants aren't just immutable by default - they're always immutable.
    You declare constants using the "const" keyword instead of the "let",
    and the type of the value must be annotated.

  1.2. Constants can be declared in any scope, including the global scope, which makes them
    useful for values that many parts of code need to know about.

  1.3. Constants may be set only to a constant expression, not the result of a value
    that could only be computed at runtime.

  1.4. Constants are valid for the entire time a program runs, whithin the scope
    in which they were declared. This property makes constants useful for values in application 
    domain that multiple parts of the program might need to know about, such as the maximum
    number of points any player of a game is allowed to earn, or the speed of light.

  1.5. Naming hardcoded values used throughout your program as constats is useful in conveying 
  the meaning of that value to future maintainers of the code. It also helps to have 
  only one place in your code you would need to change of the hardcoded value needed to be updated.
*/
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

/* SHADOWING

  1. The "Shadowing" is the process of creating a new variable with the same name as an existing 
    variable that's already defined in the same scope. This allows you to redefine the value
    of the existing variable.
*/
fn main() {
  let arr = vec![THREE_HOURS_IN_SECONDS];

  println!("{:?}", arr);

  let x = 5;

  let x = x + 1;

  println!("{x}");

  let mut a = "    ";

  let a = a.len();

  println!("{a}");
}