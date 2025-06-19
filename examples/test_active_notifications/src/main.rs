use std::collections::HashMap;
use user_notify::{NotificationCategory, NotificationCategoryAction, get_notification_manager};
use tokio::time::{sleep, Duration};

const DEFAULT_BUNDLE_ID: &str = "ai.gety.test.active";
const ACTION_CATEGORY_ID: &str = "app.category.action";

fn init_logger() {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace)
        .is_test(false)
        .init();
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
    ]
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    println!("🚀 Starting active notifications test...");

    let bundle_id = get_test_bundle_id();
    println!("📱 Using Bundle ID: {}", bundle_id);
    
    let manager = get_notification_manager(bundle_id, None);
    let categories = create_test_categories();

    // Register categories first
    println!("📝 Registering notification categories...");
    manager.register(
        Box::new(|response| {
            println!("📳 Received notification response: {response:?}");
        }),
        categories,
    )?;

    // Send first notification
    println!("📤 Sending first notification...");
    let mut user_info1 = HashMap::new();
    user_info1.insert("notification_id".to_owned(), "first".to_owned());
    user_info1.insert("test_type".to_owned(), "active_test".to_owned());

    let notification1 = user_notify::NotificationBuilder::new()
        .title("Active Test - First Notification")
        .body("This is the first notification for active testing")
        .set_thread_id("test-thread-active-1")
        .set_user_info(user_info1)
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(notification1).await?;
    println!("✅ First notification sent");

    // Wait a bit
    sleep(Duration::from_secs(2)).await;

    // Send second notification
    println!("📤 Sending second notification...");
    let mut user_info2 = HashMap::new();
    user_info2.insert("notification_id".to_owned(), "second".to_owned());
    user_info2.insert("test_type".to_owned(), "active_test".to_owned());

    let notification2 = user_notify::NotificationBuilder::new()
        .title("Active Test - Second Notification")
        .body("This is the second notification for active testing")
        .set_thread_id("test-thread-active-2")
        .set_user_info(user_info2)
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(notification2).await?;
    println!("✅ Second notification sent");

    // Wait for notifications to be processed
    println!("⏱️ Waiting for notifications to be processed...");
    sleep(Duration::from_secs(3)).await;

    // Get active notifications
    println!("📋 Getting list of active notifications...");
    let active = manager.get_active_notifications().await?;
    println!("📊 Found {} active notifications", active.len());

    for (i, handle) in active.iter().enumerate() {
        println!("🔍 Notification {}: user info = {:?}", i + 1, handle.get_user_info());
        
        if let Some(notification_id) = handle.get_user_info().get("notification_id") {
            println!("   📌 Notification ID: {}", notification_id);
        }
        
        if let Some(test_type) = handle.get_user_info().get("test_type") {
            println!("   🏷️ Test Type: {}", test_type);
        }
    }

    // Verify we can find our test notifications
    let test_notifications: Vec<_> = active
        .iter()
        .filter(|handle| handle.get_user_info().get("test_type") == Some(&"active_test".to_string()))
        .collect();

    println!("✅ Found {} test notifications out of {} total active notifications", 
             test_notifications.len(), active.len());

    if test_notifications.is_empty() {
        println!("⚠️ No test notifications found in active list. They may have been dismissed or expired.");
    } else {
        println!("🎯 Successfully verified active notification management!");
    }

    println!("💡 You can check your system notification center to see the active notifications");
    println!("🎉 Active notifications test completed!");
    Ok(())
} 