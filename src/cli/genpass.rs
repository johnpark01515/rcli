use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPwdOpt {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(8..) , default_value_t = 16)]
    pub length: u8,

    #[arg(long)]
    pub noupper: bool,

    #[arg(long)]
    pub nolower: bool,

    #[arg(long)]
    pub nonumber: bool,

    #[arg(long)]
    pub nosymbol: bool,
}
