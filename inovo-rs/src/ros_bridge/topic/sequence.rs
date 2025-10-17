use super::super::*;

#[inovo_topic("/sequence/blockly", commander_msgs::Blockly)]
pub struct Blocky;

#[inovo_topic("/sequence/configurations_match", std_msgs::Bool)]
pub struct ConfigurationsMatch;

#[inovo_topic("/sequence/errors", commander_msgs::BlockError)]
pub struct Errors;

#[inovo_topic("/sequence/log", commander_msgs::BlockLog)]
pub struct Log;

// TODO ParameterDescriptions
// TODO ParameterUpdates

#[inovo_topic("/sequence/prompt", commander_msgs::Prompt)]
pub struct Prompt;

#[inovo_topic("/sequence/runtime_state", commander_msgs::RuntimeState)]
pub struct RuntimeState;

#[inovo_topic("/sequence/scene", commander_msgs::Blockly)]
pub struct Scene;
