﻿use std::env;

pub fn init_env_path() -> String {
    let mut path = env::var("SCOOP").unwrap_or(String::new());
    if path.is_empty() {
        path = env::var("USERPROFILE").unwrap() + "\\scoop"; // 可以使用or_else 替代
    }
    path
}

pub fn init_scoop_global() -> String {
    let path = env::var("SCOOP_GLOBAL").or(env::var("ProgramData"));
    if path.is_err() {
        panic!("No SCOOP_GLOBAL environment variable provided.");
    }
    path.unwrap() + "\\scoop"
}

pub fn get_app_current_dir(app_name: &str) -> String {
    let scoop_home = init_env_path();
    format!("{}\\apps\\{}\\current", scoop_home, app_name)
}

pub fn get_app_dir(app_name: &str) -> String {
    let scoop_home = init_env_path();
    format!("{}\\apps\\{}", scoop_home, app_name)
}
pub fn get_app_version_dir(app_name: &str, version: &String) -> String {
    let scoop_home = init_env_path();
    format!("{}\\apps\\{}\\{}", scoop_home, app_name, version)
}
pub fn get_app_dir_install_json(app_name: &str) -> String {
    let scoop_home = init_env_path();

    format!("{}\\apps\\{}\\current\\install.json", scoop_home, app_name)
}
pub fn get_app_dir_manifest_json(app_name: &str) -> String {
    let scoop_home = init_env_path();
    format!("{}\\apps\\{}\\current\\manifest.json", scoop_home, app_name)
}

pub fn get_app_current_bin_path(app_name: String, bin_name: &String) -> String {
    let scoop_home = init_env_path();
    format!("{}\\apps\\{}\\current\\{}", scoop_home, app_name, bin_name)
}

pub fn get_old_scoop_dir() -> String {
    let path = env::var("LocalAppData").unwrap_or(String::new());
    path + "\\scoop"
}

pub fn get_scoop_cfg_path() -> String {
    let path = env::var("USERPROFILE").unwrap();
    path + "\\.config\\scoop\\config.json"
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
#[derive(Debug)]
pub struct HyperScoopGlobal {
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

impl HyperScoopGlobal {
    pub fn new() -> Self {
        Self {
            scoop_path: init_scoop_global(),
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

pub fn get_persist_dir_path() -> String {
    let hyper_scoop = HyperScoop::new();
    hyper_scoop.get_persist_path()
}
pub fn get_cache_dir_path() -> String {
    let hyper_scoop = HyperScoop::new();
    hyper_scoop.get_cache_path()
}

pub fn get_buckets_root_dir_path() -> String {
    let hyper_scoop = HyperScoop::new();
    hyper_scoop.get_bucket_path()
}
pub fn get_shims_path() -> String {
    let hyper_scoop = HyperScoop::new();
    hyper_scoop.get_shims_path()
}

pub fn get_apps_path() -> String {
    let hyper_scoop = HyperScoop::new();
    hyper_scoop.get_apps_path()
}

// 全局版本的 get_app_current_dir
pub fn get_app_current_dir_global(app_name: &str) -> String {
    let scoop_home = init_scoop_global();
    format!("{}\\apps\\{}\\current", scoop_home, app_name)
}

// 全局版本的 get_app_dir
pub fn get_app_dir_global(app_name: &str) -> String {
    let scoop_home = init_scoop_global();
    format!("{}\\apps\\{}", scoop_home, app_name)
}

// 全局版本的 get_app_version_dir
pub fn get_app_version_dir_global(app_name: &str, version: &String) -> String {
    let scoop_home = init_scoop_global();
    format!("{}\\apps\\{}\\{}", scoop_home, app_name, version)
}

// 全局版本的 get_app_dir_install_json
pub fn get_app_dir_install_json_global(app_name: &str) -> String {
    let scoop_home = init_scoop_global();
    format!("{}\\apps\\{}\\current\\install.json", scoop_home, app_name)
}

// 全局版本的 get_app_dir_manifest_json
pub fn get_app_dir_manifest_json_global(app_name: &str) -> String {
    let scoop_home = init_scoop_global();
    format!("{}\\apps\\{}\\current\\manifest.json", scoop_home, app_name)
}

// 全局版本的 get_app_current_bin_path
pub fn get_app_current_bin_path_global(app_name: String, bin_name: &String) -> String {
    let scoop_home = init_scoop_global();
    format!("{}\\apps\\{}\\current\\{}", scoop_home, app_name, bin_name)
}

// 全局版本的 get_old_scoop_dir
pub fn get_old_scoop_dir_global() -> String {
    let path = env::var("ProgramData").unwrap_or(String::new());
    path + "\\scoop"
}

// 全局版本的 get_scoop_cfg_path
pub fn get_scoop_cfg_path_global() -> String {
    let path = env::var("ProgramData").unwrap();
    path + "\\scoop\\.config\\config.json"
}

// 全局版本的 get_persist_dir_path
pub fn get_persist_dir_path_global() -> String {
    let hyper_scoop = HyperScoopGlobal::new();
    hyper_scoop.get_persist_path()
}

// 全局版本的 get_cache_dir_path
pub fn get_cache_dir_path_global() -> String {
    let hyper_scoop = HyperScoopGlobal::new();
    hyper_scoop.get_cache_path()
}

// 全局版本的 get_buckets_root_dir_path
pub fn get_buckets_root_dir_path_global() -> String {
    let hyper_scoop = HyperScoopGlobal::new();
    hyper_scoop.get_bucket_path()
}

// 全局版本的 get_shims_path
pub fn get_shims_path_global() -> String {
    let hyper_scoop = HyperScoopGlobal::new();
    hyper_scoop.get_shims_path()
}

// 全局版本的 get_apps_path
pub fn get_apps_path_global() -> String {
    let hyper_scoop = HyperScoopGlobal::new();
    hyper_scoop.get_apps_path()
}

mod test_path {
    #[allow(unused)]
    use super::*;
    #[test]
    fn get_current_bin_path() {
        let app_name = "zigmod";
        let exe_name = "zig/zig.exe";
        let path = get_app_current_bin_path(app_name.to_string(), &exe_name.to_string());
        if std::path::Path::new(&path).exists() {
            println!("{}", path);
        }
    }

    #[test]
    #[ignore]
    fn test_get_suffix() {
        let _exe_name = "zig/zig.cmd";
        let _exe_name = "zig//zig.cmd";
        let _exe_name = "zig.cmd";
        let _exe_name = r"bin\zig.cmd";
        let exe_name = r"bin\\zig.cmd";
        let suffix = exe_name.split(".").last().unwrap_or("");
        let prefix = exe_name.split(".").next().unwrap_or("");
        println!("{suffix}  {prefix}");
    }

    #[test]
    fn test_global() {
        let path = env::var("ProgramData").unwrap() + "\\scoop";
        println!("{}", path);
    }
}
