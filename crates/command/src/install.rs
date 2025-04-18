use crate::manifest::install_manifest::InstallManifest;
use anyhow::{bail, Result};
use crossterm::style::Stylize;
use std::path::Path;

pub mod installer;
use crate::init_env::{
    check_bucket_whether_exists, get_app_current_dir, get_app_dir, get_app_dir_global,
    get_special_bucket_all_manifest_path, get_special_bucket_all_manifest_path_global,
};
use crate::manifest::manifest_deserialize::*;
pub use installer::*;
pub mod check;
pub mod shim_and_shortcuts;
pub use check::*;
pub use shim_and_shortcuts::*;
pub mod cli_options_store;
pub use cli_options_store::*;
pub mod aria2;
pub use aria2::*;
pub mod sevenzip;
pub use sevenzip::*;
pub mod download;
use crate::manifest::manifest::{
    get_latest_manifest_from_local_bucket, get_latest_manifest_from_local_bucket_global,
};
use crate::utils::utility::{nightly_version, validate_version};
pub use download::*;

/// 下载, 解压, preinstall, create_shim_shortcut, postinstall
pub async fn install_app_from_local_manifest_file<P: AsRef<Path>>(
    manifest_path: P,
    options: Vec<InstallOptions<'_>>,
    bucket_source: Option<&str>,
) -> Result<()> { 
     add_scoop_shim_root_dir_to_env_path().expect("add scoop shim root to env path failed");   
    let options: Box<[InstallOptions]> = options.into_boxed_slice();
    let manifest_path = manifest_path.as_ref().to_str().unwrap();
    let install_arch = handle_arch(&options)?;
    log::info!("install arch: {}", install_arch);
    let content = std::fs::read_to_string(&manifest_path)?;
    let mut serde_obj: InstallManifest = serde_json::from_str(&content)?;
    let app_name = serde_obj
        .set_name(&manifest_path)
        .get_name()
        .unwrap_or(String::new());
    let obj_copy = serde_obj.clone();
    if app_name.is_empty() {
        bail!("manifest file name is empty")
    }
    let version = &serde_obj.version.unwrap_or(String::new());
    if version.is_empty() {
        bail!("manifest file version is empty")
    }
    if options.contains(&InstallOptions::ForceInstallOverride) {
        let special_app_dir = if options.contains(&InstallOptions::Global) {
            get_app_dir_global(&app_name)
        } else {
            get_app_dir(&app_name)
        };
        if Path::new(&special_app_dir).exists() {
            std::fs::remove_dir_all(special_app_dir)?;
        }
    }
    validate_version(version)?;
    let options = if version == "nightly" {
        options
            .to_vec()
            .into_iter()
            .chain(vec![InstallOptions::SkipDownloadHashCheck])
            .collect()
    } else {
        options
    };
    let version = if version == "nightly" {
        &nightly_version()?
    } else {
        version
    };

    let result = if !options.contains(&InstallOptions::ForceDownloadNoInstallOverrideCache)
        && !options.contains(&InstallOptions::OnlyDownloadNoInstall)
    {
        check_before_install(&app_name, &version, &options).await?
    } else {
        0
    };
    if result != 0 {
        return Ok(());
    };
    let end_message = if bucket_source.is_none() {
        format!("from manifest file '{}'", manifest_path)
    } else {
        format!("from bucket '{}'", bucket_source.clone().unwrap())
    };

    println!(
        "{}",
        format!("Installing '{app_name}' ({version}) [{install_arch}] {end_message}")
            .bold()
            .dark_green()
    );

    let depends = serde_obj.depends;
    let suggest = serde_obj.suggest;
    let notes = serde_obj.notes;
    let env_set = serde_obj.env_set;
    let env_add_path = serde_obj.env_add_path;
    // let url = serde_obj.url;
    // let hash = serde_obj.hash;
    let installer = serde_obj.installer;
    let shortcuts = serde_obj.shortcuts;
    let architecture = serde_obj.architecture;
    let bin = serde_obj.bin;
    let extract_dir = serde_obj.extract_dir;
    let extract_to = serde_obj.extract_to;
    let innosetup = serde_obj.innosetup;
    let persist = serde_obj.persist;
    let psmodule = serde_obj.psmodule;
    let pre_install = serde_obj.pre_install;
    let post_install = serde_obj.post_install;
    if !depends.is_none()  && !options.contains(&InstallOptions::NoAutoDownloadDepends) {
        handle_depends(depends.unwrap().as_str(), &options).await?;
    }

    //  invoke aria2  to  download  file to cache
    let download_manager = DownloadManager::new(&options, &manifest_path, bucket_source);
    download_manager.start_download()?;
    if !options.contains(&InstallOptions::SkipDownloadHashCheck) {
        download_manager.check_cache_file_hash()?
    }
    if options.contains(&InstallOptions::OnlyDownloadNoInstall) {
        return Ok(());
    }
    //  提取 cache 中的zip 到 app dir
    //  parse    pre_install
    //  parse    manifest installer

    // linking   app current dir to app version dir
    //create_shims
    //create_startmenu_shortcuts
    //install_psmodule

    if !env_set.is_none() {
        handle_env_set(env_set.unwrap(), obj_copy)?;
    };
    if env_add_path.is_some() {
        let env_add_path = env_add_path.unwrap();
        if env_add_path != StringArrayOrString::Null {
            let app_current_dir = get_app_current_dir(&app_name);
            handle_env_add_path(env_add_path, app_current_dir)?;
        }
    }
    // linking  persist_data  链接 Persist 目录
    // persist_permission  主要用于 设置文件系统权限，确保特定用户（通常是 "Users" 组）对某个目录具有写入权限。
    //  parse post_install
    //  save  install.json , manifest.json  to app version dir

    if !suggest.is_none() {
        show_suggest(&suggest.unwrap())?;
    }
    println!(
        "{}",
        format!("'{app_name}' ({version}) was installed successfully!")
            .dark_green()
            .bold()
    );

    if notes.is_some() {
        let notes = notes.unwrap();
        if notes != StringArrayOrString::Null {
            show_notes(notes)?;
        }
    }
    Ok(())
}

pub async fn install_from_specific_bucket(
    bucket_name: &str,
    app_name: &str,
    options: &[InstallOptions<'_>],
) -> Result<()> {
    log::info!("install from specific bucket from {}", bucket_name);

    if !check_bucket_whether_exists(bucket_name, options)? {
        bail!("bucket '{}' not exists,please check it!", bucket_name)
    }
    let all_manifests = if options.contains(&InstallOptions::Global) {
        get_special_bucket_all_manifest_path_global(bucket_name)?
    } else {
        get_special_bucket_all_manifest_path(bucket_name)?
    };
    let manifest_path = all_manifests
        .iter()
        .find(|path| {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            file_name.to_lowercase() == app_name.to_lowercase()
        })
        .unwrap();
    log::debug!("manifest path: {}", manifest_path.display());
    Box::pin(install_app_from_local_manifest_file(
        manifest_path,
        options.to_vec(),
        Some(bucket_name),
    ))
    .await?;
    Ok(())
}

pub async fn install_app_specific_version(
    app_name: &str,
    app_version: &str,
    options: &Vec<InstallOptions<'_>>,
) -> Result<()> {
    log::info!("install from app specific version {}", app_version);

    Ok(())
}

pub async fn install_app(app_name: &str, options: &[InstallOptions<'_>]) -> Result<()> {
    log::info!("install from app {}", app_name);
    let manifest_path = if options.contains(&InstallOptions::Global) {
        get_latest_manifest_from_local_bucket_global(app_name)?
    } else {
        get_latest_manifest_from_local_bucket(app_name)?
    };
    // 使用 Box::pin 对递归调用的结果进行装箱, 防止栈溢出
    Box::pin(install_app_from_local_manifest_file(
        manifest_path,
        options.to_vec(),
        None,
    ))
    .await?;

    Ok(())
}
