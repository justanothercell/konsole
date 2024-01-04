pub(crate) fn next_key(getch: &Getch) -> Result<Command, std::io::Error> {
    todo!("unix command input");
    match getch.getch()? {
        c => Ok(Command::Unsupported(c))
    }
}