mod runner;
mod types;

use std::path::PathBuf;

use rocket::post;
use rocket::serde::json::Json;
use rocket::{launch, routes, tokio::fs};
use runner::Runner;
use types::{ExecuteData, Limit, RunOutput};

#[post("/code", data = "<data>")]
async fn execute(data: Json<ExecuteData<'_>>) -> Json<RunOutput> {
    let path = std::env::args().nth(1);

    if path.is_none() {
        return Json(RunOutput::error("No path provided".to_string(), None, None));
    }

    let path = PathBuf::from(path.unwrap());

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

    fs::create_dir(&box_path).await.ok();

    let mut runner = Runner::new(box_path.to_str().unwrap());

    let execute_data = data.into_inner();

    // assuming its a python3 code

    let file = box_path.join("main.py");

    fs::write(file, execute_data.code).await.ok();

    let result = runner.execute_program(
        "/bin/python3",
        vec!["main.py".to_string()],
        Some(Limit {
            walltime_limit: Some(execute_data.wall_time_limit),
            time_limit: Some(execute_data.time_limit),
            memory: Some(execute_data.memory_limit),
        }),
        Some(execute_data.input.as_bytes().to_vec()),
    );

    Json(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![execute])
}
