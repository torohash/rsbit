use crate::{
    v5::ws::BybitWS,
    auth::Auth,
};
use anyhow::Result;
use tokio::net::TcpStream;
use chrono::Utc;
use tokio_tungstenite::{
    WebSocketStream,
    connect_async,
    MaybeTlsStream,
    tungstenite::Message,
};
use futures_util::{
    StreamExt,
    SinkExt,
    stream::{
        SplitSink,
        SplitStream,
    },
};
use serde_json::json;

impl BybitWS {
    pub async fn connect(&self, is_private: bool) -> Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )> {
        if is_private {
            self.connect_private().await
        } else {
            self.connect_public().await
        }
    }

    async fn connect_private(&self) -> Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )> {
        let (ws_stream, _) = connect_async(self.channel()).await?;
        let expires = (Utc::now().timestamp_millis() + 10000).to_string();
        let signature = self.create_signature(&expires)?;

        let auth = json!({
            "op": "auth",
            "args": [self.api_key(), expires.to_string(), signature]
        });
        let (mut write, read) = ws_stream.split();
        let auth_message = Message::Text(auth.to_string());
        write.send(auth_message).await?;
        Ok((write, read))
    }

    async fn connect_public(&self) -> Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )> {
        let (ws_stream, _) = connect_async(self.channel()).await?;
        let (write, read) = ws_stream.split();
        Ok((write, read))
    }
}