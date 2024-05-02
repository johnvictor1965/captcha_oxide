//! A high level async library that allows you to use the 2captcha API
//! to solve various types of captcha puzzles
//!
//! # Example
//! ```no_run
//! use captcha_oxide::{
//!   CaptchaSolver,
//!   captcha::types::recaptcha::v3::RecaptchaV3,
//!   Captcha,
//! };
//!
//! use url::Url;
//!
//! async fn example() -> captcha_oxide::Result<()> {
//!   let solver = CaptchaSolver::new("YOUR TWOCAPTCHA API KEY");
//!   
//!   let args = RecaptchaV3::builder()
//!     .website_url(Url::parse("https://someurl.com")?)
//!     .website_key("SITE_KEY")
//!     .min_score(0.3)
//!     .build();
//!   
//!   let solution = solver
//!     .solve(args)
//!     .await?
//!     .solution
//!     .g_recaptcha_response;
//!   
//!   assert!(!solution.is_empty());
//!   
//!   Ok(())
//! }
//! ```

#![deny(clippy::pedantic, clippy::nursery, clippy::mod_module_files)]
#![forbid(unsafe_code)]

pub(crate) const SOFT_ID: u16 = 4143;

pub mod captcha;
mod captcha_solver;
pub mod cookie;
mod language_pool;
mod prelude;
pub mod proxy;
mod two_captcha;

pub use captcha::Captcha;
pub use captcha_solver::CaptchaSolver;
pub use prelude::{Error, Result};