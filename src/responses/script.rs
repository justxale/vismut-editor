use crate::database::{ScriptModel};
use serde::Serialize;
use uuid::Uuid;

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
    id: Uuid,
    title: String,
    content: Option<String>,
}

impl From<ScriptModel> for ScriptResponse {
    fn from(script: ScriptModel) -> Self {
        Self { id: script.id, title: script.title, content: script.content }
    }
}