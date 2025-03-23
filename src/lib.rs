#[macro_use]
extern crate tracing;
#[macro_use]
extern crate futures;

use buttplug::core::{
    connector::{ButtplugConnector, ButtplugRemoteClientConnector},
    message::{ButtplugClientMessageCurrent, ButtplugServerMessageCurrent},
};
pub(crate) use buttplug::{core, util};

mod tungstenite_connect;
mod websocket_client;
mod websocket_server;

pub use websocket_client::ButtplugWebsocketClientTransport;
pub use websocket_server::{
    ButtplugWebsocketServerTransport, ButtplugWebsocketServerTransportBuilder,
};

/// Convenience method for creating a new Buttplug Client Websocket connector that uses the JSON
/// serializer. This is pretty much the only connector used for IPC right now, so this makes it easy
/// to create one without having to fill in the generic types.
pub fn new_json_ws_client_connector(
    address: &str,
) -> impl ButtplugConnector<ButtplugClientMessageCurrent, ButtplugServerMessageCurrent> {
    use crate::core::message::serializer::ButtplugClientJSONSerializer;

    ButtplugRemoteClientConnector::<
      ButtplugWebsocketClientTransport,
      ButtplugClientJSONSerializer,
    >::new(ButtplugWebsocketClientTransport::new_insecure_connector(
    address,
  ))
}
