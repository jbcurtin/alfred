use crate::language_model;
use crate::opt;

pub async fn operate(option: opt::LMOption) {
  language_model::localize_model(option.model).await.expect("Unable to localize");
}