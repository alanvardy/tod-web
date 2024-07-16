use askama_axum::Template;
use axum::{response::Html, routing::get, Router};
use serde::Serialize;

use crate::Link;

pub const BOOLEAN: &str = "boolean";
pub const NULLABLE_BOOLEAN: &str = "boolean";
pub const INTEGER: &str = "integer";
pub const STRING: &str = "boolean";
pub const NULLABLE_STRING: &str = "nullable string";

pub fn routes() -> Router {
    Router::new().route("/configuration", get(configuration))
}

#[derive(Serialize)]
struct Config {
    name: String,
    data_type: String,
    default_value: String,
    possible_values: String,
    description: String,
}

#[derive(Template)]
#[template(path = "configuration.html")]
struct ConfigurationTemplate {
    title: String,
    navigation: Vec<Link>,
    configuration: Vec<Config>,
}

async fn configuration() -> Html<String> {
    let template = ConfigurationTemplate {
        title: "Configuration".into(),
        navigation: crate::get_nav(),
        configuration: get_config(),
    };

    Html(template.render().unwrap())
}

fn get_config() -> Vec<Config> {
    let mut config = vec![
                Config {
                    name: "bell_on_success".into(),
                    data_type: BOOLEAN.into(),
                    default_value: "false".into(),
                    possible_values: "true or false".into(),
                    description: "Triggers the terminal bell on successful completion of a command".into(),
                },
                Config {
                    name: "bell_on_failure".into(),
                    data_type: BOOLEAN.into(),
                    default_value: "true".into(),
                    possible_values: "true or false".into(),
                    description: "Triggers the terminal bell on an error".into(),
                },
                Config {
                    name: "disable_links".into(),
                    data_type: BOOLEAN.into(),
                    default_value: "false".into(),
                    possible_values: "true or false".into(),
                    description: "If true, disables OSC8 linking and just displays plain text".into(),
                },
                Config {
                    name: "last_version_check".into(),
                    data_type: NULLABLE_STRING.into(),
                    default_value: "null".into(),
                    possible_values: "null or a string in format YYYY-MM-DD".into(),
                    description: "Holds a string date, i.e. `\"2023-08-30\"` representing the last time crates.io was checked for the latest `tod` version. Tod will check crates.io a maximum of once per day.".into(),
                },

                Config {
                    name: "next_id".into(),
                    data_type: NULLABLE_STRING.into(),
                    default_value: "null".into(),
                    possible_values: "null or any positive integer in string form".into(),
                    description: "When `task next` is executed the ID is stored in this field. When `task complete` is run the field is set back to `null`
                    ".into(),
               },

                Config {
                    name: "path".into(),
                    data_type: STRING.into(),
                    default_value: "$XDG_CONFIG_HOME/tod.cfg".into(),
                    possible_values: "any path".into(),
                    description: "Location of the tod configuration file".into(),
                },
                Config {
                    name: "natural_language_only".into(),
                    data_type: NULLABLE_BOOLEAN.into(),
                    default_value: "null".into(),
                    possible_values: "null, true, or false".into(),
                    description: "If true, the datetime selection in `project schedule` will go straight to natural language input.".into(),
                },

                Config {
                    name: "no_sections".into(),
                    data_type: NULLABLE_BOOLEAN.into(),
                    default_value: "null".into(),
                    possible_values: "null, true, or false".into(),
                    description: "If true, will not prompt for a section whenever possible.".into(),
                },

                Config {
                    name: "spinners".into(),
                    data_type: NULLABLE_BOOLEAN.into(),
                    default_value: "null".into(),
                    possible_values: "null, true, or false".into(),
                    description: "Controls whether the spinner is displayed when an API call occurs. Useful for cases where the terminal output is captured. `null` is considered the same as `true`. You can also use the environment variable DISABLE_SPINNER to turn them off, i.e. `DISABLE_SPINNER=1 tod task create`".into(),
                },

                Config {
                    name: "timeout".into(),
                    data_type: INTEGER.into(),
                    default_value: "30 (seconds)".into(),
                    possible_values: "any positive integer in seconds".into(),
                    description: "Time before an HTTP request is cancelled".into(),
                },
                Config {
                    name: "timezone".into(),
                    data_type: STRING.into(),
                    default_value: "No default".into(),
                    possible_values: "Any timezone string i.e. \"Canada/Pacific\"".into(),
                    description: "You will be prompted for a timezone on first run. This is used in cases where the API does not specify the timezone explicitly.".into(),
                },
                Config {
                    name: "token".into(),
                    data_type: STRING.into(),
                    default_value: "No default".into(),
                    possible_values: "Any valid Todoist API token".into(),
                    description: "You will be prompted for your [Todoist API token](https://todoist.com/prefs/integrations) on first run".into(),
                },
                Config {
                    name: "vecprojects".into(),
                    data_type: "Nullable array of objects".into(),
                    default_value: "null".into(),
                    possible_values: "null or a list of project objects".into(),
                    description: "Projects are stored locally in config to help save on API requests and speed up actions taken. Manage this with the `project` subcommands. The strange naming is because `projects` was used in previous versions of `tod`".into(),
                },
                Config {
                    name: "verbose".into(),
                    data_type: NULLABLE_BOOLEAN.into(),
                    default_value: "null".into(),
                    possible_values: "null, true, or false".into(),
                    description: "Outputs additional information in console to assist with debugging".into(),
                },
                Config {
                    name: "sort_value".into(),
                    data_type: "object".into(),
                    default_value: "{
                             \"no_due_date\": 80,
                             \"not_recurring\": 50,
                             \"now\": 200,
                             \"overdue\": 150,
                             \"priority_high\": 4,
                             \"priority_low\": 1,
                             \"priority_medium\": 3,
                             \"priority_none\": 2,
                             \"today\": 100
                           },
            ".into(), 
                    possible_values: "see default value".into(),
                    description: "Tasks are valued as the sum of the categories that apply.".into(),
                },
            Config {
                name: "sort_value.no_due_date".into(),
                data_type: "integer".into(),
                default_value: "80".into(), 
                possible_values: "any non-negative integer".into(),
                description: "Adds the value to task when sorting if the task has no due date or time on it.".into(),
            },
            Config {
                name: "sort_value.no_due_date".into(),
                data_type: "integer".into(),
                default_value: "80".into(), 
                possible_values: "any non-negative integer".into(),
                description: "Adds the value to task when sorting if the task has no due date or time on it.".into(),
            },
            Config {
                name: "sort_value.overdue".into(),
                data_type: "integer".into(),
                default_value: "150".into(), 
                possible_values: "any non-negative integer".into(),
                description: "Adds the value to task when sorting if the task has a due date in the past.".into(),
            },
            Config {
                name: "sort_value.not_recurring".into(),
                data_type: "integer".into(),
                default_value: "50".into(), 
                possible_values: "any non-negative integer".into(),
                description: "Adds the value to task when sorting if the task is not recurring.".into(),
            },
            Config {
                name: "sort_value.now".into(),
                data_type: "integer".into(),
                default_value: "200".into(), 
                possible_values: "any non-negative integer".into(),
                description: "Adds the value to task when sorting if the task is scheduled within the next or last 15 minutes.".into(),
            },
            Config {
                name: "sort_value.today".into(),
                data_type: "integer".into(),
                default_value: "100".into(), 
                possible_values: "any non-negative integer".into(),
                description: "Adds the value to task when sorting if the task is scheduled today but does not have a time.".into(),
            },
            Config {
                name: "sort_value.priority_none".into(),
                data_type: "integer".into(),
                default_value: "2".into(), 
                possible_values: "any non-negative integer".into(),
                description: "Adds the value to task when sorting if the task has no priority.".into(),
            },
            Config {
                name: "sort_value.priority_low".into(),
                data_type: "integer".into(),
                default_value: "1".into(), 
                possible_values: "any non-negative integer".into(),
                description: "Adds the value to task when sorting if the task has a low priority.".into(),
            },
            Config {
                name: "sort_value.priority_medium".into(),
                data_type: "integer".into(),
                default_value: "3".into(), 
                possible_values: "any non-negative integer".into(),
                description: "Adds the value to task when sorting if the task has a medium priority priority.".into(),
            },
            Config {
                name: "sort_value.priority_high".into(),
                data_type: "integer".into(),
                default_value: "4".into(), 
                possible_values: "any non-negative integer".into(),
                description: "Adds the value to task when sorting if the task has a high priority.".into(),
            },

        ];

    config.sort_by_key(|c| c.name.clone());
    config
}
