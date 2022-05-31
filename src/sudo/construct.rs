use crate::internal::types::*;

pub fn construct(path: String, options: String) {
    let sudo_defaults = SudoOptions {
        userconfigs: vec![
            SudoUserConfig {
                name: "wheel".to_string(),
                is_group: true,
                nopasswd: false,
                act_on_host: "ALL".to_string(),
                act_as_user: "ALL".to_string(),
                act_as_group: "ALL".to_string(),
                commands: vec!["ALL".to_string()],
            }
        ],
        defaults: SudoDefaults {
            pwfeedback: Some(false),
            insult: Some(false)
        }
    };

    let options_json: SudoOptions = match serde_json::from_str(&options) {
        Ok(options_json) => options_json,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let mut options_json = match options_json {
        SudoOptions {
            userconfigs,
            defaults: SudoDefaults {
                pwfeedback,
                insult
            }
        } => {
            SudoOptions {
                userconfigs,
                defaults: SudoDefaults {
                    pwfeedback: Some(pwfeedback.unwrap_or(false)),
                    insult: Some(insult.unwrap_or(false))
                }
            }
        }
    };

    if options_json.userconfigs.is_empty() {
        options_json.userconfigs = sudo_defaults.userconfigs;
    }


    let mut config_strings: Vec<String> = vec![];

    if options_json.defaults.pwfeedback == Some(true) {
        config_strings.push("Defaults pwfeedback".to_string());
    }

    if options_json.defaults.insult == Some(true) {
        config_strings.push("Defaults insult".to_string());
    }

    for userconfig in options_json.userconfigs {
        config_strings.push(format!("{}{} {}=({}:{}) {}{}",
            if userconfig.is_group { "%" } else { "" },
            userconfig.name,
            userconfig.act_on_host,
            userconfig.act_as_user,
            userconfig.act_as_group,
            if userconfig.nopasswd == true { "NOPASSWD: "} else { "" },
            userconfig.commands.join(" ")
            ).to_string()
        );
    }

    let config_string = config_strings.join("\n");
    println!();
    println!("{}", config_string);
    println!();
}