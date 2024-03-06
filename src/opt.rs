use crate::language_model;

use clap;

#[derive(clap::Parser)]
#[command(name="alfred", about="Alfred CLI")]
#[command(bin_name="alfred")]
pub enum AlfredCLI {
  Basic(LMOption)
}
#[derive(clap::Args, Clone)]
#[command(author, version, about, long_about = None)]
pub struct LMOption {
  pub model: language_model::Pretrained,
  pub input: String,
}
