// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{App, AppHandle, Manager};

pub mod editor;
pub mod lsp;
pub mod git;
pub mod fs;
pub mod ai;
pub mod config;
pub mod utils;

// Initialize logging
fn setup_logging() {
    env_logger::init();
    log::info!("TurkCode IDE started");
}

// Register all Tauri commands
fn register_commands(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    app.register_handler(tauri::generate_handler![
        // File system operations
        fs::commands::read_file,
        fs::commands::write_file,
        fs::commands::list_files,
        
        // Editor operations
        editor::commands::get_syntax_highlighting,
        
        // LSP operations
        lsp::commands::initialize_language_server,
        lsp::commands::get_completions,
        lsp::commands::get_diagnostics,
        
        // Git operations
        git::commands::get_status,
        git::commands::get_all_branches,
        git::commands::get_recent_commits,
        git::commands::checkout_branch,
        git::commands::create_branch,
        git::commands::commit,
        git::commands::pull,
        git::commands::push,
        git::commands::stage_file,
        git::commands::unstage_file,
        
        // AI operations
        ai::commands::get_code_completion,
        ai::commands::analyze_code,
        
        // Config operations
        config::commands::get_settings,
        config::commands::update_settings
    ]);
    
    Ok(())
}

// Register all services
fn register_services(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize LSP manager
    let lsp_manager = lsp::LanguageServerManager::new();
    app.manage(lsp_manager);
    
    // Initialize editor document manager
    let doc_manager = editor::DocumentManager::new();
    app.manage(doc_manager);
    
    // Initialize AI service
    let ai_service = ai::AIService::new();
    app.manage(ai_service);
    
    Ok(())
}

// Initialize database
async fn init_database(app_handle: &AppHandle) -> Result<sqlx::SqlitePool, Box<dyn std::error::Error>> {
    let app_dir = app_handle.path().app_data_dir()?;
    
    // Create app data directory if it doesn't exist
    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir)?;
    }
    
    let db_path = app_dir.join("turkcode.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    
    // Create SQLite connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await?;
    
    // Run migrations if needed
    // sqlx::migrate!().run(&pool).await?;
    
    Ok(pool)
}

// Initialize config manager
fn init_config_manager(app_handle: &AppHandle) -> Result<config::ConfigManager, Box<dyn std::error::Error>> {
    let config_manager = config::ConfigManager::new(app_handle)
        .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;
    
    Ok(config_manager)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logging();
    
    tauri::Builder::default()
        .setup(|app| {
            // Initialize modules on app startup
            let app_handle = app.handle();
            
            // Register commands
            register_commands(app)?;
            
            // Register services
            register_services(app)?;
            
            // Initialize config manager
            let config_manager = init_config_manager(&app_handle)?;
            app.manage(config_manager);
            
            // Spawn a new task to initialize database
            tauri::async_runtime::spawn(async move {
                match init_database(&app_handle).await {
                    Ok(pool) => {
                        // Store pool in app state for later use
                        app_handle.manage(pool);
                        log::info!("Database initialized successfully");
                    },
                    Err(err) => {
                        log::error!("Failed to initialize database: {}", err);
                    }
                }
            });
            
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
