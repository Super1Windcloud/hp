use crate::command_args::status::StatusArgs;
use command_util_lib::init_env::{ get_apps_path, get_apps_path_global, get_buckets_root_dir_path, get_buckets_root_dir_path_global};
use command_util_lib::list::VersionJSON;
use crossterm::style::Stylize;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn execute_status_command(status_args: StatusArgs) -> Result<(), anyhow::Error> {
    let apps_path = if status_args.global {
        get_apps_path_global()
    } else {
        get_apps_path()
    };
    let bucket_path = if status_args.global {
        get_buckets_root_dir_path_global()
    } else {
        get_buckets_root_dir_path()
    };

    let mut current_versions = Vec::new();
    let mut installed_apps = Vec::new();
    for app_path in std::fs::read_dir(apps_path)? {
        let app_path = app_path?.path();
        let app_name = app_path
            .file_name()
            .expect("Invalid app path")
            .to_str()
            .unwrap();
        let current = app_path.join("current");
        let manifest_path = current.join("manifest.json");

        if !manifest_path.exists() {
            current_versions.push("Not Install Correctly".to_string());
            installed_apps.push(app_name.to_string());
            continue;
        }
        let manifest = std::fs::read_to_string(manifest_path)?;
        let manifest: VersionJSON = serde_json::from_str(&manifest)?;
        let current_version = manifest.version.unwrap_or("Not Found".to_string());
        current_versions.push(current_version.to_string());
        installed_apps.push(app_name.to_string());
    }
    let install_apps = installed_apps.as_slice();
    let version_map = build_version_map(bucket_path, &installed_apps)?;

    let latest_versions: Vec<String> = install_apps
        .iter()
        .map(|app_name| {
            version_map
                .get(app_name.to_lowercase().as_str())
                .cloned()
                .unwrap_or_else(|| "Not Found".to_string())
        })
        .collect();
    let max_version_len = latest_versions.iter().map(|s| s.len()).max().unwrap_or(0) + 4;
    let max_name_len = install_apps.iter().map(|s| s.len()).max().unwrap_or(0) + 4;

    let mut executed = false;
    for (app_name, (current_version, latest_version)) in install_apps
        .iter()
        .zip(current_versions.iter().zip(latest_versions.iter()))
    {
        if executed == false {
            println!(
                "{:<width$}{:<width1$}{:<width1$}{:<width$}",
                "Name\t\t\t\t".green().bold(),
                "Installed Version\t\t".green().bold(),
                "Latest Version\t\t".green().bold(),
                "Need Update\t\t\t".green().bold(),
                width = max_name_len,
                width1 = max_version_len
            );

            println!(
                "{:<width$}{:<width1$}{:<width1$}{:<width$}",
                "____\t\t\t\t".green().bold(),
                "_________________\t\t".green().bold(),
                "_______________\t\t".green().bold(),
                "_____________\t\t".green().bold(),
                width = max_name_len,
                width1 = max_version_len
            );
        }
        executed = true;
        if latest_version > current_version {
            println!(
                "{:<width1$}{:<width1$}{:<width2$}{:<width1$}",
                app_name,
                current_version,
                latest_version,
                "YES",
                width1 = max_name_len,
                width2 = max_version_len
            );
        } else {
            println!(
                "{:<width1$}{:<width1$}{:<width2$}{:<width1$}",
                app_name,
                current_version,
                latest_version,
                "NO",
                width1 = max_name_len,
                width2 = max_version_len
            );
        }
    }
    Ok(())
}

fn build_version_map(
    bucket_path: String,
    installed_app_name: &[String],
) -> Result<HashMap<String, String>, anyhow::Error> {
    let version_map: HashMap<String, String> = std::fs::read_dir(bucket_path)?
        .par_bridge()
        .filter_map(|bucket| {
            let bucket = bucket.ok()?.path().join("bucket");
            let entries: Vec<_> = std::fs::read_dir(bucket).ok()?.collect();
            Some(entries)
        })
        .flatten()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().unwrap_or_default() != "json" {
                return None;
            }
            let file_name = path.file_stem()?.to_str()?.to_lowercase();
            if !installed_app_name.contains(&file_name) {
                return None;
            }
            let manifest = std::fs::read_to_string(&path).ok()?;
            let manifest: VersionJSON = serde_json::from_str(&manifest).ok()?;
            let version = manifest.version.unwrap_or("Not Found".to_string());
            Some((file_name, version))
        })
        .collect();
    Ok(version_map)
}
