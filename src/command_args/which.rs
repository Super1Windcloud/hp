
use  clap::Args;


#[derive(Args, Debug)]
#[clap(name = "which", about = "🐸\t\t打印指定APP的可执行文件路径")]
#[clap(arg_required_else_help = true)]
pub struct  WhichArgs             {
  pub(crate) name: Option<String>, 
  #[arg(from_global)]
  pub global :bool
}
