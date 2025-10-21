use dioxus::prelude::*;

#[component]
pub fn ConfigButton() -> Element {
    let mut status_message = use_signal(|| "".to_string());

    let open_config_file = move |_| {
        spawn(async move {
            match crate::path::server_config_file() {
                Some(config_path) => {
                    // Open the file with the default system editor
                    #[cfg(target_os = "windows")]
                    {
                        match std::process::Command::new("cmd")
                            .args(&["/C", "start", "", &config_path.to_string_lossy()])
                            .spawn()
                        {
                            Ok(_) => {
                                status_message.set("Configuration file opened".to_string());
                            }
                            Err(e) => {
                                status_message.set(format!("Error opening config file: {}", e));
                            }
                        }
                    }

                    #[cfg(target_os = "macos")]
                    {
                        match std::process::Command::new("open").arg(&config_path).spawn() {
                            Ok(_) => {
                                status_message.set("Configuration file opened".to_string());
                            }
                            Err(e) => {
                                status_message.set(format!("Error opening config file: {}", e));
                            }
                        }
                    }

                    #[cfg(target_os = "linux")]
                    {
                        match std::process::Command::new("xdg-open")
                            .arg(&config_path)
                            .spawn()
                        {
                            Ok(_) => {
                                status_message.set("Configuration file opened".to_string());
                            }
                            Err(e) => {
                                status_message.set(format!("Error opening config file: {}", e));
                            }
                        }
                    }
                }
                None => {
                    status_message.set("Could not determine configuration file path".to_string());
                }
            }

            // Clear status message after 3 seconds
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            status_message.set("".to_string());
        });
    };

    rsx! {
        div {
            class: "glass-card text-center",

            h3 {
                class: "component-title text-lg mb-4",
                "Configuration"
            }

            button {
                class: "btn btn-primary w-full mb-4",
                onclick: open_config_file,
                "📝 Open Config File"
            }

            if !status_message().is_empty() {
                div {
                    class: {
                        if status_message().contains("Error") {
                            "status-message error"
                        } else {
                            "status-message success"
                        }
                    },
                    "{status_message}"
                }
            }
        }
    }
}
