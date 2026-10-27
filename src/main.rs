use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
mod vos;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Severity {
    High,
    Low,
    Medium,
}

fn severity_from_str(severity: &str) -> Result<Severity, String> {
    match severity.trim().to_lowercase().as_str() {
        "high" => Ok(Severity::High),
        "low" => Ok(Severity::Low),
        "medium" => Ok(Severity::Medium),
        _ => Err("Opção não disponível".to_owned()),
    }
}

#[derive(Deserialize)]
struct CreateOri {
    asset_id: String,
    description: String,
    severity: String,
    reported_by: Option<String>,
}

#[derive(Serialize)]
struct Ori {
    asset_id: vos::AssetId,
    description: vos::Description,
    severity: Severity,
    reported_by: vos::ReportedBy,
}
impl Ori {
    fn new(
        asset_id: &str,
        description: &str,
        severity: &str,
        reported_by: &Option<String>,
    ) -> Result<Self, String> {
        let asset_id_in = vos::AssetId::new(asset_id)?;
        let severity_in = severity_from_str(severity)?;
        let description_in = vos::Description::new(description)?;
        let reported_by_in = vos::ReportedBy::new(reported_by);

        Ok(Self {
            asset_id: asset_id_in,
            description: description_in,
            severity: severity_in,
            reported_by: reported_by_in,
        })
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/ori", post(create_ori));

    let listener = match TcpListener::bind("127.0.0.1:3000").await {
        Ok(l) => l,
        Err(e) => {
            println!("{:#?}", e);
            return;
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => (),
        Err(e) => println!("{:#?}", e),
    }
}

async fn root() -> &'static str {
    "ORI Running..."
}

async fn create_ori(
    Json(payload): Json<CreateOri>,
) -> Result<(StatusCode, Json<Ori>), (StatusCode, String)> {
    let ori = Ori::new(
        &payload.asset_id,
        &payload.description,
        &payload.severity,
        &payload.reported_by,
    )
    .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e))?;

    Ok((StatusCode::CREATED, Json(ori)))
}
