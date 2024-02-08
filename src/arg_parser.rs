use std::env::args;

pub fn get_application_args() -> Result<(String,), String> {
    let mut args = args();
    args.next().ok_or("No executable file")?;
    let filename = args.next().ok_or("No filename passed")?;
    Ok((filename,))
}
