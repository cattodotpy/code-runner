mod config;
mod runner;
mod types;

use std::{env, io, path::PathBuf};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use config::Config;
use runner::Runner;
use serde_json::{Value, json};
use tokio::fs;
use types::{ExecuteData, Limit, RunOutput, RunStatus};

const PROGRAM_NAME: &str = "program";

#[derive(Clone)]
struct AppState {
    config: Config,
}

async fn execute(
    State(AppState { config }): State<AppState>,
    data: Json<ExecuteData>,
) -> (StatusCode, Json<RunOutput>) {
    if !config.languages.contains_key(&data.language) {
        return (
            StatusCode::BAD_REQUEST,
            Json(RunOutput::error("Unknown language".to_string(), None, None)),
        );
    }

    let path = config.code_dir.clone();
    let path = PathBuf::from(path);

    let mut box_name = (0..20)
        .map(|_| fastrand::alphanumeric())
        .take(20)
        .collect::<String>();

    while path.join(&box_name).exists() {
        box_name = (0..20)
            .map(|_| fastrand::alphanumeric())
            .take(20)
            .collect::<String>();
    }

    let box_path = path.join(&box_name);

    if let Err(e) = fs::create_dir_all(&box_path).await {
        eprintln!("Unable to create directory: {}", e.to_string());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RunOutput::error(
                format!("Unable to create directory: {}", e.to_string()),
                None,
                None,
            )),
        );
    }

    let mut runner = Runner::new(box_path.to_str().unwrap());

    let language = config.languages.get(&data.language).unwrap();

    let file = box_path.join(format!("{}.{}", PROGRAM_NAME, language.extension));
    if let Err(e) = fs::write(&file, &data.code).await {
        eprintln!("Unable to write file: {}", e.to_string());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RunOutput::error(
                format!("Unable to write file: {}", e.to_string()),
                None,
                None,
            )),
        );
    }

    if let Some(compile) = language.compile.clone() {
        let mut iter = compile.iter();

        let command = iter.next().unwrap();
        let args = iter.map(|s| s.to_string()).collect::<Vec<String>>();

        let compile_out = runner.execute_program(
            command,
            args,
            Some(Limit {
                time_limit: Some(4),
                memory: Some(512 * 1024 * 1024),
                walltime_limit: Some(8),
            }),
            None,
        );

        let compilation_error = match compile_out.status {
            RunStatus::Success => false,
            _ => true,
        };

        if compilation_error {
            return (
                StatusCode::OK,
                Json(RunOutput {
                    status: RunStatus::CompileError,
                    stderr: compile_out.stderr,
                    stdout: compile_out.stdout,
                    runtime: compile_out.runtime,
                    memory_usage: compile_out.memory_usage,
                }),
            );
        }
    }

    let run = language.run.clone();

    let mut run_iter = run.iter();
    let run_command = run_iter.next().unwrap();
    let run_args = run_iter.map(|s| s.to_string()).collect::<Vec<String>>();

    let result = runner.execute_program(
        run_command,
        run_args,
        Some(Limit {
            time_limit: data.time_limit,
            memory: data.memory_limit,
            walltime_limit: data.wall_time_limit,
        }),
        if data.input.is_some() {
            Some(data.input.as_ref().unwrap().as_bytes().to_vec())
        } else {
            None
        },
    );

    let status = match result.status {
        RunStatus::SystemError(_) | RunStatus::UnknownError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::OK,
    };

    println!("Execution completed with status: {:?}", status);

    (status, Json(result))
}

async fn get_languages(State(AppState { config }): State<AppState>) -> Json<Value> {
    let languages = config
        .languages
        .keys()
        .map(|key| key.to_string())
        .collect::<Vec<String>>();

    Json(json!({
        "languages": languages
    }))
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let config: Option<String> = fs::read_to_string(
        env::args()
            .nth(1)
            .unwrap_or("/etc/code-runner/config.toml".to_string()),
    )
    .await
    .ok();

    if config.is_none() {
        eprintln!("Unable to read config file.");
        return;
    }

    let config: Result<Config, _> = toml::from_str(&config.unwrap());

    if config.is_err() {
        eprintln!("Unable to read config file, wrong format.");
        return;
    }

    let config = config.unwrap();

    let languages_len = config.languages.len();

    let addr = config.address.clone().unwrap_or("0.0.0.0".to_string());
    let port = config.port.clone().unwrap_or(8080);

    let app = Router::new()
        .route("/code", post(execute))
        .route("/languages", get(get_languages))
        .with_state(AppState { config });

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", addr, port))
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    println!("running with {} languages", languages_len);
    axum::serve(listener, app).await.unwrap();
}
