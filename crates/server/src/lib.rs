mod error;
pub use error::*;

pub mod app;
pub mod app_config;
pub mod cli;
pub mod controllers;
pub mod entities;

pub mod migrations;
pub mod models;
pub mod services;
pub mod utils;
pub mod web_server;
pub mod worker;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
