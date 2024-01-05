use serde_json;
use websocket::{ClientBuilder, Message, OwnedMessage};

use crate::logger::{Logable, Logger};

pub struct RosBridge {
    host: String,
    logger: Logger,
    interval_ms: u64,
}

impl RosBridge {
    pub fn new(host: impl Into<String>, interval_ms: u64) -> Result<RosBridge, String> {
        let host = host.into();
        let logger = Logger::default_target(format!("ros {}", host))?;
        Ok(RosBridge {
            host,
            logger,
            interval_ms,
        })
    }
    fn make_request(&mut self, json: String) -> Result<serde_json::Value, String> {
        // The websocket URL using the provided host
        let url = format!("ws://{}:9090/", self.host);
        self.debug(format!("trying to send json to {}", url))?;

        // Attempt to connect to Websocket server until it is successful
        let mut client = loop {
            // create the Websocketclient builder
            let mut client_builder = ClientBuilder::new(&url).map_err(|e| e.to_string())?;

            self.debug("    trying to connect to websocket . . . ")?;
            if let Ok(h) = client_builder.connect_insecure() {
                self.debug("Successful connected to websocket")?;
                break h;
            }
        };

        // send the json message to call service
        self.debug("sending message . . .")?;
        self.debug(format!(">>> {}", json))?;
        let msg = Message::text(json);
        client.send_message(&msg).map_err(|e| e.to_string())?;

        // read message from websocket in loop
        loop {
            self.debug("reading message . . .")?;
            let message = client.recv_message().map_err(|e| e.to_string())?;

            match &message {
                // If the message is text
                OwnedMessage::Text(text) => {
                    self.debug(format!("<<< {}", text))?;

                    // try to pares it into json
                    let json: serde_json::Value =
                        serde_json::from_str(text).map_err(|e| e.to_string())?;
                    self.debug(format!("<<< {:?}", json))?;

                    return Ok(json);
                }
                // if the message is not in text, just log the message
                _ => {
                    self.debug(format!("<<< <<< {:?}", message))?;
                }
            }
        }
    }

    fn stop_json() -> String {
        serde_json::json!(
            {
                "op": "call_service",
                "service": "/sequence/stop",
                "id": "call_service:/sequence/stop",
                "type": "std_srvs/Trigger",
                "args": {},
            }
        )
        .to_string()
    }
    fn start_json(procedure_name: &String) -> String {
        serde_json::json!(
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
        .to_string()
    }
    fn runtime_json() -> String {
        serde_json::json!(
            {
                "op": "subscribe",
                "topic": "/sequence/runtime_state",
                "type": "commander_msgs/RuntimeState",
            }
        )
        .to_string()
    }

    fn call_service(&mut self, json: String) -> Result<(), String> {
        let value = self.make_request(json)?["values"]["success"].clone();
        match value {
            serde_json::Value::Bool(true) => Ok(()),
            _ => Err(format!("Unexpected Value : {}", value)),
        }
    }

    fn start_sequence(&mut self, procedure_name: &String) -> Result<(), String> {
        self.call_service(RosBridge::start_json(procedure_name))
    }
    fn stop_sequence(&mut self) -> Result<(), String> {
        self.call_service(RosBridge::stop_json())
    }

    pub fn run_sequence(&mut self, procedure_name: impl Into<String>) -> Result<(), String> {
        let procedure_name = procedure_name.into();
        match self.start_sequence(&procedure_name) {
            Err(_) => {
                self.stop_sequence()?;
                self.start_sequence(&procedure_name)
            }
            _ => Ok(()),
        }
    }

    pub fn get_runtime_state(&mut self) -> Result<RuntimeState, String> {
        let value = self.make_request(RosBridge::runtime_json())?["msg"]["state"].clone();
        match value {
            serde_json::Value::Number(i) => match i.as_i64() {
                Some(0) => Ok(RuntimeState::Stop),
                Some(1) => Ok(RuntimeState::Running),
                Some(2) => Ok(RuntimeState::Pause),
                Some(3) => Ok(RuntimeState::Disabled),
                _ => Err(format!("Invalid Number : {}", i)),
            },
            _ => Err(format!("Unexpected Value : {}", value)),
        }
    }

    pub fn until_sequence_stop(&mut self) -> Result<(), String> {
        loop {
            let runtime_state = self.get_runtime_state().unwrap();
            match runtime_state {
                RuntimeState::Stop => break,
                _ => {}
            }
            std::thread::sleep(std::time::Duration::from_millis(self.interval_ms));
        }
        Ok(())
    }

    pub fn run_sequence_blocking(
        &mut self,
        procedure_name: impl Into<String>,
    ) -> Result<(), String> {
        self.run_sequence(procedure_name)?;
        self.until_sequence_stop()
    }
}

impl Logable for RosBridge {
    fn get_logger(&mut self) -> &mut Logger {
        &mut self.logger
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeState {
    Stop,
    Running,
    Pause,
    Disabled,
}
