use std::{path::PathBuf, process::Command};

use super::{ProjectConfig, WranglerConfig};

// pub fn run_wrangler(config: ProjectConfig) {
//     //let run_command = config.build.command;

//     let (main_command, args) = fragment_command(config.wrangler_config.build.command);
//     match Command::new(main_command)
//         .args(args)
//         .current_dir(PathBuf::from(config.path))
//         .output() {
//         Ok(output) => {
//             let stdout = String::from_utf8_lossy(&output.stdout);
//             let stderr = String::from_utf8_lossy(&output.stderr);

//             println!("Stdout: {}", stdout);
//             println!("Stderr: {}", stderr);
//         },
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             return;
//         }
//      }
// }

pub fn run_wrangler(config: ProjectConfig) {
    // let mut chmod_command = Command::new("chmod");
    // let perm_path = format!("{}/{}", config.path.clone(), "node_modules/.bin/react-scripts");
    // chmod_command.arg("+x").arg(PathBuf::from(perm_path));

    // match chmod_command.status() {
    //     Ok(status) if status.success() => {
    //         println!("Successfully set execute permissions for react-scripts.");
    //     },
    //     Ok(status) => {
    //         eprintln!("Failed to set execute permissions, exit status: {}", status);
    //         return;
    //     },
    //     Err(e) => {
    //         eprintln!("Failed to run chmod: {}", e);
    //         return;
    //     }
    // }


    let (main_command, args) = fragment_command(config.wrangler_config.build.command);
    println!("Project dir: {}", config.path);

    let mut command = Command::new("sh");
    command.arg("-c");
    command.arg(format!("{} {}", main_command, args.join(" ")));
    command.current_dir(PathBuf::from(config.path));  // Set the working directory

    match command.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            println!("Stdout: {}", stdout);
            println!("Stderr: {}", stderr);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    }
}

fn fragment_command(config: String) -> (String, Vec<String>) {
    let commands = config.trim().to_string();
    let commands_vec: Vec<&str> = commands.split_whitespace().collect();

    let main_command = commands_vec[0].to_string();
    let command_args = commands_vec[1..].iter().map(|&s| s.to_string()).collect();

    (main_command, command_args)
}