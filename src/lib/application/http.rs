use std::{sync::Arc, vec};

use crate::application::http::handlers::create_professional::create_professional;
use anyhow::Context;
use axum::{
    routing::{get, post},
    Extension,
};
use axum_keycloak_auth::{
    instance::{KeycloakAuthInstance, KeycloakConfig},
    layer::KeycloakAuthLayer,
    PassthroughMode,
};
use handlers::get_professional::get_professional;
use reqwest::Url;
use tokio::net;
use tracing::{info, info_span};

use crate::domain::professional::ports::ProfessionalService;

mod handlers;
mod layers;
mod responses;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

#[derive(Debug, Clone)]
struct AppState<P>
where
    P: ProfessionalService,
{
    professional_service: Arc<P>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new<P>(
        config: HttpServerConfig,
        professional_service: Arc<P>,
    ) -> anyhow::Result<Self>
    where
        P: ProfessionalService + Send + Sync,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = AppState {
            professional_service,
        };

        let keycloak_auth_instance = KeycloakAuthInstance::new(
            KeycloakConfig::builder()
                .server(Url::parse("http://localhost:8080/").unwrap())
                .realm(String::from("libreconnect"))
                .build(),
        );

        let auth_layer = KeycloakAuthLayer::<String>::builder()
            .instance(keycloak_auth_instance)
            .passthrough_mode(PassthroughMode::Block)
            .persist_raw_claims(false)
            .expected_audiences(vec![String::from("account")])
            .build();

        let router = axum::Router::new()
            .nest("/v1", api_routes())
            .layer(trace_layer)
            .layer(auth_layer)
            .layer(Extension(Arc::clone(&state.professional_service)))
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error while running http server")?;

        Ok(())
    }
}

fn api_routes<P>() -> axum::Router<AppState<P>>
where
    P: ProfessionalService + Send + Sync + 'static,
{
    axum::Router::new()
        .route("/professionals", post(create_professional::<P>))
        .route(
            "/professionals/:professional_id",
            get(get_professional::<P>),
        )
}
