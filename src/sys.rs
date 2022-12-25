use users::get_current_username;

pub fn get_user() -> String {
  match get_current_username() {
    Some(uname) => {
      let user: &str = &format!("{:?}", uname);

      let mut trimmed_users = user.chars();
      trimmed_users.next();
      trimmed_users.next_back();

      trimmed_users.as_str().to_string()
    }

    None => {
      let trimmed_users = &"".chars();
      trimmed_users.as_str().to_string()
    }
  }
}