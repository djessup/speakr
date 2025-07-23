/// TUI version of the model metadata extraction tool
///
/// This provides a more interactive and visually appealing interface for updating
/// model metadata compared to the CLI version. Uses structured logging with tracing
/// and minimal emoji usage, preferring styled text and colors.
use std::fs;
use std::path::PathBuf;

use color_eyre::eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Terminal,
};
use size::Size;
use tempfile::TempDir;
use tracing::{debug, error, info, instrument, warn};

use speakr_core::model::{ModelListUpdater, ModelMetadata};

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_step: Step,
    pub progress: f64,
    pub status_message: String,
    pub models_processed: Vec<ModelMetadata>,
    pub error_message: Option<String>,
    pub should_quit: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    Initializing,
    CloningRepository,
    ExtractingMetadata,
    GeneratingCode,
    Complete,
    Error,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_step: Step::Initializing,
            progress: 0.0,
            status_message: "Initializing model update process".to_string(),
            models_processed: Vec::new(),
            error_message: None,
            should_quit: false,
        }
    }
}

pub struct TuiApp {
    state: AppState,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl TuiApp {
    /// Creates a new TUI application instance
    #[instrument]
    pub fn new() -> Result<Self> {
        info!("Initializing TUI application");

        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            state: AppState::default(),
            terminal,
        })
    }

    /// Updates the application state
    #[instrument(skip(self))]
    pub fn update_state(&mut self, new_state: AppState) {
        debug!("Updating app state: {:?}", new_state.current_step);
        self.state = new_state;
    }

    /// Renders the current application state
    #[instrument(skip(self))]
    pub fn render(&mut self) -> Result<()> {
        let state = self.state.clone(); // Clone state to avoid borrowing issues

        self.terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // Title
                        Constraint::Length(3), // Progress bar
                        Constraint::Min(0),    // Main content
                        Constraint::Length(3), // Status bar
                    ]
                    .as_ref(),
                )
                .split(size);

            // Title
            let title = Paragraph::new("Speakr Model Updater")
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Progress bar
            let progress_block = Block::default().borders(Borders::ALL).title("Progress");
            let progress = Gauge::default()
                .block(progress_block)
                .gauge_style(Style::default().fg(Color::Green))
                .percent((state.progress * 100.0) as u16);
            f.render_widget(progress, chunks[1]);

            // Main content based on current step
            Self::render_main_content_static(&state, f, chunks[2]);

            // Status bar
            let status_style = if state.error_message.is_some() {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Green)
            };

            let status_text = state
                .error_message
                .as_ref()
                .unwrap_or(&state.status_message);

            let status = Paragraph::new(status_text.as_str())
                .style(status_style)
                .block(Block::default().borders(Borders::ALL).title("Status"));
            f.render_widget(status, chunks[3]);
        })?;
        Ok(())
    }

    fn render_main_content_static(state: &AppState, f: &mut ratatui::Frame, area: Rect) {
        match state.current_step {
            Step::Initializing => {
                let text = Paragraph::new("Preparing to update model metadata...").block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Initialization"),
                );
                f.render_widget(text, area);
            }
            Step::CloningRepository => {
                let text = Paragraph::new(
                    "Cloning whisper.cpp repository\nThis may take a few moments...",
                )
                .block(Block::default().borders(Borders::ALL).title("Repository"));
                f.render_widget(text, area);
            }
            Step::ExtractingMetadata => {
                let text = Paragraph::new("Extracting model metadata from LFS pointer files")
                    .block(Block::default().borders(Borders::ALL).title("Processing"));
                f.render_widget(text, area);
            }
            Step::GeneratingCode => {
                let items: Vec<ListItem> = state
                    .models_processed
                    .iter()
                    .enumerate()
                    .map(|(i, model)| {
                        let size = Size::from_bytes(model.size_bytes);
                        let formatted_size = if size.bytes() >= 1_000_000_000 {
                            format!(
                                "{:.1} GiB",
                                (size.bytes() as f64) / (1024.0 * 1024.0 * 1024.0)
                            )
                        } else {
                            format!("{} MiB", size.bytes() / (1024 * 1024))
                        };

                        let model_name = model
                            .filename
                            .trim_start_matches("ggml-")
                            .trim_end_matches(".bin");

                        ListItem::new(Line::from(vec![
                            Span::styled(
                                format!("{:2}. ", i + 1),
                                Style::default().fg(Color::Blue),
                            ),
                            Span::styled(model_name, Style::default().fg(Color::White)),
                            Span::styled(
                                format!(" ({formatted_size})"),
                                Style::default().fg(Color::Gray),
                            ),
                        ]))
                    })
                    .collect();

                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Models Found"));
                f.render_widget(list, area);
            }
            Step::Complete => {
                let success_text = format!(
                    "Model update completed successfully!\n\nProcessed {} models\nGenerated code ready for integration",
                    state.models_processed.len()
                );
                let text = Paragraph::new(success_text)
                    .style(Style::default().fg(Color::Green))
                    .block(Block::default().borders(Borders::ALL).title("Complete"));
                f.render_widget(text, area);
            }
            Step::Error => {
                let unknown_error = "Unknown error occurred".to_string();
                let error_text = state.error_message.as_ref().unwrap_or(&unknown_error);
                let text = Paragraph::new(error_text.as_str())
                    .style(Style::default().fg(Color::Red))
                    .block(Block::default().borders(Borders::ALL).title("Error"));
                f.render_widget(text, area);
            }
        }
    }

    /// Handles user input events
    #[instrument(skip(self))]
    pub fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        info!("User requested quit");
                        self.state.should_quit = true;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    /// Runs the main application loop
    #[instrument(skip(self))]
    pub async fn run(
        &mut self,
        repo_name: &str,
        workspace_dir: Option<PathBuf>,
        output_file: &PathBuf,
    ) -> Result<()> {
        info!("Starting TUI application main loop");

        // Initial render
        self.render()?;

        // Step 1: Initialize
        self.state.current_step = Step::CloningRepository;
        self.state.progress = 0.1;
        self.state.status_message = format!("Cloning repository {repo_name}");
        self.render()?;

        // Clone repository
        let result = if let Some(workspace_path) = workspace_dir {
            let updater = ModelListUpdater::with_repo(workspace_path.clone(), repo_name);
            self.process_models(&updater, output_file).await
        } else {
            let temp_dir = TempDir::new()?;
            let updater = ModelListUpdater::with_repo(temp_dir.path().to_path_buf(), repo_name);
            self.process_models(&updater, output_file).await
        };

        match result {
            Ok(metadata_count) => {
                self.state.current_step = Step::Complete;
                self.state.progress = 1.0;
                self.state.status_message =
                    format!("Successfully processed {metadata_count} models");
                info!(
                    "Model update completed successfully with {} models",
                    metadata_count
                );
            }
            Err(e) => {
                self.state.current_step = Step::Error;
                self.state.error_message = Some(format!("Update failed: {e}"));
                error!("Model update failed: {}", e);
            }
        }

        // Final render and wait for user to quit
        self.render()?;
        while !self.state.should_quit {
            self.handle_events()?;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(())
    }

    async fn process_models(
        &mut self,
        updater: &ModelListUpdater,
        output_file: &PathBuf,
    ) -> Result<usize> {
        // Clone repository
        updater.clone_repository().await?;

        self.state.current_step = Step::ExtractingMetadata;
        self.state.progress = 0.4;
        self.state.status_message = "Extracting metadata from LFS pointer files".to_string();
        self.render()?;

        // Extract metadata
        let metadata = updater.extract_all_metadata().await?;

        self.state.current_step = Step::GeneratingCode;
        self.state.progress = 0.7;
        self.state.models_processed = metadata.clone();
        self.state.status_message = format!("Found {} models, generating code", metadata.len());
        self.render()?;

        // Generate and write code
        let updated_code = generate_models_code(&metadata);
        fs::write(output_file, updated_code)?;

        self.state.progress = 1.0;
        Ok(metadata.len())
    }
}

impl Drop for TuiApp {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
    }
}

// Re-use the code generation functions from the original implementation
fn generate_models_code(metadata: &[ModelMetadata]) -> String {
    let mut code = String::new();

    // Add the header documentation with model table
    code.push_str(&generate_header_docs(metadata));
    code.push_str("\n\n");

    // Add imports
    code.push_str("use size::Size;\n\n");

    // Add the enum definition
    code.push_str(&generate_model_enum(metadata));
    code.push_str("\n\n");

    // Add implementation block
    code.push_str(&generate_impl_block(metadata));
    code.push_str("\n\n");

    code
}

fn generate_header_docs(metadata: &[ModelMetadata]) -> String {
    let mut docs = String::new();
    docs.push_str(
        "/// OpenAI's Whisper models converted to ggml format for use with whisper.cpp\n",
    );
    docs.push_str("///\n");

    let repo_link = if let Some(first_meta) = metadata.first() {
        if let Some(repo_start) = first_meta.download_url.find("huggingface.co/") {
            let after_hf = &first_meta.download_url[repo_start + 15..];
            if let Some(resolve_pos) = after_hf.find("/resolve/") {
                let repo_name = &after_hf[..resolve_pos];
                format!("https://huggingface.co/{repo_name}/tree/main")
            } else {
                "https://huggingface.co/ggerganov/whisper.cpp/tree/main".to_string()
            }
        } else {
            "https://huggingface.co/ggerganov/whisper.cpp/tree/main".to_string()
        }
    } else {
        "https://huggingface.co/ggerganov/whisper.cpp/tree/main".to_string()
    };

    docs.push_str(&format!("/// [Available models]({repo_link})\n"));
    docs.push_str("///\n");
    docs.push_str(
        "/// | Model               | Disk    | SHA                                        |\n",
    );
    docs.push_str(
        "/// | ------------------- | ------- | ------------------------------------------ |\n",
    );

    for meta in metadata {
        let model_name = meta
            .filename
            .trim_start_matches("ggml-")
            .trim_end_matches(".bin");
        let size = Size::from_bytes(meta.size_bytes);
        let formatted_size = if size.bytes() >= 1_000_000_000 {
            format!(
                "{:.1} GiB",
                (size.bytes() as f64) / (1024.0 * 1024.0 * 1024.0)
            )
        } else {
            format!("{} MiB", size.bytes() / (1024 * 1024))
        };

        docs.push_str(&format!(
            "/// | {:<19} | {:<7} | `{}` |\n",
            model_name, formatted_size, meta.sha256
        ));
    }

    docs.push_str("///\n");
    docs.push_str(&format!(
        "/// Example: {}\n",
        metadata
            .first()
            .map(|m| m.download_url.as_str())
            .unwrap_or("")
    ));

    docs
}

fn generate_model_enum(metadata: &[ModelMetadata]) -> String {
    let mut enum_code = String::new();
    enum_code.push_str("#[allow(dead_code)]\n");
    enum_code.push_str("#[derive(Debug)]\n");
    enum_code.push_str("pub enum Model {\n");

    for meta in metadata {
        let variant_name = filename_to_variant_name(&meta.filename);
        enum_code.push_str(&format!("    {variant_name},\n"));
    }

    enum_code.push_str("}\n");
    enum_code
}

fn generate_impl_block(metadata: &[ModelMetadata]) -> String {
    let mut impl_code = String::new();
    impl_code.push_str("impl Model {\n");

    // Generate filename method
    impl_code.push_str("    pub fn filename(&self) -> &'static str {\n");
    impl_code.push_str("        match self {\n");
    for meta in metadata {
        let variant_name = filename_to_variant_name(&meta.filename);
        let base_name = meta
            .filename
            .trim_start_matches("ggml-")
            .trim_end_matches(".bin");
        impl_code.push_str(&format!(
            "            Model::{variant_name} => \"{base_name}\",\n"
        ));
    }
    impl_code.push_str("        }\n");
    impl_code.push_str("    }\n\n");

    // Generate filesize method
    impl_code.push_str("    pub fn filesize(&self) -> Size {\n");
    impl_code.push_str("        match self {\n");
    for meta in metadata {
        let variant_name = filename_to_variant_name(&meta.filename);
        let size = Size::from_bytes(meta.size_bytes);
        let size_expr = if size.bytes() >= 1_000_000_000 {
            format!(
                "Size::from_gib({:.1})",
                (size.bytes() as f64) / (1024.0 * 1024.0 * 1024.0)
            )
        } else {
            format!("Size::from_mib({})", size.bytes() / (1024 * 1024))
        };
        impl_code.push_str(&format!(
            "            Model::{variant_name} => {size_expr},\n"
        ));
    }
    impl_code.push_str("        }\n");
    impl_code.push_str("    }\n\n");

    // Generate sha method
    impl_code.push_str("    pub fn sha(&self) -> &'static str {\n");
    impl_code.push_str("        match self {\n");
    for meta in metadata {
        let variant_name = filename_to_variant_name(&meta.filename);
        impl_code.push_str(&format!(
            "            Model::{} => \"{}\",\n",
            variant_name, meta.sha256
        ));
    }
    impl_code.push_str("        }\n");
    impl_code.push_str("    }\n\n");

    // Generate url method
    impl_code.push_str("    pub fn url(&self) -> String {\n");
    impl_code.push_str("        format!(\n");

    let (repo_name, git_ref) = if let Some(first_meta) = metadata.first() {
        let repo = if let Some(repo_start) = first_meta.download_url.find("huggingface.co/") {
            let after_hf = &first_meta.download_url[repo_start + 15..];
            if let Some(resolve_pos) = after_hf.find("/resolve/") {
                after_hf[..resolve_pos].to_string()
            } else {
                "ggerganov/whisper.cpp".to_string()
            }
        } else {
            "ggerganov/whisper.cpp".to_string()
        };
        (repo, first_meta.git_ref.clone())
    } else {
        ("ggerganov/whisper.cpp".to_string(), "main".to_string())
    };

    impl_code.push_str(&format!(
        "            \"https://huggingface.co/{repo_name}/resolve/{git_ref}/ggml-{{}}.bin\",\n"
    ));
    impl_code.push_str("            self.filename()\n");
    impl_code.push_str("        )\n");
    impl_code.push_str("    }\n");

    impl_code.push_str("}\n");
    impl_code
}

fn filename_to_variant_name(filename: &str) -> String {
    let base = filename
        .trim_start_matches("ggml-")
        .trim_end_matches(".bin");

    let mut result = String::new();
    let mut capitalize_next = true;

    for c in base.chars() {
        match c {
            '-' | '_' | '.' => {
                capitalize_next = true;
            }
            c if c.is_numeric() => {
                result.push(c);
                capitalize_next = false;
            }
            c if capitalize_next => {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            }
            c => {
                result.push(c);
                capitalize_next = false;
            }
        }
    }

    // Handle quantized model patterns after camel case conversion
    result = result.replace("Q51", "QuantizedQ5_1");
    result = result.replace("Q80", "QuantizedQ8_0");
    result = result.replace("Q50", "QuantizedQ5_0");

    result
}

/// Initialize logging system following the pattern from ratatui documentation
pub fn initialize_logging() -> Result<()> {
    use directories::ProjectDirs;
    use tracing_error::ErrorLayer;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

    let project_name = env!("CARGO_CRATE_NAME").to_uppercase();
    let log_env = format!("{project_name}_LOGLEVEL");
    let log_file = format!("{}.log", env!("CARGO_PKG_NAME"));

    let directory =
        if let Some(proj_dirs) = ProjectDirs::from("com", "speakr", env!("CARGO_PKG_NAME")) {
            proj_dirs.data_local_dir().to_path_buf()
        } else {
            PathBuf::from(".").join(".data")
        };

    std::fs::create_dir_all(&directory)?;
    let log_path = directory.join(log_file);
    let log_file_handle = std::fs::File::create(log_path)?;

    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG")
            .or_else(|_| std::env::var(log_env))
            .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))),
    );

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file_handle)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}

fn get_arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == flag)
        .and_then(|pos| args.get(pos + 1))
        .cloned()
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging first
    initialize_logging()?;

    // Initialize color-eyre for better error handling
    color_eyre::install()?;

    info!("Starting TUI model updater");

    let args: Vec<String> = std::env::args().collect();

    let repo_name =
        get_arg_value(&args, "--repo").unwrap_or_else(|| "ggerganov/whisper.cpp".to_string());

    let workspace_dir = get_arg_value(&args, "--workspace-dir").map(PathBuf::from);

    let output_file = get_arg_value(&args, "--output")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("updated_list.rs"));

    info!(
        "Configuration: repo={}, output={}",
        repo_name,
        output_file.display()
    );

    let mut app = TuiApp::new()?;
    app.run(&repo_name, workspace_dir, &output_file).await?;

    info!("TUI model updater completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        // RED: Test that AppState has correct default values
        let state = AppState::default();

        assert_eq!(state.current_step, Step::Initializing);
        assert_eq!(state.progress, 0.0);
        assert_eq!(state.status_message, "Initializing model update process");
        assert!(state.models_processed.is_empty());
        assert!(state.error_message.is_none());
        assert!(!state.should_quit);
    }

    #[test]
    fn test_filename_to_variant_name_conversion() {
        // RED: Test that filename conversion works correctly
        assert_eq!(filename_to_variant_name("ggml-base.bin"), "Base");
        assert_eq!(filename_to_variant_name("ggml-small.en.bin"), "SmallEn");
        assert_eq!(
            filename_to_variant_name("ggml-medium.q5_1.bin"),
            "MediumQuantizedQ5_1"
        );
    }

    #[tokio::test]
    async fn test_generate_models_code_creates_valid_rust_code() {
        // RED: Test that code generation produces valid structure
        let metadata = vec![ModelMetadata {
            filename: "ggml-base.bin".to_string(),
            size_bytes: 147_965_312,
            sha256: "60ed5bc3dd14eea856493d334349b405782e7c9595e0e7e0f89f5f9c89309b32".to_string(),
            git_ref: "main".to_string(),
            download_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin"
                .to_string(),
        }];

        let code = generate_models_code(&metadata);

        // Should contain expected structure
        assert!(code.contains("use size::Size;"));
        assert!(code.contains("pub enum Model {"));
        assert!(code.contains("Base,"));
        assert!(code.contains("impl Model {"));
        assert!(code.contains("pub fn filename(&self)"));
        assert!(code.contains("pub fn filesize(&self)"));
        assert!(code.contains("pub fn sha(&self)"));
        assert!(code.contains("pub fn url(&self)"));
    }

    #[test]
    fn test_get_arg_value_parses_correctly() {
        // RED: Test argument parsing
        let args = vec![
            "program".to_string(),
            "--repo".to_string(),
            "test/repo".to_string(),
            "--output".to_string(),
            "test_output.rs".to_string(),
        ];

        assert_eq!(
            get_arg_value(&args, "--repo"),
            Some("test/repo".to_string())
        );
        assert_eq!(
            get_arg_value(&args, "--output"),
            Some("test_output.rs".to_string())
        );
        assert_eq!(get_arg_value(&args, "--missing"), None);
    }

    // Integration tests would go here but require more complex setup
    // These test the individual components that can be tested in isolation
}
