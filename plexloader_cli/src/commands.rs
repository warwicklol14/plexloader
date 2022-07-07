mod login;
mod download;
mod info;

pub use login::Login;
pub use download::Download;
pub use info::Info;

pub use crate::utils::print_err;

pub trait CommandHandler {
    fn handle(&self) -> anyhow::Result<()>;
}

pub fn handle_command<T>(command: &T)
    where T: CommandHandler
{
    print_err(command.handle())
}
