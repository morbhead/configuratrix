use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct SudoOptions {
    pub userconfigs: Vec<SudoUserConfig>,
    pub defaults: SudoDefaults,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SudoDefaults {
    pub pwfeedback: Option<bool>,
    pub insult: Option<bool>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SudoUserConfig {
    pub name: String,
    pub is_group: bool,
    pub nopasswd: bool,

    pub act_on_host: String,
    pub act_as_user: String,
    pub act_as_group: String,

    pub commands: Vec<String>,
}