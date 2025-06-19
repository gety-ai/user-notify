use user_notify::{NotificationCategory, NotificationCategoryAction, get_notification_manager};
use tokio::time::{sleep, Duration};

const DEFAULT_BUNDLE_ID: &str = "ai.gety.test.basic";
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
    println!("🚀 Starting basic notification test...");

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

    // Send basic notification
    println!("📤 Sending basic notification...");
    let notification = user_notify::NotificationBuilder::new()
        .title("Test Basic Notification")
        .body("This is a basic test notification from standalone example")
        .set_thread_id("test-thread-basic")
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(notification).await?;
    println!("✅ Basic notification sent successfully");

    // Wait a bit to see the notification
    println!("⏱️ Waiting 5 seconds to observe the notification...");
    sleep(Duration::from_secs(5)).await;

    println!("🎉 Basic notification test completed!");
    Ok(())
} 