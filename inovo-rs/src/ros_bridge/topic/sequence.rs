use super::super::*;

// TODO Blocky

// TODO ConfigurationsMatch

#[inovo_topic("/sequence/errors", commander_msgs::BlockError)]
pub struct Errors;

#[inovo_topic("/sequence/log", commander_msgs::BlockLog)]
pub struct Log;

// TODO ParameterDescriptions
// TODO ParameterUpdates
// TODO Prompt

#[inovo_topic("/sequence/runtime_state", commander_msgs::RuntimeState)]
pub struct RuntimeState;

// TODO Scene
