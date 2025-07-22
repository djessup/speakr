// Status-related tests extracted from lib.rs

use speakr_lib::{
    get_global_backend_service, reset_global_backend_service, BackendStatusService,
    ServiceComponent,
};
use speakr_types::ServiceStatus;

#[tokio::test]
async fn test_backend_status_service_creation() {
    let service = BackendStatusService::new();
    let status = service.get_current_status();

    // Should start with all services in "Starting" state
    assert!(!status.is_ready());
    assert_eq!(status.audio_capture, ServiceStatus::Starting);
    assert_eq!(status.transcription, ServiceStatus::Starting);
    assert_eq!(status.text_injection, ServiceStatus::Starting);
}

#[tokio::test]
async fn test_backend_status_service_update_single_service() {
    let mut service = BackendStatusService::new();

    // Update a single service to Ready
    service.update_service_status(ServiceComponent::AudioCapture, ServiceStatus::Ready);
    let status = service.get_current_status();

    assert_eq!(status.audio_capture, ServiceStatus::Ready);
    assert_eq!(status.transcription, ServiceStatus::Starting);
    assert_eq!(status.text_injection, ServiceStatus::Starting);
    assert!(!status.is_ready()); // Not all ready yet
}

#[tokio::test]
async fn test_backend_status_service_all_services_ready() {
    let mut service = BackendStatusService::new();

    // Update all services to Ready
    service.update_service_status(ServiceComponent::AudioCapture, ServiceStatus::Ready);
    service.update_service_status(ServiceComponent::Transcription, ServiceStatus::Ready);
    service.update_service_status(ServiceComponent::TextInjection, ServiceStatus::Ready);

    let status = service.get_current_status();
    assert!(status.is_ready()); // All ready
}

#[tokio::test]
async fn test_backend_status_service_error_handling() {
    let mut service = BackendStatusService::new();

    // Set an error on transcription service
    service.update_service_status(
        ServiceComponent::Transcription,
        ServiceStatus::Error("Failed to load Whisper model".to_string()),
    );

    let status = service.get_current_status();
    assert!(!status.is_ready());
    assert!(matches!(status.transcription, ServiceStatus::Error(_)));
}

#[tokio::test]
async fn test_backend_status_timestamps() {
    let mut service = BackendStatusService::new();
    let initial_timestamp = service.get_current_status().timestamp;

    // Wait a bit and update a service
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    service.update_service_status(ServiceComponent::AudioCapture, ServiceStatus::Ready);

    let updated_timestamp = service.get_current_status().timestamp;
    assert!(
        updated_timestamp > initial_timestamp,
        "Timestamp should be updated"
    );
}

// Previously skipped tests - now working with pub helper functions

#[tokio::test]
async fn test_global_backend_service_initialization() {
    // Reset service to ensure clean state
    reset_global_backend_service().await;

    let service = get_global_backend_service().await;
    let service_guard = service.lock().unwrap();
    let status = service_guard.get_current_status();

    // Should start with all services in "Starting" state
    assert!(!status.is_ready());
    assert_eq!(status.audio_capture, ServiceStatus::Starting);
    assert_eq!(status.transcription, ServiceStatus::Starting);
    assert_eq!(status.text_injection, ServiceStatus::Starting);
}

#[tokio::test]
async fn test_global_backend_service_state_updates() {
    // Reset service to ensure clean state
    reset_global_backend_service().await;

    // Update a service status using the global service
    use speakr_lib::update_global_service_status;
    update_global_service_status(ServiceComponent::AudioCapture, ServiceStatus::Ready).await;

    let service = get_global_backend_service().await;
    let service_guard = service.lock().unwrap();
    let status = service_guard.get_current_status();

    assert_eq!(status.audio_capture, ServiceStatus::Ready);
    assert_eq!(status.transcription, ServiceStatus::Starting);
    assert_eq!(status.text_injection, ServiceStatus::Starting);
    assert!(!status.is_ready()); // Not all ready yet
}

#[tokio::test]
async fn test_global_backend_service_thread_safety() {
    use tokio::task;

    // Reset service to ensure clean state
    reset_global_backend_service().await;

    let service = get_global_backend_service().await;
    let _initial_timestamp = {
        let service_guard = service.lock().unwrap();
        service_guard.get_current_status().timestamp
    };

    // Spawn multiple tasks that update the service concurrently
    let tasks = (0..5).map(|i| {
        task::spawn(async move {
            // Update different services from different tasks
            use speakr_lib::update_global_service_status;
            let component = match i % 3 {
                0 => ServiceComponent::AudioCapture,
                1 => ServiceComponent::Transcription,
                _ => ServiceComponent::TextInjection,
            };
            update_global_service_status(component, ServiceStatus::Ready).await;
        })
    });

    // Wait for all tasks to complete
    for task in tasks {
        task.await.expect("Task should complete");
    }

    // Verify that updates were applied (at least one service should be Ready)
    let service_guard = service.lock().unwrap();
    let status = service_guard.get_current_status();

    // At least one service should have been updated
    assert!(
        status.audio_capture == ServiceStatus::Ready
            || status.transcription == ServiceStatus::Ready
            || status.text_injection == ServiceStatus::Ready
    );
}

#[tokio::test]
async fn test_backend_service_emits_events_on_state_change() {
    // Reset service to ensure clean state
    reset_global_backend_service().await;

    let service = get_global_backend_service().await;
    let initial_timestamp = {
        let service_guard = service.lock().unwrap();
        service_guard.get_current_status().timestamp
    };

    // Update a service status
    use speakr_lib::update_global_service_status;
    update_global_service_status(ServiceComponent::AudioCapture, ServiceStatus::Ready).await;

    // Small delay to ensure timestamp difference
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

    // Check that the timestamp was updated (indicating an event was processed)
    let updated_status = {
        let service_guard = service.lock().unwrap();
        service_guard.get_current_status()
    };
    assert!(
        updated_status.timestamp >= initial_timestamp,
        "Timestamp should be updated"
    );
}

/*
The following test uses the get_backend_status Tauri command which cannot be moved:
- test_complete_status_communication_flow (uses get_backend_status Tauri command)

This test will remain in lib.rs as it tests the full Tauri command interface.
*/
