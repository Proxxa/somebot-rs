pub use anyhow::Context as _;
pub use poise::serenity_prelude as serenity;
use poise::FrameworkBuilder;
pub use shuttle_poise::ShuttlePoise;
pub use shuttle_secrets::SecretStore;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = StdResult<T, Error>;

pub struct PoiseRocketService {
    pub poise:
        FrameworkBuilder<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>,
    pub rocket: shuttle_rocket::RocketService,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for PoiseRocketService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> std::result::Result<(), shuttle_runtime::Error> {
        let binder = self.rocket.bind(addr);

        tokio::select! {
            _ = self.poise.run() => {},
            _ = binder => {},
        }

        Ok(())
    }
}
