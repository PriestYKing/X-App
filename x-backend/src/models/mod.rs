pub mod media;
pub mod notifiication;
pub mod post;
pub mod user;
// Re-export commonly used structs for easier imports
pub use media::{CreateMedia, Media};
pub use post::{CreatePost, Post, PostQuery, PostWithDetails};
pub use user::{CreateUser, LoginUser, UpdateProfile, User, UserPublic};
