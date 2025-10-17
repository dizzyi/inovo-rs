use super::super::*;

#[inovo_service("/sequence/continue", std_srvs::Trigger)]
pub struct Continue;

#[inovo_service("/sequence/debug", commander_msgs::RunSequence)]
pub struct Debug;

#[inovo_service("/sequence/delete_project", commander_msgs::Project)]
pub struct DeleteProject;

#[inovo_service("/sequence/delete_var", commander_msgs::DeleteVariable)]
pub struct DeleteVar;

// TODO GetProjectConfig

#[inovo_service("/sequence/get_var", commander_msgs::GetVariable)]
pub struct GetVar;

// TODO ImportConfig

#[inovo_service("/sequence/list_projects", commander_msgs::ListProject)]
pub struct ListProject;

#[inovo_service("/sequence/new_project", commander_msgs::NewProject)]
pub struct NewProject;

#[inovo_service("/sequence/open_project", commander_msgs::ListProject)]
pub struct OpenProject;

#[inovo_service("/sequence/pause", std_srvs::Trigger)]
pub struct Pause;

// TODO Prompt

// TODO RenameProject

// TODO RevertProject

#[inovo_service("/sequence/save_project", commander_msgs::Project)]
pub struct SaveProject;

// TODO SetCurrentBlock

// TODO SetParameter

#[inovo_service("/sequence/set_var", commander_msgs::SetVariable)]
pub struct SetVar;

#[inovo_service("/sequence/start", commander_msgs::RunSequence)]
pub struct Start;

#[inovo_service("/sequence/start", commander_msgs::RunSequenceRequest)]
pub struct StartRequest;

#[inovo_service("/sequence/step", std_srvs::Trigger)]
pub struct Step;

#[inovo_service("/sequence/stop", std_srvs::Trigger)]
pub struct Stop;

// TODO UpdateConfig

// TOOD UpdateScene

// TODO Upload
