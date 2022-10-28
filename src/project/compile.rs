use crate::{cwd, replace::replacer::Replacer, Error};

pub fn compile_program() -> Result<(), Error> {
    let current_directory = cwd!()?;

    Replacer::from_path(&current_directory.join("main.alg"))?
        .replace_to_file(&current_directory.join("compiled.cpp"))?;

    Ok(())
}
