#[tauri::command(async)]
pub async fn embed(url: String) -> String {

  if let Ok(res) = reqwest::get(&url).await {
    if let Ok(text) = res.text().await {
      return text;
    }
  }

  String::new()
}