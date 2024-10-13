use std::sync::Arc;

use crate::application::http::handlers::create_professional::create_professional;
use anyhow::Context;
use axum::{routing::post, Extension};
use tokio::net;
use tracing::{info, info_span};

use crate::domain::professional::ports::ProfessionalService;

mod handlers;
mod responses;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
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
    pub async fn new<'a, P>(
        config: HttpServerConfig<'a>,
        professional_service: Arc<P>,
    ) -> anyhow::Result<Self>
    where
        P: ProfessionalService + Send + Sync + 'a,
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

        let router = axum::Router::new()
            .nest("/api", api_routes())
            .layer(trace_layer)
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
    axum::Router::new().route("/professionals", post(create_professional::<P>))
}
