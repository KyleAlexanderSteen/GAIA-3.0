use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct IntentArgs {
    /// The intent text to submit
    pub text: String,

    /// Stream output as SSE chunks as it arrives
    #[arg(long, default_value_t = false)]
    pub stream: bool,

    /// Gateway base URL
    #[arg(long)]
    pub gateway: Option<String>,
}

pub async fn run(args: IntentArgs) -> Result<()> {
    // TODO: POST /intent  with Accept: text/event-stream when args.stream
    //       pipe each SSE chunk to stdout
    let _ = args;
    Err(super::not_implemented("intent", "#1299"))
}
