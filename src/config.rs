use std;
use once_cell::sync::OnceCell;



#[derive(Debug)]
pub struct AppConfig {
    pub serve_directory: String,
    pub port: u32
}


impl AppConfig {
    const SERVE_DIRECTORY: &str = "./tmp/";
    const PORT: u32 = 4221;
    
    pub fn initialize() -> &'static AppConfig {

        let mut app_config: AppConfig = AppConfig {
            serve_directory: Self::SERVE_DIRECTORY.to_string(),
            port: Self::PORT
        };

        let program_arguments: Vec<String> = std::env::args().collect();
        for (index, program_argument) in program_arguments.iter().enumerate() {
            match program_argument.as_str() {
                "--directory" => {
                    match program_arguments.get(index + 1) {
                        Some(directory_value) => {
                           app_config.serve_directory = directory_value.clone();
                        },
                        None => {
                            panic!("No value provided for --directory flag")
                        }
                    }
                },
                _ => {
                    println!("Unsupported argument: {}", program_argument);
                }
            }
        }
        
        APP_CONFIG_INSTANCE.set(app_config).expect("Could not create AppConfig instance..");

        let instance = APP_CONFIG_INSTANCE.get().expect("AppConfig is not initialized");

        return instance;
    }

    pub fn global() -> &'static AppConfig {
        return APP_CONFIG_INSTANCE.get().expect("AppConfig is not initialized")
    }

}

pub static APP_CONFIG_INSTANCE: OnceCell<AppConfig> = OnceCell::new();
