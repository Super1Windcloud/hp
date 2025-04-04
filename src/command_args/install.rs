﻿
use clap::ArgAction;
use  clap::Args;

#[derive(Args, Debug)]
#[command(name = "install", alias = "i",  about = "🐘          安装指定APP,别名i")]
#[clap(author="superwindcloud", version , long_about = None)]
#[command(arg_required_else_help = true)]
#[command(after_help = r#"
e.g. 安装应用程序的通常方法（使用您的本地buckets）： hp install git

指定特定buckets的清单中安装:   hp install  main/genact

安装应用程序的不同版本,如果存在多版本清单 :  hp install gh@2.7.0

从计算机上的指定路径清单中安装应用程序 :   hp install \path\to\app.json
     "#)]
pub struct InstallArgs  {
  #[arg(help = "安装APP的名称,精准匹配,仅单个安装", required = false  )]
  pub  app_name: Option<String>,

  #[arg(short='k' , long, help = "跳过本地缓存，强制从远程源重新下载安装", required = false , action = ArgAction::SetTrue, help_heading = "Install Options" )]
   pub no_use_download_cache : bool,
  #[arg(short='i' , long, help = "不自动下载manifest里的依赖", required = false , action = ArgAction::SetTrue,help_heading = "Install Options" )]
  pub  no_auto_download_dependencies : bool,
  #[arg(short, long, help = "跳过下载哈希校验", required = false, action = ArgAction::SetTrue,help_heading = "Install Options"  )]
  pub ship_hash_check : bool,
  #[arg(short='u' , long, help = "安装前更新hp和bucket,默认不更新", required = false , action = ArgAction::SetTrue,help_heading = "Install Options" )]
  pub update_hp_and_bucket : bool,

  #[arg(short='a', long, help = "指定安装架构, 如果支持的话", help_heading = "Install Options", 
    required = false ,default_value ="64bit" ,value_name="<32bit|64bit|arm64>")]
  pub arch : Option<String>,

  #[arg(from_global)]
  pub  global :bool

}
