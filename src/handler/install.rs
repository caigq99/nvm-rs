use crate::utils::{get_download_url, get_suffix};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::{
    fmt::Write,
    fs, io,
    path::{Path, PathBuf},
};
use tokio::{
    fs::{self as tokiofs, File},
    io::AsyncWriteExt,
};
use zip::ZipArchive;

#[tokio::main]
pub async fn handle_install(version: String) -> Result<()> {
    let mut version = version.clone();
    if !version.starts_with("v") {
        version.insert(0, 'v');
    }
    //如果该目录存在，说明已经下载过了，直接退出
    match tokiofs::try_exists(&version).await? {
        true => {
            println!("version {} already exists", version);
            Ok(())
        }
        false => {
            let download_url = get_download_url(&version);
            println!("download url: {}", download_url);
            download_file(&download_url).await?;
            unzip_file(&format!("{}{}", &version, get_suffix())).await?;
            Ok(())
        }
    }
}

async fn download_file(url: &str) -> Result<()> {
    // 发起请求
    let mut response = reqwest::get(url).await?;
    // 获取下载文件总大小
    let total_size = response.content_length().unwrap();
    // 提取文件名
    let filename = url
        .split("/")
        .filter(|x| x.starts_with("v"))
        .last()
        .unwrap();
    println!("total size: {}", total_size);
    println!("filename: {}", filename);
    // 创建临时压缩包和文件夹
    let mut file = File::create(format!("{}{}", filename, get_suffix())).await?;
    // let _ = fs::create_dir(filename).await?;
    // 创建进度条
    let progress = ProgressBar::new(total_size);
    // 设置进度条样式
    progress .set_style(ProgressStyle::with_template("Starting download \n {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta}) \n{msg}")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    let mut downloaded = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        progress.set_position(downloaded);
    }
    progress.finish_with_message("Download complete!");
    Ok(())
}

// 在windows系统，解压zip包
async fn unzip_file(filename: &str) -> Result<()> {
    let name = filename.replace(".zip", "");
    let file = fs::File::open(filename).unwrap();
    let extract_path = PathBuf::from(&name);
    println!("extract path: {:?}", extract_path);
    let mut archive = ZipArchive::new(file).unwrap();
    let total_files = archive.len() as u64;
    // 创建进度条
    let progress = ProgressBar::new(total_files);
    // 设置进度条样式
    progress .set_style(ProgressStyle::with_template("Starting extract \n {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) \n{msg}")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    let mut extract_num = 0;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };
        if file.is_dir() {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        extract_num += 1;
        progress.set_position(extract_num);
    }
    progress.finish_with_message("Extract complete!");
    Ok(())
}

// 在linux,mac系统，解压tar包
fn untar_file() {}
