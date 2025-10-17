//! Robert Server - Remote execution server for Robert desktop application
//!
//! This binary starts the Warp web server with configured routes and middleware.

use robert_server::{
    api::{delete_session_handler, execute_handler, get_session_handler, health_handler},
    auth::{with_auth, AuthState},
    claude::{ClaudeExecutor, Executor, MockClaudeExecutor},
    Config,
};
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;
use warp::Filter;

mod api {
    pub use robert_server::api::health::HealthState;
}

use api::HealthState;
use robert_server::session::SessionManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut dev_mode = false;
    let mut mock_mode = false;
    let mut config_path = None;

    for i in 0..args.len() {
        match args[i].as_str() {
            "--dev" => dev_mode = true,
            "--mock" => mock_mode = true,
            "--config" => {
                if i + 1 < args.len() {
                    config_path = Some(PathBuf::from(&args[i + 1]));
                }
            }
            _ => {}
        }
    }

    // Load configuration
    let config = if let Some(path) = config_path {
        Config::from_file(path)?
    } else if dev_mode {
        Config::dev_default()
    } else {
        // Default to looking for config.toml in current directory
        Config::from_file("config.toml")?
    };

    // Override mock mode if specified on command line
    let use_mock = mock_mode || config.claude.mock_mode;

    // Initialize logging
    init_logging(&config);

    info!("Starting Robert Server v{}", env!("CARGO_PKG_VERSION"));
    info!("Configuration:");
    info!("  Host: {}", config.server.host);
    info!("  Port: {}", config.server.port);
    info!("  Dev mode: {}", config.server.dev_mode);
    info!("  TLS enabled: {}", config.server.enable_tls);
    info!("  Auth required: {}", config.auth.require_auth);
    info!("  Mock mode: {}", use_mock);

    // Create shared state
    let config = Arc::new(config);
    let session_manager = Arc::new(SessionManager::new(1000)); // Keep 1000 completed sessions
    let auth_state = Arc::new(AuthState::new(
        config.valid_tokens(),
        config.auth.require_auth,
        config.auth.rate_limit_per_minute,
    ));
    let health_state = Arc::new(HealthState::new(config.claude.binary_path.clone()));

    // Create executor (mock or real)
    let executor: Arc<dyn Executor> = if use_mock {
        info!("Using mock executor");
        Arc::new(MockClaudeExecutor::new())
    } else {
        info!(
            "Using real Claude CLI executor: {}",
            config.claude.binary_path
        );
        Arc::new(ClaudeExecutor::new(
            config.claude.binary_path.clone(),
            config.claude.default_timeout_seconds,
        ))
    };

    // Build routes
    let routes = build_routes(
        config.clone(),
        executor,
        session_manager,
        auth_state,
        health_state,
    );

    // Add middleware
    let routes = routes.with(warp::trace::request());

    let cors = if config.server.dev_mode {
        warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
    } else {
        // Restrictive CORS for production (configure as needed)
        warp::cors()
            .allow_origin("https://yourdomain.com")
            .allow_methods(vec!["GET", "POST", "DELETE"])
    };

    let routes = routes.with(cors);

    // Parse bind address
    let addr: SocketAddr = config.bind_address().parse()?;

    info!("Server listening on {}", addr);

    // Start server
    if config.server.enable_tls {
        // TLS mode (production)
        info!("Starting server with TLS");
        // TODO: Implement TLS support
        return Err("TLS support not yet implemented".into());
    } else {
        // HTTP mode (development)
        info!("Starting server in HTTP mode (no TLS)");
        warp::serve(routes).run(addr).await;
    }

    Ok(())
}

/// Builds all API routes
fn build_routes(
    config: Arc<Config>,
    executor: Arc<dyn Executor>,
    session_manager: Arc<SessionManager>,
    auth_state: Arc<AuthState>,
    health_state: Arc<HealthState>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Health endpoint (no auth required)
    let health = warp::path!("api" / "v1" / "health")
        .and(warp::get())
        .and(with_health_state(health_state))
        .and_then(health_handler);

    // Execute endpoint (with auth)
    let execute = warp::path!("api" / "v1" / "execute")
        .and(warp::post())
        .and(with_auth(auth_state.clone()))
        .and(warp::body::json())
        .and(with_executor(executor))
        .and(with_session_manager(session_manager.clone()))
        .and(with_config(config.clone()))
        .and_then(
            |_token: String, request, executor, session_manager, config| {
                execute_handler(request, executor, session_manager, config)
            },
        );

    // Get session endpoint (with auth)
    let get_session = warp::path!("api" / "v1" / "sessions" / Uuid)
        .and(warp::get())
        .and(with_auth(auth_state.clone()))
        .and(with_session_manager(session_manager.clone()))
        .and_then(|session_id: Uuid, _token: String, manager| {
            get_session_handler(session_id, manager)
        });

    // Delete session endpoint (with auth)
    let delete_session = warp::path!("api" / "v1" / "sessions" / Uuid)
        .and(warp::delete())
        .and(with_auth(auth_state))
        .and(with_session_manager(session_manager))
        .and_then(|session_id: Uuid, _token: String, manager| {
            delete_session_handler(session_id, manager)
        });

    health.or(execute).or(get_session).or(delete_session)
}

/// Warp filter to inject executor
fn with_executor(
    executor: Arc<dyn Executor>,
) -> impl Filter<Extract = (Arc<dyn Executor>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || executor.clone())
}

/// Warp filter to inject session manager
fn with_session_manager(
    manager: Arc<SessionManager>,
) -> impl Filter<Extract = (Arc<SessionManager>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

/// Warp filter to inject config
fn with_config(
    config: Arc<Config>,
) -> impl Filter<Extract = (Arc<Config>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}

/// Warp filter to inject health state
fn with_health_state(
    state: Arc<HealthState>,
) -> impl Filter<Extract = (Arc<HealthState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

/// Initializes logging based on configuration
fn init_logging(config: &Config) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.logging.level));

    if config.logging.pretty_print {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .pretty()
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .json()
            .init();
    }
}
