use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub database_url: String,
    #[clap(env)]
    pub database_user: String,
    #[clap(env)]
    pub database_password: String,

    #[clap(env)]
    pub keycloak_url: String,
    #[clap(env)]
    pub keycloak_realm: String,

    #[clap(env)]
    pub nats_url: String,

    #[clap(env)]
    pub port: String,
}
