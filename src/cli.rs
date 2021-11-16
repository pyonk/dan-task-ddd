use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Option<Command>,
}

#[derive(StructOpt)]
pub enum Command {
    Add { arg: String },
}
