use crate::check_self_update::auto_check_hp_update;
use crate::command_args::install::InstallArgs;
use crate::hyperscoop_middle::invoke_update::{update_buckets, update_hp};
use anyhow::bail;
use command_util_lib::install::*;
use command_util_lib::utils::system::{is_admin, request_admin};
use crossterm::style::Stylize;
use std::path::Path;

pub async fn execute_install_command(args: InstallArgs) -> Result<(), anyhow::Error> {
    let options = inject_user_options(&args)?;
    if options.contains(&InstallOptions::CheckCurrentVersionIsLatest) {
        auto_check_hp_update().await?;
    }
    if args.app_name.is_none() {
        return Ok(());
    }
    if args.global {
        if !is_admin()? {
            request_admin()
        }
    }

    if options.contains(&InstallOptions::UpdateHpAndBuckets) {
        println!("{}", "开始更新hp和buckets".dark_cyan().bold());
        let update_option = create_update_options(&options)?;
        update_hp(&update_option).await?;
        update_buckets().await?;
    }
    let app_name = args.app_name.clone().unwrap();
    let app_name = convert_path(app_name.trim());
    if Path::new(&app_name).exists() {
        log::debug!("manifest file {}", app_name);
        let manifest_path = app_name;
        install_app_from_local_manifest_file(manifest_path, options, None).await?;
        return Ok(());
    }
    if contains_special_char(app_name.as_str()) {
        bail!("指定的APP格式错误 error char")
    }

    if app_name.contains("/") {
        if app_name.contains('@') {
            bail!("指定的App格式不正确")
        }
        let split_arg = app_name.split('/').collect::<Vec<&str>>();
        if split_arg.iter().count() == 2 {
            let bucket = split_arg[0];
            let app_name = split_arg[1];
            if bucket.is_empty() || app_name.is_empty() {
                bail!("指定的App格式不正确")
            }
            install_from_specific_bucket(bucket, app_name, &options).await?;
            return Ok(());
        } else if split_arg.iter().count() > 2 || split_arg.len() == 1 {
            bail!("指定的APP格式错误")
        }
    }
    if app_name.contains('@') {
        let split_version = app_name.split('@').collect::<Vec<&str>>();
        if split_version.iter().count() == 2 {
            let app_name = split_version[0];
            let app_version = split_version[1];
            if app_name.is_empty() || app_version.is_empty() {
                bail!("指定的APP格式错误")
            }
            install_app_specific_version(app_name, app_version, &options).await?;
            return Ok(());
        } else if split_version.len() == 1 || split_version.len() > 2 {
            bail!("指定的APP格式错误")
        }
    }
    if contains_special_char(app_name.as_str()) {
        bail!("指定的APP格式错误")
    }
    install_app(app_name.as_str(), &options).await?;
    Ok(())
}

fn create_update_options(option: &[InstallOptions]) -> anyhow::Result<Vec<UpdateOptions>> {
    let mut update_options = vec![];
    if option.contains(&InstallOptions::UpdateHpAndBuckets) {
        update_options.push(UpdateOptions::UpdateHpAndBuckets);
    }
    if option.contains(&InstallOptions::NoUseDownloadCache) {
        update_options.push(UpdateOptions::NoUseDownloadCache);
    }
    if option.contains(&InstallOptions::SkipDownloadHashCheck) {
        update_options.push(UpdateOptions::SkipDownloadHashCheck);
    }

    if option.contains(&InstallOptions::NoAutoDownloadDepends) {
        update_options.push(UpdateOptions::NoAutoDownloadDepends);
    }
    if option.contains(&InstallOptions::Global) {
        update_options.push(UpdateOptions::Global);
    }

    Ok(update_options)
}

pub fn inject_user_options(install_args: &InstallArgs) -> anyhow::Result<Vec<InstallOptions>> {
    let mut install_options = vec![];
    if let Some(arch) = install_args.arch.as_ref() {
        // as_ref 引用原始数据
        let arch = arch.trim();
        if arch != "64bit" && arch != "32bit" && arch != "arm64" {
            bail!("arch 格式错误, 请使用 64bit, 32bit, arm64")
        }
        install_options.push(InstallOptions::ArchOptions(arch));
    }
    if install_args.only_download_no_install_with_override_cache {
        install_options.push(InstallOptions::ForceDownloadNoInstallOverrideCache)
    }
    if install_args.skip_download_hash_check {
        install_options.push(InstallOptions::SkipDownloadHashCheck)
    }
    if install_args.update_hp_and_buckets {
        install_options.push(InstallOptions::UpdateHpAndBuckets)
    }
    if install_args.no_use_download_cache {
        install_options.push(InstallOptions::NoUseDownloadCache)
    }
    if install_args.no_auto_download_dependencies {
        install_options.push(InstallOptions::NoAutoDownloadDepends)
    }
    if install_args.global {
        install_options.push(InstallOptions::Global)
    }
    if install_args.only_download_no_install {
        install_options.push(InstallOptions::OnlyDownloadNoInstall)
    }
    if install_args.check_version_up_to_date {
        install_options.push(InstallOptions::CheckCurrentVersionIsLatest)
    } 
  
    if install_args. force_install_override { 
       install_options.push(InstallOptions::ForceInstallOverride) 
    }
    Ok(install_options)
}

fn contains_special_char(s: &str) -> bool {
    let special_chars = r#"!#$%^&*()+=\[]\{}|;':",.<>?~`"#;
    s.chars().any(|c| special_chars.contains(c))
}

fn convert_path(path: &str) -> String {
    let path = path.replace("\\", "/");
    path
}
