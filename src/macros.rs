macro_rules! error_email (
  ($e:expr) => (match $e {
    Ok(ok) => ok,
    Err(err) => {
      let error = err.to_string();
      email::send("Error", &error).expect("[email] send");
      panic!("{}", &error);
    }
  })
);
