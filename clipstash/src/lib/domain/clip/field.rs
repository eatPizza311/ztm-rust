// declare submodule
mod clip_id;
// export structures into this crate so it can be direct access from outside
pub use clip_id::ClipId;

mod shortcode;
pub use shortcode::ShortCode;

mod content;
pub use content::Content;

mod title;
pub use title::Title;

mod posted;
pub use posted::Posted;

mod expires;
pub use expires::Expires;

mod password;
pub use password::Password;

mod hits;
pub use hits::Hits;
