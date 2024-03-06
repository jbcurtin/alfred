use clap;
use futures_util::StreamExt;
use reqwest;
use tokio::io::AsyncWriteExt;

#[derive(clap::ValueEnum, Clone)]
pub enum Pretrained {
  #[clap(value_enum)]
  BertUncased,
}
pub async fn localize_model(model: Pretrained)
-> Result<(), reqwest::Error>
{
  let url = match model {
    Pretrained::BertUncased => "https://huggingface.co/google-bert/bert-base-uncased/resolve/main/pytorch_model.bin".to_string(),
  };
  let client = reqwest::Client::builder()
    .build()
    .expect("Unable to bulid Client");
  let current_directory = std::env::current_dir().expect("Unable to get current Directory");
  let current_directory = std::path::Path::new(&current_directory);
  let dirpath = current_directory.join("models");
  let filepath = dirpath.join("bert-base-uncased-pytorch_model.bin");
  if !filepath.exists() {
    tracing::info!("Downloading model from: {}", &url);
    if !dirpath.exists() {
      std::fs::create_dir_all(&dirpath).expect("Unable to create directory");
    }
    let mut file = tokio::fs::File::create(&filepath).await.expect("Unable to create file");
    let mut stream = client.get(&url).send().await?.bytes_stream();
    while let Some(item) = stream.next().await {
      file.write_all(&item?).await.expect("Unable to write file");
    }
    tracing::info!("Model downloaded to filepath: {}", &filepath.to_string_lossy());
  }
  Ok(())
}
