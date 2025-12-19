use std::{
    io::Write,
    process::{Command, Stdio},
    thread,
};

use actix_web::{HttpRequest, HttpResponse, Responder, http::StatusCode, post, web};

#[derive(serde::Deserialize)]
struct PostQuery {
    query: String,
}

#[post("/api/raw_sql")]
pub async fn raw_sql(_req: HttpRequest, body: web::Json<PostQuery>) -> impl Responder {
    let mut child = Command::new("sqlite3")
        .arg("./data.db")
        .arg("-batch")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b".headers ON\n").unwrap();
        stdin.write_all(body.query.as_bytes()).unwrap();
        stdin.write_all(b"\n.quit\n").unwrap();
    }

    match thread::spawn(move || child.wait_with_output())
        .join()
        .unwrap()
    {
        Ok(output) => {
            return HttpResponse::Ok().body(
                String::new()
                    + &String::from_utf8_lossy(&output.stdout)
                    + &String::from_utf8_lossy(&output.stderr),
            );
        }
        Err(e) => {
            return HttpResponse::Ok()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(e.to_string());
        }
    }
}
