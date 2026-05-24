use super::payloads;
use crate::objects::{AppState,StreamPayLoad};
use axum::{
    extract::Query,
    extract::State,
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
};
use futures_util::{Stream, StreamExt};
use std::convert::Infallible;
use tokio_stream::wrappers::WatchStream;


pub async fn stream_cpu_metrics(
    State(state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let node_recever = match state
        .stream_data
        .get(&format!("node_cpu_strem_{}", params.node))
    {
        Some(v) => v,
        None => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };

    let rx = node_recever.subscribe();

    let stream=WatchStream::new(rx)
   .filter_map(|payload|async move{
        match payload{
            StreamPayLoad::Cpu(cpu)=>Some(Ok(Event::default().json_data(cpu).unwrap())),
            _=>None
        }
    });

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

pub async fn stream_ram_metrics(
    State(state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let node_recever = match state
        .stream_data
        .get(&format!("node_ram_strem_{}", params.node))
    {
        Some(v) => v,
        None => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };

    let rx = node_recever.subscribe();

    let strem = WatchStream::new(rx)
        .filter_map(|payload|async move{
            match payload {
                StreamPayLoad::Ram(ram)=>Some( Ok(Event::default().json_data(ram).unwrap())),
                _=>None
            }
        });
    Ok(Sse::new(strem).keep_alive(KeepAlive::default()))
}

pub async fn stream_disk_metrix(
   State(state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,  
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode>{
    let node_recever = match state
        .stream_data
        .get(&format!("node_diskio_strem_{}", params.node))
    {
        Some(v) => v,
        None => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };

    let rx = node_recever.subscribe();

    let strem = WatchStream::new(rx)
        .filter_map(|payload|async move{
            match payload {
                StreamPayLoad::Disk(disk)=>Some( Ok(Event::default().json_data(disk).unwrap())),
                _=>None
            }
        });
    Ok(Sse::new(strem).keep_alive(KeepAlive::default()))
}
// test all of these and and arrange a notfication system ssl/tls exp