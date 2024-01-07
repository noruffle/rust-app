mod cli_example;
mod cli_struct;
mod cli_tests;
mod cli_impl;

pub use {
  cli_example::smth,
  cli_struct::Config,
  cli_impl::{ run, search, search_case_insensitive }
};