﻿use clap::Args;

#[derive(Args, Debug)]
///🐬          打开指定APP的主页
pub struct HomeArgs {
    pub name: Option<String>,
}
