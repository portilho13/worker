mod tunnel;
mod files;
mod wrangler;
mod settings;
mod helper;
mod frameworks;

use std::path::PathBuf;

fn main() {
    let local_path = helper::get_local_path();

    let local_path = local_path.display().to_string() + "/settings.toml";

    let local_path = PathBuf::from(local_path);
    
    let content = helper::read_file_content(local_path);

    let server_settings = match settings::read_toml_file(content) {
        Some(settings) => settings,
        None => {
            return;
        }
    };

    let ip = server_settings.settings.ip;
    
    tunnel::server(ip);

}
