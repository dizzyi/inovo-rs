use serde_json;
use websocket::{ClientBuilder, Message, OwnedMessage};

use crate::logger::Logger;

/// Function to send JSON data to a ROS bridge via websocket
fn send_json_to_ros_bridge(logger: &mut Logger, host: &String, json: String) -> Result<(), String> {
    // The websocket URL using the provided host
    let url = format!("ws://{}:9090/", host);
    logger.info(format!("trying to send json to {}", url))?;

    // Attempt to connect to Websocket server until it is successful
    let mut handle: websocket::sync::Client<std::net::TcpStream> = loop {
        // create the Websocketclient builder
        let mut client_builder = ClientBuilder::new(&url).map_err(|e| e.to_string())?;

        logger.debug("    trying to connect to websocket . . . ")?;
        if let Ok(h) = client_builder.connect_insecure() {
            logger.debug("Successful connected to websocket")?;
            break h;
        }
    };

    // send the json message to call service
    logger.debug("sending message . . .")?;
    logger.debug(format!(">>> {}", json))?;
    let msg = Message::text(json);
    handle.send_message(&msg).map_err(|e| e.to_string())?;

    // read message from websocket in loop
    loop {
        logger.debug("reading message . . .")?;
        let message = handle.recv_message().map_err(|e| e.to_string())?;

        match &message {
            // If the message is text
            OwnedMessage::Text(text) => {
                logger.debug(format!("<<< {}", text))?;

                // try to pares it into json
                let json: serde_json::Value =
                    serde_json::from_str(text).map_err(|e| e.to_string())?;
                logger.debug(format!("<<< {:?}", json))?;

                // try to access the field
                let success: serde_json::Value = json["values"]["success"].clone();
                logger.debug(format!("<<< {:?}", success))?;

                // try to match the data structure of `success` and it value
                return match success {
                    serde_json::Value::Bool(true) => Ok(()),
                    _ => Err("call service not successsful.".to_string()),
                };
            }
            // if the message is not in text, just log the message
            _ => {
                logger.debug(format!("<<< <<< {:?}", message))?;
            }
        }
    }
}

/// Function to start a sequence on the ROS bridge
fn start_sequence(
    logger: &mut Logger,
    host: &String,
    procedure_name: &String,
) -> Result<(), String> {
    let json: String = serde_json::json!(
        {
            "op": "call_service",
            "service": "/sequence/start",
            "id": "call_service:/sequence/start",
            "type": "sequencer/RunSequence",
            "args": serde_json::json!({
                "procedure_name": procedure_name
            }),
        }
    )
    .to_string();

    send_json_to_ros_bridge(logger, host, json)
}

/// Function to stop the current sequence on the ROS bridge
fn stop_sequence(logger: &mut Logger, host: &String) -> Result<(), String> {
    let json: String = serde_json::json!(
        {
            "op": "call_service",
            "service": "/sequence/stop",
            "id": "call_service:/sequence/stop",
            "type": "std_srvs/Trigger",
            "args": {},
        }
    )
    .to_string();
    send_json_to_ros_bridge(logger, host, json)
}

/// Function to start a sequence, if at first not successful,
///
/// it will try to stop the sequence and try again one more.
pub fn run_sequence(
    host: impl Into<String>,
    procedure_name: impl Into<String>,
) -> Result<(), String> {
    let host = host.into();
    let procedure_name = procedure_name.into();
    let mut logger = Logger::default_target(format!("ros {}", host))?;

    match start_sequence(&mut logger, &host, &procedure_name) {
        Ok(()) => return Ok(()),
        Err(e) => {
            logger.warn(e)?;
            stop_sequence(&mut logger, &host)?;
            logger.info("successfully stop the sequence")?;
            start_sequence(&mut logger, &host, &procedure_name)
        }
    }
}
