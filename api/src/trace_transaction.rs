use ethers::{
    core::types::{GethDebugTracingOptions, H256},
    providers::Middleware,
    types::{GethDebugBuiltInTracerType, GethDebugTracerType},
};
use std::str::FromStr;

/// use `debug_traceTransaction` to fetch traces
/// requires, a valid endpoint in `RPC_URL` env var that supports `debug_traceTransaction`
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let app = api::App::from_stdin_password().await.unwrap();

  let client = app.on_chain.contract.client();

  let tx_hash = "0x4b8105da33f8e9b14ec1ac0dde9b1f6ad3c7f16ba4f052a52b4fca2a9e84327b";
  let h: H256 = H256::from_str(tx_hash)?;

  dbg!("here");
  // default tracer
  //let options = GethDebugTracingOptions::default();
  //let traces = client.debug_trace_transaction(h, options).await?;
  //println!("{traces:?}");

  // call tracer
  let options = GethDebugTracingOptions {
    disable_storage: Some(true),
    enable_memory: Some(false),
    tracer: Some(GethDebugTracerType::BuiltInTracer(
      GethDebugBuiltInTracerType::CallTracer,
    )),
    ..Default::default()
  };
  let traces = client.debug_trace_transaction(h, options).await?;
  println!("{traces:?}");

  // js tracer
  let options = GethDebugTracingOptions {
    disable_storage: Some(true),
    enable_memory: Some(false),
    tracer: Some(GethDebugTracerType::JsTracer(String::from("{data: [], fault: function(log) {}, step: function(log) { if(log.op.toString() == \"DELEGATECALL\") this.data.push(log.stack.peek(0)); }, result: function() { return this.data; }}"))),
    ..Default::default()
  };
  let traces = client.debug_trace_transaction(h, options).await?;
  println!("{traces:?}");

  Ok(())
}
