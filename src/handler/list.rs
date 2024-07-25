use crate::utils::download::NODE_VERSION_JSON_URL;
use prettytable::{Cell, Row, Table};
use reqwest::Response;
use std::collections::BTreeMap;

type VersionMap = BTreeMap<String, Vec<String>>;

pub fn handle_list() {
    let _ = get_available_versions_list();
}

async fn get_available_versions_list() -> Result<(), reqwest::Error> {
    let mut version_map: VersionMap = BTreeMap::new();
    let response: Response = reqwest::get(NODE_VERSION_JSON_URL).await?;
    if response.status().is_success() {
        let versions: serde_json::Value = response.json().await?;
        if let Some(versions_array) = versions.as_array() {
            for version in versions_array {
                // 如果为boolean 则跳过本次循环
                if version["lts"].is_boolean() {
                    continue;
                }
                if version_map.contains_key(&version["lts"].to_string()) {
                    version_map
                        .get_mut(&version["lts"].to_string())
                        .unwrap()
                        .push(version["version"].to_string());
                } else {
                    version_map.insert(
                        version["lts"].to_string(),
                        vec![version["version"].to_string()],
                    );
                }
            }
        }
        print_table_stdout(&version_map);
    } else {
        eprintln!("Failed to fetch versions: {}", response.status());
    }
    Ok(())
}

fn print_table_stdout(version_map: &VersionMap) {
    let mut table = Table::new();

    let header: Row = version_map
        .keys()
        .map(|key| Cell::new(&key.replace("\"", "")))
        .collect();

    table.add_row(header);

    let mut max_len = 0;

    for (k, _) in version_map.iter() {
        if k.len() > max_len {
            max_len = k.len();
        }
    }
    for index in 0..max_len {
        let mut row: Vec<Cell> = vec![];
        for (_, (_, v)) in version_map.iter().enumerate() {
            if v[index].is_empty() {
                continue;
            }
            row.push(Cell::new(&v[index].replace("\"", "")));
        }
        table.add_row(Row::new(row));
    }
    println!("\n Nodejs version list: \n");

    table.printstd();

    println!("\n This is a partial list. For a complete list, visit https://nodejs.org/en/download/releases");
}
