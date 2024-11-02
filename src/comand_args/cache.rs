﻿use clap::{Args, Subcommand};

#[derive(Clone, Subcommand ,  Debug)]
pub enum  Cache  {
  Show (ShowArgs) ,
  Rm (RmArgs) ,
}

#[derive(Debug, Clone , Args)]
///显示所有缓存
pub struct ShowArgs  {

}
#[derive(Debug, Clone , Args)]
///删除指定缓存
pub struct RmArgs  {
   rm_app : String,
}

#[derive(Args, Debug)]
#[command(arg_required_else_help = true, subcommand_negates_reqs = true)]
#[ command(about ="🎨          显示或清理下载缓存 ")]
#[command(override_usage = " hyperscoop  cache show|rm [app(s)]" )]
pub  struct    CacheArgs{
    #[clap(subcommand)]
    command: Option<Cache> ,
   #[clap(long , short='a', help="清理所有缓存\t---hyperscoop cache rm -a" ,alias = "*" )]
   #[clap(alias = "*")]
   all: bool,
}


