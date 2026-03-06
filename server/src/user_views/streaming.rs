use axum::{extract::State,extract::Query, http::StatusCode, response::sse::{Event, KeepAlive, Sse}};
use futures_util::{Stream, StreamExt};
use std::{convert::Infallible};
use tokio_stream::wrappers::WatchStream;
use crate::objects::AppState;
use super::payloads;



pub async fn stream_metrics(
    State(state): State<AppState>,Query(params): Query<payloads::IdQuery>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>,StatusCode> {
      let node_recever=match  state.cpu_strem.get(&format!("node_cpu_strem_{}",params.node))
      {
        Some(v)=>v,
        None=>return Err(StatusCode::SERVICE_UNAVAILABLE)
      };

      let rx = node_recever.subscribe();

      let strem=WatchStream::new(rx)
      .map(|cpu|{
        // let json = serde_json::to_string(&cpu).unwrap();
        Ok(Event::default().data(cpu.to_string()))
      });
 Ok(Sse::new(strem).keep_alive(KeepAlive::default()))
}

