#![warn(clippy::pedantic)]
//! # Crate for interacting with the Smite API.
//! For possible endpoints and details, see the [official documentation](https://webcdn.hirezstudios.com/hirez-studios/legal/smite-api-developer-guide.pdfhttps://webcdn.hirezstudios.com/hirez-studios/legal/smite-api-developer-guide.pdf)
//! ## Example
//! ```rust
//! use smite::client::Client;
//! use smite::error::Result;
//!
//! fn example() -> Result<()> {
//!     let client = Client::new("your_developer_id".to_string(), "your_auth_key".to_string());
//!     let motds = client.get_motds()?;
//!     for motd in motds {
//!         println!("Motd {} at {:?}", motd.title, motd.start_date_time);
//!     }
//!
//!     Ok(())
//! }
//! ```
pub mod client;
pub mod entities;
pub mod error;
mod utils;
