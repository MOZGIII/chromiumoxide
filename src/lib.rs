//! A high-level API for programmatically interacting with the [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/).
//!
//! This crate uses the [Chrome DevTools protocol] to drive/launch a Chromium or
//! Chrome (potentially headless) browser.
//!
//! # Example
//! ```no_run
//! use futures::StreamExt;
//! use chromiumoxide::{Browser, BrowserConfig};
//!
//! #[async_std::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! pretty_env_logger::init();
//!
//!     let (browser, mut handler) =
//!         Browser::launch(BrowserConfig::builder().with_head().build()?).await?;
//!
//!     let handle = async_std::task::spawn(async move {
//!         loop {
//!             let _event = handler.next().await.unwrap();
//!         }
//!     });
//!
//!     let page = browser.new_page("https://en.wikipedia.org").await?;
//!
//!     // type into the search field and hit `Enter`,
//!     // this triggers a navigation to the search result page
//!     page.find_element("input#searchInput")
//!             .await?
//!             .click()
//!             .await?
//!             .type_str("Rust programming language")
//!             .await?
//!             .press_key("Enter")
//!             .await?;
//!
//!     // wait until the search result page is loaded and get the first hit
//!     let path = page
//!             .wait_for_navigation()
//!             .await?
//!             .find_element("li.mw-search-result a")
//!             .await?
//!             .attribute("href")
//!             .await?
//!             .unwrap();
//!
//!     // navigate to the page and get its html content
//!     let html = page
//!             .goto(format!("https://en.wikipedia.org{}", path))
//!             .await?
//!             .content()
//!             .await?;
//!
//!     handle.await;
//!     Ok(())
//! }
//! ```
//!
//! The [`chromiumoxide_pdl`] crate contains a [PDL
//! parser](chromiumoxide_pdl/src/pdl/parser.rs), which is a rust rewrite of a
//! [python script in the chromium source tree]( https://chromium.googlesource.com/deps/inspector_protocol/+/refs/heads/master/pdl.py) and a
//! [`Generator`](chromiumoxide_pdl/src/build/generator.rs) that turns the
//! parsed PDL files into rust code. The
//! [`chromiumoxide_cdp`](chromiumoxide_cdp) crate only purpose is to integrate
//! the generator during is build process and include the generated output
//! before compiling the crate itself. This separation is done merely because
//! the generated output is ~60K lines of rust code (not including all the Proc
//! macro extensions). So expect the compilation to take some time.
//!
//! The generator can be configured and used independently, see [`build.rs`] of
//! [`chromiumoxide_cdp`].
//!
//! [chromedp](https://github.com/chromedp/chromedp)
//! [rust-headless-chrome](https://github.com/Edu4rdSHL/rust-headless-chrome) which the launch
//! config, `KeyDefinition` and typing support is taken from.
//! [puppeteer](https://github.com/puppeteer/puppeteer)

#![warn(missing_debug_implementations, rust_2018_idioms)]

pub mod browser;
pub(crate) mod cmd;
pub mod conn;
pub mod element;
pub mod error;
pub mod handler;
pub mod keys;
pub mod layout;
pub mod page;

pub use crate::browser::{Browser, BrowserConfig};
pub use crate::conn::Connection;
pub use crate::element::Element;
pub use crate::handler::Handler;
pub use crate::page::Page;
