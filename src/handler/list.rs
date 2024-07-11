pub fn handle_list() {
    println!("List handler");
    let _ = get_available_versions_list();
}

#[tokio::main]
async fn get_available_versions_list() -> Result<(), reqwest::Error> {
    let url = "https://nodejs.org/dist/index.json";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let versions: serde_json::Value = response.json().await?;
        if let Some(versions_array) = versions.as_array() {
            for version in versions_array {
                // println!("Versions: {:?}", version);
                if let Some(version_str) = version["version"].as_str() {
                    println!("Version: {}", version_str);
                }
            }
        }
    } else {
        eprintln!("Failed to fetch versions: {}", response.status());
    }

    Ok(())
}
