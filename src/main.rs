use clap::Parser;

mod language_model;
mod opt;
mod prompt_basic;

async fn setup_tracing()
{
  let log_level = tracing::Level::DEBUG;
  let subscriber = tracing_subscriber::fmt()
    .with_max_level(log_level)
    .finish();
  tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() {
  setup_tracing().await;
  let options = opt::AlfredCLI::parse();
  match options {
    opt::AlfredCLI::Basic(option) => {
      prompt_basic::operate(option).await;
    }
  }
}
