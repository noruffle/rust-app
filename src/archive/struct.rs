fn main() {
  let _ruffle = User {
    is_admin: true,
    active: true,
    username: String::from("Ruffle"),
    email: String::from("{}@email.com"),
    sign_in_count: 1,
  };
  let _black = Color(0, 0, 0);

  let _phoks = build_user(_ruffle.email, String::from("Phoks"));
}

struct User {
  is_admin: bool,
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
      is_admin: false,
      active: true,
      username,
      email,
      sign_in_count: 1,
    }
}