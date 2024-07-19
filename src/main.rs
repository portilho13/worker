use std::path::PathBuf;

mod tunnel;
mod helper;
mod settings;
mod files;
mod wrangler;
mod frameworks;
mod api;

#[tokio::main]
async fn main() {
    let local_path = helper::get_local_path();

    let local_path = local_path.await.display().to_string() + "/settings.toml";

    let local_path = PathBuf::from(local_path);
    
    let content = helper::read_file_content(local_path).await;

    let server_settings = match settings::read_toml_file(content) {
        Some(settings) => settings,
        None => {
            return;
        }
    };

    let ip = server_settings.conn.ip;

    let api_ip = server_settings.api.ip;

    tokio::join!(
        api::api::api(&api_ip),
        tunnel::tunnel::server(&ip)
    );
}
