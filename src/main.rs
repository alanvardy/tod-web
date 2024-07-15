use std::sync::Arc;

use axum::{extract::State, response::Html, routing::get, Router};
use minijinja::{context, path_loader, Environment, Value};
use serde::Serialize;

#[derive(Clone)]
struct AppState {
    env: Arc<Environment<'static>>,
}
#[derive(Serialize)]
struct Link {
    name: String,
    href: String,
}

#[derive(Serialize)]
struct Config {
    name: String,
    data_type: String,
    default_value: String,
    description: String,
}

pub const BOOLEAN: &str = "boolean";

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    let state = AppState { env: Arc::new(env) };

    let router = Router::new()
        // Routes
        .route("/", get(index))
        .route("/configuration", get(configuration))
        // State
        .with_state(state);

    Ok(router.into())
}

async fn index(State(app_state): State<AppState>) -> Html<String> {
    let context = context!(title => "Tod", navigation => get_nav());

    render_template(app_state, "index.html", context).await
}

async fn configuration(State(app_state): State<AppState>) -> Html<String> {
    let context =
        context!(title => "Configuration", navigation => get_nav(), configuration => get_config());
    render_template(app_state, "configuration.html", context).await
}

async fn render_template(app_state: AppState, name: &str, context: Value) -> Html<String> {
    let tmpl = app_state.env.get_template(name).unwrap();
    Html(tmpl.render(context).unwrap())
}

fn get_config() -> Vec<Config> {
    let mut config = vec![
        Config {
            name: "bell_on_success".into(),
            data_type: BOOLEAN.into(),
            default_value: "false".into(),
            description: "Triggers the terminal bell on successful completion of a command".into(),
        },
        Config {
            name: "bell_on_failure".into(),
            data_type: BOOLEAN.into(),
            default_value: "true".into(),
            description: "Triggers the terminal bell on an error".into(),
        },
        Config {
            name: "disable_links".into(),
            data_type: BOOLEAN.into(),
            default_value: "false".into(),
            description: "If true, disables OSC8 linking and just displays plain text".into(),
        },
        Config {
            name: "last_version_check".into(),
            data_type: "nullable string".into(),
            default_value: "null".into(),
            description: "Holds a string date, i.e. `\"2023-08-30\"` representing the last time crates.io was checked for the latest `tod` version. Tod will check crates.io a maximum of once per day.".into(),
        },

        // ### next_id

        // ```
        //   type: nullable string
        //   default: null
        //   possible values: null or any positive integer in string form
        // ```

        // When `task next` is executed the ID is stored in this field. When `task complete` is run the field is set back to `null`

        // ### path

        // ```
        //   type: string
        //   default: $XDG_CONFIG_HOME/tod.cfg
        //   possible values: Any path
        // ```

        // Location of the `tod` configuration file

        // ### natural_language_only

        // ```
        //   type: nullable boolean
        //   default: null
        //   possible values: null, true, or false
        // ```

        // If true, the datetime selection in `project schedule` will go straight to natural language input.

        // ### no_sections

        // ```
        //   type: nullable boolean
        //   default: null
        //   possible values: null, true, or false
        // ```

        // If true will not prompt for a section whenever possible

        // ### sort_value

        // Tasks are ranked by points and the first is returned, the points are the sum of the following:

        //   - Task is overdue: 150
        //   - The date is today with no time: 100
        //   - The date is today with time in next or last 15 min: 200
        //   - No date: 80
        //   - Not recurring: 50
        //   - Task has no priority: 2
        //   - Priority 1: 1
        //   - Priority 2: 3
        //   - Priority 3: 4

        // Defaults:

        // ```
        //   {
        //     "no_due_date": 80,
        //     "not_recurring": 50,
        //     "now": 200,
        //     "overdue": 150,
        //     "priority_high": 4,
        //     "priority_low": 1,
        //     "priority_medium": 3,
        //     "priority_none": 2,
        //     "today": 100
        //   },
        // ```

        // ### spinners

        // ```
        //   type: nullable boolean
        //   default: null
        //   possible values: null, true, or false
        // ```

        // Controls whether the spinner is displayed when an API call occurs. Useful for cases where the terminal output is captured. `null` is considered the same as `true`.

        // You can also use the environment variable `DISABLE_SPINNER` to turn them off.

        // ```bash
        //   DISABLE_SPINNER=1 tod task create
        // ```

        // ### timeout

        // ```
        //   type: integer
        //   default: 30 (seconds)
        //   possible values: Any positive number in seconds
        // ```

        // ### timezone

        // ```
        //   type: string
        //   default: No default
        //   possible values: Any timezone string i.e. "Canada/Pacific"
        // ```

        // You will be prompted for timezone on first run

        // ### token

        // ```
        //   type: string
        //   default: No default
        //   possible values: Any valid token
        // ```

        // You will be prompted for your [Todoist API token](https://todoist.com/prefs/integrations) on first run

        // ### vecprojects

        // ```
        //   type: Nullable array of objects
        //   default: null
        //   possible values: List of project objects from the Todoist API
        // ```

        // Projects are stored locally in config to help save on API requests and speed up actions taken. Manage this with the `project` subcommands. The strange naming is because `projects` was used in previous versions of `tod`.

        // ### verbose

        // ```
        //   type: nullable boolean
        //   default: null
        //   possible values: null, true, or false
        // ```

        // Outputs additional information in console to assist with debugging.
    ];

    config.sort_by_key(|c| c.name.clone());
    config
}

fn get_nav() -> Vec<Link> {
    vec![
        Link {
            href: "/".into(),
            name: "Tod".into(),
        },
        Link {
            href: "/configuration".into(),
            name: "Configuration".into(),
        },
    ]
}
