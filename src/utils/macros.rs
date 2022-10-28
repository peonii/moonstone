#[macro_export]
macro_rules! cwd {
    () => {
        std::env::current_dir()
    };
}

#[macro_export]
macro_rules! home {
    () => {
        match home::home_dir() {
            Some(abcdefgh) => abcdefgh,
            None => {
                return Err("Can't get home dir!".into());
            }
        }
    };
}
