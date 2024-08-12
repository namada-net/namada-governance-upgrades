use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env)]
    pub tendermint_url: String,

    #[clap(long, env)]
    pub proposal_id: u64,

    #[clap(long, env)]
    pub expected_hash: Option<String>,

    #[clap(long, env)]
    pub dump_to: Option<PathBuf>,
}