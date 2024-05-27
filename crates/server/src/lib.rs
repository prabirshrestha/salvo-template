mod app_config;
pub use app_config::*;

pub mod app;
pub mod controllers;
pub mod entities;
pub mod migrations;
pub mod models;
pub mod services;
pub mod utils;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
