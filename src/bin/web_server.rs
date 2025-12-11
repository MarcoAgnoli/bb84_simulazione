use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use tokio_stream::wrappers::BroadcastStream;
use futures_util::StreamExt;
use tokio::io::AsyncBufReadExt;
use serde::Deserialize;
use tokio::sync::broadcast;

#[derive(Deserialize)]
struct RunRequest {
    lung_msg: usize,
    attiva_avversario: bool,
}

async fn index() -> impl Responder {
    let html = r#"
<!doctype html>
<html>
<head>
    <meta charset=\"utf-8\"> 
    <title>BB84 Simulazione - Web UI</title>
    <style>body{font-family:Segoe UI,Arial;margin:16px}#log{white-space:pre-wrap;background:#111;color:#0f0;padding:12px;height:400px;overflow:auto;border-radius:6px}</style>
</head>
<body>
    <h1>BB84 Simulazione - Web UI</h1>
    <form id=\"cfg\"> 
    <label>Lunghezza stream (LUNG_MSG): <input id=\"lung\" type=\"number\" value=\"64\"></label>
    <label style=\"margin-left:12px\">Avversario attivo: <input id=\"adv\" type=\"checkbox\" checked></label>
    <button type=\"submit\" style=\"margin-left:12px\">Esegui simulazione</button>
    </form>
    <h2>Log</h2>
    <div id=\"log\"></div>

    <script>
        const log = document.getElementById('log');
        const form = document.getElementById('cfg');
    function appendLine(s){ log.textContent += s + '\n'; log.scrollTop = log.scrollHeight; }
    const es = new EventSource('/events');
    es.onmessage = (e) => appendLine(e.data);
        es.onerror = () => appendLine('[EventSource] connection error');
        form.addEventListener('submit', async (ev)=>{
            ev.preventDefault();
    log.textContent = '';
    const body = { lung_msg: Number(document.getElementById('lung').value), attiva_avversario: document.getElementById('adv').checked };
    appendLine('[UI] Starting simulation with ' + JSON.stringify(body));
            await fetch('/run', { method: 'POST', headers: {'Content-Type':'application/json'}, body: JSON.stringify(body) });
        });
    </script>
</body>
</html>
"#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

struct AppState {
    tx: broadcast::Sender<String>,
}

async fn events(data: Data<AppState>) -> impl Responder {
    let rx = data.tx.subscribe();
    let stream = BroadcastStream::new(rx).map(|res| {
        match res {
            Ok(msg) => Ok::<_, actix_web::Error>(web::Bytes::from(format!("data: {}\n\n", msg))),
            Err(_) => Ok(web::Bytes::from("data: [disconnected]\n\n")),
        }
    });
    HttpResponse::Ok()
        .append_header(("Content-Type", "text/event-stream"))
        .streaming(stream)
}

async fn run_sim(data: Data<AppState>, cfg: web::Json<RunRequest>) -> impl Responder {
    let tx = data.tx.clone();
    let lung = cfg.lung_msg.to_string();
    let adv = if cfg.attiva_avversario { "true" } else { "false" };

    // Determine path to simulation executable
    let exe = {
        let mut path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        path.push("target");
        path.push("debug");
        let mut name = "bb84_simulazione".to_string();
        if cfg!(windows) { name.push_str(".exe"); }
        path.push(name);
        path
    };

    let tx2 = tx.clone();
    tokio::spawn(async move {
        let mut cmd = tokio::process::Command::new(exe);
        cmd.env("LUNG_MSG", &lung);
        cmd.env("ATTIVA_AVVERSARIO", &adv);
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        match cmd.spawn() {
            Ok(mut child) => {
                let mut handles = Vec::new();
                if let Some(out) = child.stdout.take() {
                    let mut reader = tokio::io::BufReader::new(out).lines();
                    let txc = tx2.clone();
                    let h = tokio::spawn(async move {
                        while let Ok(Some(line)) = reader.next_line().await {
                            let _ = txc.send(line);
                        }
                    });
                    handles.push(h);
                }
                if let Some(err) = child.stderr.take() {
                    let mut reader = tokio::io::BufReader::new(err).lines();
                    let txc = tx2.clone();
                    let h = tokio::spawn(async move {
                        while let Ok(Some(line)) = reader.next_line().await {
                            let _ = txc.send(line);
                        }
                    });
                    handles.push(h);
                }
                let _ = child.wait().await;
                for h in handles { let _ = h.await; }
            }
            Err(e) => {
                let _ = tx2.send(format!("[web_server] failed to spawn child: {}", e));
            }
        }
    });

    HttpResponse::Accepted().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (tx, _rx) = broadcast::channel(1024);
    let data = Data::new(AppState { tx });

    println!("[web_server] Starting web UI on http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index))
            .route("/run", web::post().to(run_sim))
            .route("/events", web::get().to(events))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
