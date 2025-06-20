use std::{collections::HashMap, time::Duration};

use tokio::time::sleep;
use user_notify::{NotificationCategory, NotificationCategoryAction, get_notification_manager};

const DEFAULT_BUNDLE_ID: &str = "ai.gety";

const ACTION_CATEGORY_ID: &str = "app.category.action";
const TEXT_INPUT_CATEGORY_ID: &str = "app.category.textinput";

fn init_logger() {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace)
        .is_test(true)
        .try_init();
}

fn get_test_bundle_id() -> String {
    std::env::var("TEST_BUNDLE_ID").unwrap_or_else(|_| DEFAULT_BUNDLE_ID.to_string())
}

fn create_test_categories() -> Vec<NotificationCategory> {
    vec![
        NotificationCategory {
            identifier: ACTION_CATEGORY_ID.to_string(),
            actions: vec![
                NotificationCategoryAction::Action {
                    identifier: format!("{}.button.submit", ACTION_CATEGORY_ID),
                    title: "Submit".to_string(),
                },
                NotificationCategoryAction::Action {
                    identifier: format!("{}.button.cancel", ACTION_CATEGORY_ID),
                    title: "Cancel".to_string(),
                },
                NotificationCategoryAction::Action {
                    identifier: format!("{}.button.detail", ACTION_CATEGORY_ID),
                    title: "Detail".to_string(),
                },
            ],
        },
        NotificationCategory {
            identifier: TEXT_INPUT_CATEGORY_ID.to_string(),
            actions: vec![NotificationCategoryAction::TextInputAction {
                identifier: format!("{}.button.send", TEXT_INPUT_CATEGORY_ID),
                title: "Reply".to_string(),
                input_button_title: "Send".to_string(),
                input_placeholder: "type your message here".to_string(),
            }],
        },
    ]
}

#[tokio::test]
async fn test_notification_manager_creation() -> anyhow::Result<()> {
    init_logger();
    log::debug!("Testing notification manager creation");

    let bundle_id = get_test_bundle_id();
    let manager = get_notification_manager(bundle_id.clone(), None);

    log::info!(
        "✅ Notification manager created successfully with bundle ID: {}",
        bundle_id
    );
    Ok(())
}

#[tokio::test]
async fn test_category_registration() -> anyhow::Result<()> {
    init_logger();
    log::debug!("Testing notification category registration");

    let bundle_id = get_test_bundle_id();
    let manager = get_notification_manager(bundle_id, None);
    let categories = create_test_categories();

    log::debug!("Registering {} notification categories", categories.len());

    manager.register(
        Box::new(|response| {
            log::info!("📳 Received notification response in test: {response:?}");
        }),
        categories,
    )?;

    log::info!("✅ Notification categories registered successfully");
    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_permission_request() -> anyhow::Result<()> {
    init_logger();
    log::debug!("Testing notification permission request (macOS only)");

    let bundle_id = get_test_bundle_id();
    let manager = get_notification_manager(bundle_id, None);
    let categories = create_test_categories();

    // Register categories first
    manager.register(
        Box::new(|response| {
            log::info!("📳 Received notification response: {response:?}");
        }),
        categories,
    )?;

    log::debug!("Requesting notification permission...");
    let result = manager.first_time_ask_for_notification_permission().await;

    match result {
        Ok(_) => log::info!("✅ Notification permission request completed successfully"),
        Err(err) => log::warn!("⚠️ Permission request failed: {err:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn test_basic_notification_send() -> anyhow::Result<()> {
    init_logger();
    log::debug!("Testing basic notification sending");

    let bundle_id = get_test_bundle_id();
    let manager = get_notification_manager(bundle_id, None);
    let categories = create_test_categories();

    // Register categories first
    manager.register(
        Box::new(|response| {
            log::info!("📳 Received notification response: {response:?}");
        }),
        categories,
    )?;

    let notification = user_notify::NotificationBuilder::new()
        .title("Test Basic Notification")
        .body("This is a basic test notification")
        .set_thread_id("test-thread-basic")
        .set_category_id(ACTION_CATEGORY_ID);

    log::debug!("Sending basic notification...");
    manager.send_notification(notification).await?;
    log::info!("✅ Basic notification sent successfully");

    Ok(())
}

#[tokio::test]
async fn test_notification_with_user_info() -> anyhow::Result<()> {
    init_logger();
    log::debug!("Testing notification with user info");

    let bundle_id = get_test_bundle_id();
    let manager = get_notification_manager(bundle_id, None);
    let categories = create_test_categories();

    // Register categories first
    manager.register(
        Box::new(|response| {
            log::info!("📳 Received notification response: {response:?}");
        }),
        categories,
    )?;

    let mut user_info = HashMap::new();
    user_info.insert("test_key".to_owned(), "test_value".to_owned());
    user_info.insert("notification_type".to_owned(), "user_info_test".to_owned());

    let notification = user_notify::NotificationBuilder::new()
        .title("Test Notification with User Info")
        .body("This notification contains user info")
        .set_thread_id("test-thread-userinfo")
        .set_user_info(user_info.clone())
        .set_category_id(TEXT_INPUT_CATEGORY_ID);

    log::debug!("Sending notification with user info: {:?}", user_info);
    manager.send_notification(notification).await?;
    log::info!("✅ Notification with user info sent successfully");

    Ok(())
}

#[tokio::test]
async fn test_get_active_notifications() -> anyhow::Result<()> {
    init_logger();
    log::debug!("Testing getting active notifications");

    let bundle_id = get_test_bundle_id();
    let manager = get_notification_manager(bundle_id, None);
    let categories = create_test_categories();

    // Register categories first
    manager.register(
        Box::new(|response| {
            log::info!("📳 Received notification response: {response:?}");
        }),
        categories,
    )?;

    // Send a test notification first
    let mut user_info = HashMap::new();
    user_info.insert("active_test".to_owned(), "true".to_owned());

    let notification = user_notify::NotificationBuilder::new()
        .title("Active Notification Test")
        .body("This notification is for testing active notifications")
        .set_thread_id("test-thread-active")
        .set_user_info(user_info)
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(notification).await?;

    // Wait a bit for the notification to be processed
    sleep(Duration::from_secs(1)).await;

    log::debug!("Getting list of active notifications...");
    let active = manager.get_active_notifications().await?;
    log::info!("📋 Retrieved {} active notifications", active.len());

    for (i, handle) in active.iter().enumerate() {
        log::debug!(
            "Active notification {}: user info = {:?}",
            i + 1,
            handle.get_user_info()
        );
    }

    log::info!("✅ Successfully retrieved active notifications");
    Ok(())
}

#[tokio::test]
async fn test_notification_verification() -> anyhow::Result<()> {
    init_logger();
    log::debug!("Testing notification verification");

    let bundle_id = get_test_bundle_id();
    let manager = get_notification_manager(bundle_id, None);
    let categories = create_test_categories();

    // Register categories first
    manager.register(
        Box::new(|response| {
            log::info!("📳 Received notification response: {response:?}");
        }),
        categories,
    )?;

    // Send a notification with specific user info for verification
    let mut user_info = HashMap::new();
    user_info.insert(
        "verification_key".to_owned(),
        "verification_value".to_owned(),
    );
    user_info.insert("test_id".to_owned(), "verification_test".to_owned());

    let notification = user_notify::NotificationBuilder::new()
        .title("Verification Test Notification")
        .body("This notification is for verification testing")
        .set_thread_id("test-thread-verify")
        .set_user_info(user_info.clone())
        .set_category_id(TEXT_INPUT_CATEGORY_ID);

    manager.send_notification(notification).await?;

    // Wait for the notification to be processed
    sleep(Duration::from_secs(2)).await;

    // Get active notifications and verify
    let active = manager.get_active_notifications().await?;
    log::info!(
        "📋 Retrieved {} active notifications for verification",
        active.len()
    );

    let found_notification = active
        .iter()
        .find(|handle| handle.get_user_info().contains_key("verification_key"));

    match found_notification {
        Some(handle) => {
            log::info!(
                "✅ Found notification with expected user info: {:?}",
                handle.get_user_info()
            );
            assert_eq!(
                handle.get_user_info().get("verification_key"),
                Some(&"verification_value".to_string())
            );
        }
        None => {
            log::error!("❌ No notification containing 'verification_key' found!");
            for (i, handle) in active.iter().enumerate() {
                log::error!(
                    "Notification {} user info: {:?}",
                    i + 1,
                    handle.get_user_info()
                );
            }
            panic!("Should exist notification containing 'verification_key'");
        }
    }

    log::info!("✅ Notification verification completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_integration_full_flow() -> anyhow::Result<()> {
    init_logger();
    log::debug!("Testing full integration flow");

    let bundle_id = get_test_bundle_id();
    let manager = get_notification_manager(bundle_id.clone(), None);
    let categories = create_test_categories();

    log::info!("Using Bundle ID: {}", bundle_id);

    // Step 1: Register categories
    log::debug!("Step 1: Registering notification categories...");
    manager.register(
        Box::new(|response| {
            log::info!("📳 Received notification response: {response:?}");
        }),
        categories,
    )?;
    log::info!("✅ Notification categories registered successfully");

    // Step 2: Request permission (macOS only)
    #[cfg(target_os = "macos")]
    {
        log::debug!("Step 2: Requesting notification permission...");
        if let Err(err) = manager.first_time_ask_for_notification_permission().await {
            log::warn!("⚠️ Permission request failed: {err:?}");
        } else {
            log::info!("✅ Notification permission request completed");
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        log::info!("💡 Non-macOS platform, skipping permission request");
    }

    // Step 3: Send first notification
    log::debug!("Step 3: Sending first notification...");
    let notification1 = user_notify::NotificationBuilder::new()
        .title("Integration Test - First")
        .body("First notification in integration test")
        .set_thread_id("integration-thread-1")
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(notification1).await?;
    log::info!("✅ First notification sent successfully");

    // Step 4: Send second notification with user info
    log::debug!("Step 4: Sending second notification with user info...");
    let mut user_info = HashMap::new();
    user_info.insert("integration_test".to_owned(), "full_flow".to_owned());
    user_info.insert("step".to_owned(), "4".to_owned());

    let notification2 = user_notify::NotificationBuilder::new()
        .title("Integration Test - Second")
        .body("Second notification with user info")
        .set_thread_id("integration-thread-2")
        .set_user_info(user_info.clone())
        .set_category_id(TEXT_INPUT_CATEGORY_ID);

    manager.send_notification(notification2).await?;
    log::info!("✅ Second notification sent successfully");

    // Step 5: Wait and verify
    log::debug!("Step 5: Waiting for notifications to be processed...");
    sleep(Duration::from_secs(2)).await;

    let active = manager.get_active_notifications().await?;
    log::info!("📋 Retrieved {} active notifications", active.len());

    let found_notification = active
        .iter()
        .find(|handle| handle.get_user_info().contains_key("integration_test"));

    assert!(
        found_notification.is_some(),
        "Should exist notification containing 'integration_test' key"
    );

    log::info!("✅ Integration test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_long_text_notification() -> anyhow::Result<()> {
    init_logger();
    log::debug!("Testing long text notification");

    let bundle_id = get_test_bundle_id();
    let manager = get_notification_manager(bundle_id, None);
    let long_text_notification = user_notify::NotificationBuilder::new()
        .title("📄 Long Text Test - This is a very long title that might get truncated or wrapped depending on the system notification display limits")
        .body("这是一个超长文本测试通知。This is a very long text notification test to see how the notification system handles extremely long content. We want to test if the text gets truncated, wrapped, or displayed in some other way. The notification system should handle this gracefully without breaking or causing issues. 这个通知包含了中英文混合的超长文本内容，用来测试通知系统对于长文本的处理能力。We're testing various scenarios: very long titles, very long body text, mixed languages (Chinese and English), special characters, emoji 🎉🔥💯, and other edge cases that might occur in real-world usage. This helps ensure our notification library is robust and can handle different types of content gracefully.")
        .subtitle("📏 Subtitle: Testing how subtitles work with extremely long notification content and whether they get proper formatting")
        .sound("default")
        .set_thread_id("test-thread-long-text")
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(long_text_notification).await?;
    log::info!("✅ Long text notification sent successfully");

    Ok(())
}
