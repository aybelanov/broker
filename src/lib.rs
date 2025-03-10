pub mod app;
pub mod config;
pub mod data;
pub mod models;
pub mod api;
pub mod auth;
pub mod macros;
pub mod common;

//#[macro_use]
extern crate actix_web;

// aliases
pub use data::rep::*;