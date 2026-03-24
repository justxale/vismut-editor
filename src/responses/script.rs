use crate::database::{ScriptModel};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct AllScriptsResponse {
    files: Vec<ScriptResponse>
}

impl From<Vec<ScriptModel>> for AllScriptsResponse {
    fn from(files: Vec<ScriptModel>) -> Self {
        Self {
            files: files.into_iter().map(ScriptResponse::from).collect()
        }
    }
}

#[derive(Serialize)]
pub struct ScriptResponse {

}

impl From<ScriptModel> for ScriptResponse {
    fn from(script: ScriptModel) -> Self {
        Self {}
    }
}