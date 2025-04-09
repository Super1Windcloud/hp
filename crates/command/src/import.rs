use std::path::PathBuf;
use crate::init_hyperscoop;
use anyhow::bail;
use crossterm::style::Stylize;

pub fn write_into_scoop_config(config: String) {
    let default_config_path =
        std::env::var("USERPROFILE").unwrap() + "\\.config\\scoop\\config.json";
    log::info!("{:?}", default_config_path);
    std::fs::write(default_config_path, config).unwrap();
}

pub fn add_buckets(buckets: Vec<(&str, &str)>, path: String) -> Result<(), anyhow::Error> {
    for bucket in buckets {
        if bucket.0.is_empty() || bucket.1.is_empty() {
            bail!(
                "Config_File Error: bucket name or url is empty on {}",
                path.red().bold()
            )
        }
        let name = bucket.0;
        let source = bucket.1;
        log::info!("{:?}", name);
        log::info!("{:?}", source);
        invoke_hp_bucket_add(name, source);
    }

    Ok(())
}

fn invoke_hp_bucket_add(name: &str, url: &str) {
    let mut cmd = std::process::Command::new("hp");
    cmd.arg("bucket").arg("add").arg(name).arg(url);
    let output = cmd.output().expect("failed to execute hp command process");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
    }
}

pub fn install_apps(app_info: Vec<(&str, &str, &str)>, path: String) -> Result<(), anyhow::Error> {
    for (app_name, bucket, version) in app_info {
        if app_name.is_empty() || bucket.is_empty() || version.is_empty() {
            bail!(
                "Config_File Error: app name or bucket or version is empty, on {}",
                path.red().bold()
            )
        }
        invoke_hp_install(app_name, bucket )?;
    }

    Ok(())
}

fn invoke_hp_install(app_name: &str, bucket: &str ) -> Result<(), anyhow::Error> {
    let hp = init_hyperscoop()?;
    let buckets_path = hp.bucket_path.clone();
    for entry in std::fs::read_dir(buckets_path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let bucket_name = path.file_name().unwrap().to_str().unwrap();
        if bucket_name != bucket {
            continue;
        }
        log::debug!("{:?}", &path);
        let  path = path.join("bucket");
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file()  {
                continue;
            }
           let file_name = path.file_name().unwrap().to_str().unwrap().replace(".json", "");
           if file_name != app_name {
               continue;
           }
           log::debug!("app manifest {:?}", path.display());
          parser_app_manifest(path.clone())?;
          return Ok(());
        }
      bail!("Config_File Error: app not exist on {}", path.clone().to_str().unwrap().red().bold())
    }
    eprintln!("Error: bucket {} not found", bucket.red().bold());
    bail!("Config_File Error: bucket not exist ");
}

fn parser_app_manifest(path : PathBuf) -> Result<() , anyhow::Error> {
   log::info!("开始安装app") ;
   let manifest = std::fs::read_to_string(&path)?;
    log::info!("{:?}", &manifest);
   Ok(())
}
