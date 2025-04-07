﻿use std::path::Path;
use crate::command_args::cat::CatArgs;
use crate::command_args::checkup::CheckupArgs;
use crate::command_args::cleanup::CleanupArgs;
use crate::command_args::config::ConfigArgs;
use crate::command_args::export::ExportArgs;
use crate::command_args::home::HomeArgs;
use crate::command_args::import::ImportArgs;
use crate::command_args::info::InfoArgs;
use crate::command_args::install::InstallArgs;
use crate::command_args::list::ListArgs;
use crate::command_args::merge_bucket::MergeArgs;
use crate::command_args::prefix::PrefixArgs;
use crate::command_args::reset::ResetArgs;
use crate::command_args::search::SearchArgs;
use crate::command_args::shim::ShimArgs;
use crate::command_args::status::StatusArgs;
use crate::command_args::uninstall::UninstallArgs;
use crate::command_args::update::UpdateArgs;
use crate::command_args::which::WhichArgs;
pub(crate) use crate::command_args::{bucket_args::BucketArgs, cache::CacheArgs};
use clap::{Args, Subcommand};
use crossterm::style::Stylize;
use command_util_lib::init_env::{get_app_dir_install_json};

#[derive(Debug, Subcommand)]
#[command(propagate_version = true)] // 自动传递版本信息
#[command(subcommand_negates_reqs = true)] // 禁止子命令的短选项冲突
#[command(infer_subcommands = true, infer_long_args = true)] // 自动推断子命令和长选项
#[command(
    arg_required_else_help = true,
    next_line_help = false,
    disable_help_subcommand = true
)]
pub(crate) enum Commands {
    Bucket(BucketArgs),

    Cat(CatArgs),
    Cache(CacheArgs),
    Checkup(CheckupArgs),
    Cleanup(CleanupArgs),
    Config(ConfigArgs),
    Export(ExportArgs),
    Home(HomeArgs),
    Hold(HoldArgs),
    Import(ImportArgs),
    Info(InfoArgs),
    Install(InstallArgs),
    List(ListArgs),
    Prefix(PrefixArgs),
    Reset(ResetArgs),
    #[clap(alias = "s")]
    Search(SearchArgs),
    Shim(ShimArgs),
    Status(StatusArgs),
    #[clap(alias = "un")]
    Uninstall(UninstallArgs),
    Update(UpdateArgs),
    Which(WhichArgs),
    Merge(MergeArgs),
    Credits(CreditsArgs),
}

#[derive(Args, Debug)]
#[clap(author, version, about="💖\t\t显示Credit信息", long_about = None)]
#[command(arg_required_else_help = false , subcommand_negates_reqs = true)]
#[command(no_binary_name = true)]
pub struct  CreditsArgs {}

pub fn  execute_credits_command()  -> anyhow::Result<()> {
  let str=  "hp  is created by superwindcloud(https://gitee.com/superwindcloud)".to_string().dark_blue().bold();
  println!("💖 {str}");
  Ok(())
}



#[derive(Args, Debug)]
#[clap(author, version, about="💖\t\t锁定指定APP版本,锁定之后更新所有APP或者检测更新状态将自动跳过", long_about = None)]
#[command(arg_required_else_help = true , subcommand_negates_reqs = true)]
#[command(no_binary_name = true)]
pub struct  HoldArgs {
   #[arg( required = false,  num_args =1.., help = "要锁定的APP名称,精准匹配,支持多参数")]
   pub   app_names :Option<Vec<String  >>,

}

pub fn  execute_hold_command(hold_args: HoldArgs) -> anyhow::Result<()> {
     if hold_args.app_names.is_none() { return Ok(()); }
     let  app_names = hold_args.app_names .unwrap();
      let  install_json_files = app_names.iter().filter_map(|name| { 
        let  install_json = get_app_dir_install_json(name); 
        if !Path::new(&install_json).exists() {
             eprintln!("{install_json} 不存在");
              None 
        }else { 
          Some(install_json)
        }
      }).collect::<Vec<_>>();
     
  Ok(())
}
