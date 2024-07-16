mod arch;
mod current;
mod install;
mod list;
mod none;
mod uninstall;
mod use_version;

pub use arch::handle_arch;
pub use current::handle_current;
pub use install::handle_install;
pub use list::handle_list;
pub use none::handle_none;
pub use uninstall::handle_uninstall;
pub use use_version::handle_use_version;
