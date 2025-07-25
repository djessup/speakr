// ============================================================================
//! Backend Status Service
// ============================================================================

use crate::services::types::ServiceComponent;
use speakr_types::{AppError, BackendStatus, ServiceStatus, StatusUpdate};
use std::sync::{Arc, LazyLock, Mutex};
use tauri::{AppHandle, Emitter};

/// Service responsible for tracking backend component status
pub struct BackendStatusService {
    status: Arc<Mutex<BackendStatus>>,
}

impl BackendStatusService {
    /// Creates a new backend status service with all services starting
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(BackendStatus::new_starting())),
        }
    }

    /// Gets the current status snapshot
    pub fn get_current_status(&self) -> BackendStatus {
        match self.status.lock() {
            Ok(status) => status.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        }
    }

    /// Updates the status of a specific service component
    pub fn update_service_status(&mut self, component: ServiceComponent, status: ServiceStatus) {
        let current_status = match self.status.lock() {
            Ok(status) => status,
            Err(poisoned) => poisoned.into_inner(),
        };

        let mut current_status = current_status;
        // Update timestamp when any service changes (WASM-compatible)
        current_status.timestamp = chrono::Utc::now().timestamp_millis() as u64;

        // Update the specific service
        match component {
            ServiceComponent::AudioCapture => {
                current_status.audio_capture = status;
            }
            ServiceComponent::Transcription => {
                current_status.transcription = status;
            }
            ServiceComponent::TextInjection => {
                current_status.text_injection = status;
            }
        }
    }

    /// Emits status change event to frontend
    pub fn emit_status_change(&self, app_handle: &AppHandle) -> Result<(), String> {
        let status = self.get_current_status();
        app_handle
            .emit("speakr-status-changed", &status)
            .map_err(|e| format!("Failed to emit status change: {e}"))
    }

    /// Emits heartbeat event to frontend
    pub fn emit_heartbeat(&self, app_handle: &AppHandle) -> Result<(), String> {
        let status = self.get_current_status();
        app_handle
            .emit("speakr-heartbeat", &status)
            .map_err(|e| format!("Failed to emit heartbeat: {e}"))
    }
}

impl Default for BackendStatusService {
    fn default() -> Self {
        Self::new()
    }
}

// Global backend status service instance
static GLOBAL_BACKEND_SERVICE: LazyLock<Arc<Mutex<BackendStatusService>>> =
    LazyLock::new(|| Arc::new(Mutex::new(BackendStatusService::new())));

/// Gets a reference to the global backend service instance.
///
/// # Internal API
/// This function is only intended for internal use and testing.
pub async fn get_global_backend_service() -> Arc<Mutex<BackendStatusService>> {
    Arc::clone(&GLOBAL_BACKEND_SERVICE)
}

/// Internal implementation for getting backend status
pub async fn get_backend_status_internal() -> Result<StatusUpdate, AppError> {
    let service = get_global_backend_service().await;
    let service_guard = match service.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    Ok(service_guard.get_current_status())
}

/// Updates the status of a specific service component in the global service.
///
/// # Arguments
///
/// * `component` - The service component to update
/// * `status` - The new status for the component
///
/// # Examples
///
/// ```no_run
/// # use speakr_lib::services::{ServiceComponent, update_global_service_status};
/// # use speakr_types::ServiceStatus;
/// #
/// # #[tokio::main]
/// # async fn main() {
/// // Update audio capture service to ready
/// update_global_service_status(ServiceComponent::AudioCapture, ServiceStatus::Ready).await;
/// # }
/// ```
pub async fn update_global_service_status(component: ServiceComponent, status: ServiceStatus) {
    let service = Arc::clone(&GLOBAL_BACKEND_SERVICE);
    let mut service_guard = match service.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    service_guard.update_service_status(component, status);
}

/// Internal implementation for updating service status
pub async fn update_service_status_internal(
    component: ServiceComponent,
    status: ServiceStatus,
) -> Result<(), AppError> {
    update_global_service_status(component, status).await;
    Ok(())
}

/// Resets the global backend service to initial state for testing.
///
/// # Internal API
/// This function is only intended for internal use and testing.
#[cfg(any(test, debug_assertions))]
pub async fn reset_global_backend_service() {
    let service = Arc::clone(&GLOBAL_BACKEND_SERVICE);
    {
        let mut service_guard = match service.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        *service_guard = BackendStatusService::new();
    } // Drop the mutex guard here before await

    // Add a small delay to ensure the reset is fully processed
    // before other operations access the global state
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
}
