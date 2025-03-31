﻿use std::env;

pub fn init_env_path() -> String {
    let mut path = env::var("SCOOP").unwrap_or(String::new());
    if path.is_empty() {
        path = env::var("USERPROFILE").unwrap() + "\\scoop";
    }
    return path;
}
pub fn get_app_current_dir(app_name: String) -> String {
    let scoop_home = init_env_path();
    return format!("{}\\apps\\{}\\current", scoop_home, app_name);
}

pub fn get_app_dir(app_name: &String) -> String {
    let scoop_home = init_env_path();
    return format!("{}\\apps\\{}", scoop_home, app_name);
}
pub fn get_app_version_dir(app_name: &String, version: &String) -> String {
    let scoop_home = init_env_path();
    return format!("{}\\apps\\{}\\{}", scoop_home, app_name, version);
}
pub fn get_app_dir_install_json(app_name: &String) -> String {
    let scoop_home = init_env_path();
    return format!("{}\\apps\\{}\\current\\install.json", scoop_home, app_name);
}
pub fn get_app_dir_manifest_json(app_name: &String) -> String {
    let scoop_home = init_env_path();
    return format!("{}\\apps\\{}\\current\\manifest.json", scoop_home, app_name);
}
pub fn init_scoop_global_path() -> String {
    let mut path = env::var("SCOOP_GLOBAL").unwrap_or(String::new());
    if path.is_empty() {
        path = env::var("ProgramData").unwrap() + "\\scoop";
    }
    return path;
}
pub  fn  get_app_current_bin_path(app_name : String , bin_name  :String ) -> String  {
  let scoop_home = init_env_path();
    format!("{}\\apps\\{}\\current\\{}", scoop_home, app_name  ,bin_name)  
} 


pub fn get_old_scoop_dir() -> String {
    let path = env::var("LocalAppData").unwrap_or(String::new());
    return path + "\\scoop";
}

pub fn get_scoop_cfg_path() -> String {
    let path = env::var("USERPROFILE").unwrap();
    return path + "\\.config\\scoop\\config.json";
}

#[derive(Debug)]
pub struct HyperScoop {
    pub scoop_path: String,
    pub bucket_path: String,
    pub cache_path: String,
    pub shims_path: String,
    pub persist_path: String,
    pub apps_path: String,
}

impl HyperScoop {
    pub fn new() -> Self {
        Self {
            scoop_path: init_env_path(),
            bucket_path: format!("{}\\buckets", init_env_path()),
            cache_path: format!("{}\\cache", init_env_path()),
            shims_path: format!("{}\\shims", init_env_path()),
            persist_path: format!("{}\\persist", init_env_path()),
            apps_path: format!("{}\\apps", init_env_path()),
        }
    }
    pub fn get_apps_path(&self) -> String {
        self.apps_path.clone()
    }
    pub fn get_psmodule_path(&self) -> String {
        format!("{}\\modules", self.scoop_path)
    }
    pub fn get_persist_path(&self) -> String {
        self.persist_path.clone()
    }

    pub fn get_bucket_path(&self) -> String {
        self.bucket_path.clone()
    }
    pub fn get_cache_path(&self) -> String {
        self.cache_path.clone()
    }
    pub fn get_shims_path(&self) -> String {
        self.shims_path.clone()
    }
    pub fn get_scoop_path(&self) -> String {
        self.scoop_path.clone()
    }
    pub fn print_all_paths(&self) {
        println!("Scoop Path: {}", self.scoop_path);
        println!("Buckets: {}", self.bucket_path);
        println!("Cache: {}", self.cache_path);
        println!("Shims: {}", self.shims_path);
        println!("Persist: {}", self.persist_path);
        println!("Apps: {}", self.apps_path);
    }
}


pub fn  get_shims_path() -> String {
  let  hyper_scoop = HyperScoop::new();
  hyper_scoop.get_shims_path()
}

pub  fn get_apps_path() -> String {
  let  hyper_scoop = HyperScoop::new();
  hyper_scoop.get_apps_path()
}