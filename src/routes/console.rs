use axum::body::Body;
use axum::extract::{State, WebSocketUpgrade};
use axum::http::Response;
use axum::Router;
use axum::extract::ws::{WebSocket, Message};
use axum::routing::any;
use futures_util::SinkExt;
use futures_util::stream::{StreamExt, SplitStream, SplitSink};
use crate::state::VismutState;

async fn write(mut writer: SplitSink<WebSocket, Message>, state: VismutState) {
    let mut rx = state.sender().subscribe();
    writer.send(Message::text("hi")).await.unwrap();
    while let Ok(message) = rx.recv().await {
        writer.send(Message::text(message.to_text().unwrap())).await.unwrap();   
    }
    
}

async fn read(mut reader: SplitStream<WebSocket>) {

}

async fn handle_ws(mut socket: WebSocket, state: VismutState) {
    let (writer, reader) = socket.split();
    tokio::spawn(write(writer, state));
    tokio::spawn(read(reader));
}

async fn get_console(
    ws: WebSocketUpgrade,
    State(state): State<VismutState>,
) -> Response<Body> {
    ws.on_upgrade(|socket| handle_ws(socket, state))
}

pub fn build_console_route() -> Router<VismutState> {
    Router::new().route("/console", any(get_console))
}