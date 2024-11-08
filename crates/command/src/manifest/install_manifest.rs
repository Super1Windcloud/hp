﻿use serde::{Deserialize, Serialize};
use std::collections::HashMap;



macro_rules! arch_specific_field {
    ($self:ident, $field:ident) => {{
        let mut ret = $self.inner.$field.as_ref();

        if let Some(arch) = $self.inner.architecture.as_ref() {
            if cfg!(target_arch = "x86") {
                if let Some(ia32) = &arch.ia32 {
                    let $field = ia32.$field.as_ref();
                    if $field.is_some() {
                        ret = $field;
                    }
                }
            }

            if cfg!(target_arch = "x86_64") {
                if let Some(amd64) = &arch.amd64 {
                    let $field = amd64.$field.as_ref();
                    if $field.is_some() {
                        ret = $field;
                    }
                }
            }

            if cfg!(target_arch = "aarch64") {
                if let Some(aarch64) = &arch.aarch64 {
                    let $field = aarch64.$field.as_ref();
                    if $field.is_some() {
                        ret = $field;
                    }
                }
            }
        }
        ret
    }};
}


#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Manifest {
  #[serde(skip)]
  bucket: Option<String>,
  #[serde(skip)]
  name: Option<String>,

  /// 版本号
  pub version: Option<String>,

  // 与installer相同的选项，但运行文件/脚本来卸载应用程序。
  pub uninstaller: Option<String>,

  //运行非 MSI 安装程序的说明。
  pub installer: Option<String>,  // 安装程序的名称，如 `innosetup`

  //  指定在开始菜单中可用的快捷方式值
  pub shortcuts: Option<String>,  // 快捷方式配置

  //如果应用程序不是 32 位，则可以使用架构来包装差异
  pub architecture: Option<String>,
  //定义如何自动更新清单。
  pub autoupdate: Option<String>,
  pub cookie: Option<HashMap<String, Option<serde_json::Value>>>,
  //将自动安装的应用程序的运行时依赖项
  pub depends: Option<String>,
  pub description: Option<String>,
  //应用程序维护人员和开发人员可以使用bin/checkver工具来检查应用程序的更新版本
  // 。清单中的checkver属性是一个正则表达式，可用于匹配应用程序主页中应用程序的当前稳定版本
  pub checkver: Option<String>, // 用于检查更新的配置

  //在用户路径上可用的程序（可执行文件或脚本）的字符串或字符串数 组
  pub bin: Option<String>,    //可执行文件所在的目录。
  pub checksum: Option<String>,  //文件的校验和
  pub url: Option<String>,

  //字符串或字符串数组，其中包含url中每个 URL 的文件哈希值。默认情况下，
  // 哈希值是 SHA256，但您可以通过在哈希字符串前添加“sha512:”、“sha1:”或“md5:”前缀来使用 SHA512、SHA1 或 MD5
  pub hash: Option<String>,
  //   如果url指向压缩文件（支持 .zip、.7z、.tar、.gz、.lzma 和 .lzh），Scoop 将仅提取其中指定的目录
  pub extract_dir: Option<String>,
  // 如果url指向压缩文件（支持 .zip、.7z、.tar、.gz、.lzma 和 .lzh），Scoop 会将所有内容提取到指定目录
  pub extract_to: Option<String>,  // 解压缩后的目录
  ///  主页 URL
  pub homepage: Option<String>,
  //将此目录添加到用户路径（如果使用--global则添加到系统路径）。
  // 该目录是相对于安装目录的，并且必须位于安装目录内。
  pub env_add_path: Option<String>, // 添加到 PATH 环境变量的路径。
  //如果安装程序基于 InnoSetup，则设置为布尔值true
  pub innosetup: Option<bool>,
  pub license: Option<String>,

  //：单行字符串或字符串数组，其中包含在安装应用程序后显示的消息。
  pub notes: Option<String>,

  //保存在应用程序的数据目录中的目录和文件的字符串或字符串数组。持久数据
  pub persist: Option<String>,

  //作为 PowerShell 模块安装在~/scoop/modules中。
  pub psmodule: Option<String>,

  /// 为用户（或系统，如果使用--global ）设置一个或多个环境变量（
  pub env_set: Option<String>,

  pub pre_install: Option<String>,  // 安装前执行的命令
  pub post_install: Option<String>, // 安装后执行的命令
  pub pre_uninstall: Option<String>, // 卸载前执行的命令
  pub post_uninstall: Option<String>, // 卸载后执行的命令

}


impl Manifest {
  #[must_use]

  pub unsafe fn name(&self) -> &str {
    unsafe { self.name.as_ref().unwrap_unchecked() }
  }

  #[must_use]
  /// Get the name of the manifest, or [`None`] if it is not set
  pub fn name_opt(&self) -> Option<&str> {
    self.name.as_deref()
  }

  /// Set the name of the manifest
  pub fn set_name(&mut self, name: impl Into<String>) {
    self.name = Some(name.into());
  }

  #[must_use]

  pub unsafe fn bucket(&self) -> &str {
    unsafe { self.bucket.as_ref().unwrap_unchecked() }
  }

  #[must_use]
  /// Get the bucket the manifest is from, or [`None`] if it is not set
  pub fn bucket_opt(&self) -> Option<&str> {
    self.bucket.as_deref()
  }

  /// Set the bucket the manifest is from
  pub fn set_bucket(&mut self, bucket: impl Into<String>) {
    self.bucket = Some(bucket.into());
  }
}
