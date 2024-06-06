//! Module handling ROSbridge communication
//! ```no_run
//! use inovo_rs::ros_bridge::*;
//!
//! let mut ros_bridge = RosBridge::new("psu002", 500);
//!
//! ros_bridge.run_sequence("some sequence").unwrap();
//! ```

use serde_json;
use websocket::{ClientBuilder, Message, OwnedMessage, WebSocketError};

use crate::logger::{Logable, Logger};

/// Data structure for ROSbridge communication
///
/// ## Example
/// ```no_run
/// use inovo_rs::ros_bridge::*;
///
/// let mut ros_bridge = RosBridge::new("psu002", 500);
///
/// // to start a sequence
/// ros_bridge.start_sequence("some sequence").unwrap();
///
/// // to stop the robot runtime
/// ros_bridge.stop_sequence().unwrap();
///
/// // to start a sequence
/// // if the runtime is currently running
/// // stop the runtime before starting the sequence
/// ros_bridge.run_sequence("some sequence").unwrap();
/// ```
pub struct RosBridge {
    host: String,
    logger: Logger,
    interval_ms: u64,
}

impl RosBridge {
    /// Create a new structure for ros bridge communication
    ///
    /// ## Argument
    /// - `host`: host of the psu
    /// - `interval_ms`: retry interval
    pub fn new(host: impl Into<String>, interval_ms: u64) -> RosBridge {
        let host = host.into();
        let logger = Logger::default_target(format!("ros {}", host));
        RosBridge {
            host,
            logger,
            interval_ms,
        }
    }

    fn make_request(&mut self, json: String) -> Result<serde_json::Value, RosBridgeError> {
        // The websocket URL using the provided host
        let url = format!("ws://{}:9090/", self.host);
        self.debug(format!("trying to send json to {}", url));

        // Attempt to connect to Websocket server until it is successful
        let mut client = ClientBuilder::new(&url).unwrap().connect_insecure()?;
        self.debug("Successful connected to websocket");

        // send the json message to call service
        self.debug("sending message . . .");
        self.debug(format!(">>> {}", json));
        let msg = Message::text(json);
        client.send_message(&msg)?;

        // read message from websocket in loop
        loop {
            self.debug("reading message . . .");
            let message = client.recv_message()?;

            match &message {
                // If the message is text
                OwnedMessage::Text(text) => {
                    self.debug(format!("<<< {}", text));

                    // try to pares it into json
                    match serde_json::from_str(text) {
                        Ok(json) => {
                            self.debug(format!("<<< {:?}", json));
                            return Ok(json);
                        }
                        _ => self.error("Invaild json."),
                    }
                }
                // if the message is not in text, just log the message
                _ => {
                    self.debug(format!("<<< <<< {:?}", message));
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
    fn start_json(procedure_name: String) -> String {
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

    fn call_service(&mut self, json: String) -> Result<(), RosBridgeError> {
        let value = self.make_request(json)?["values"]["success"].clone();
        match value {
            serde_json::Value::Bool(true) => Ok(()),
            _ => Err(RosBridgeError::UnexpectedValue),
        }
    }

    /// start a sequence in the runtime
    ///
    /// ## Argument
    /// - `procedure_name`: function to start
    ///
    /// ## Error
    /// this function error if the runtime is currently not in stop state,
    /// i.e. running, pausing, error
    pub fn start_sequence(
        &mut self,
        procedure_name: impl Into<String>,
    ) -> Result<(), RosBridgeError> {
        let procedure_name = procedure_name.into();
        self.call_service(RosBridge::start_json(procedure_name))
    }

    /// stop the runtime
    pub fn stop_sequence(&mut self) -> Result<(), RosBridgeError> {
        self.call_service(RosBridge::stop_json())
    }

    /// start a sequence in the runtime.
    ///
    /// if the runtime is not at stop state, i.e. running, pausing,
    /// it will try to stop the runtime first and start the sequence
    ///
    /// ## Argument
    /// - `procedure_name`: function to start
    pub fn run_sequence(
        &mut self,
        procedure_name: impl Into<String>,
    ) -> Result<(), RosBridgeError> {
        let procedure_name = procedure_name.into();
        match self.start_sequence(&procedure_name) {
            Err(_) => {
                self.stop_sequence()?;
                self.start_sequence(&procedure_name)
            }
            _ => Ok(()),
        }
    }

    /// get the current runtime state
    pub fn get_runtime_state(&mut self) -> Result<RuntimeState, RosBridgeError> {
        let value = self.make_request(RosBridge::runtime_json())?["msg"]["state"].clone();
        match value {
            serde_json::Value::Number(i) => match i.as_i64() {
                Some(0) => Ok(RuntimeState::Stop),
                Some(1) => Ok(RuntimeState::Running),
                Some(2) => Ok(RuntimeState::Pause),
                Some(3) => Ok(RuntimeState::Disabled),
                _ => Err(RosBridgeError::UnexpectedValue),
            },
            _ => Err(RosBridgeError::UnexpectedValue),
        }
    }

    /// wait until the runtime finish running current sequence,
    ///
    /// it will keep waiting if the sequence is pause or error.
    pub fn until_sequence_stop(&mut self) -> Result<(), RosBridgeError> {
        loop {
            let runtime_state = self.get_runtime_state()?;
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
    ) -> Result<(), RosBridgeError> {
        self.run_sequence(procedure_name)?;
        self.until_sequence_stop()
    }
}

impl Logable for RosBridge {
    fn get_logger(&mut self) -> &mut Logger {
        &mut self.logger
    }
}

/// Runtime state of the robot
#[derive(Debug, Clone)]
pub enum RuntimeState {
    Stop,
    Running,
    Pause,
    Disabled,
}

/// ROS bridge related error
#[derive(Debug, thiserror::Error)]
pub enum RosBridgeError {
    #[error(transparent)]
    WebSocketError(#[from] WebSocketError),
    #[error("Unexpected Value")]
    UnexpectedValue,
}
