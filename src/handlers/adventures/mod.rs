pub mod delete;
pub mod get;
pub mod insert;
pub mod list;
pub mod play_list;
pub mod tabs;
pub mod update;
pub mod version;

pub mod responses;

pub use delete::delete_adventure;
pub use get::get_adventure;
pub use insert::insert_adventure;
pub use list::list_adventures;
pub use play_list::play_list_adventures;
pub use tabs::tabs_adventures;
pub use update::update_adventure;
pub use version::version_update_adventures;
