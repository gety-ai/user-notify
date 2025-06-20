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
    println!("🚀 Starting basic notification test with sound...");

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

    // Check and request permission first
    println!("🔐 Checking notification permission...");
    let has_permission = manager.get_notification_permission_state().await?;
    
    if !has_permission {
        println!("❗ No notification permission. Requesting permission...");
        let granted = manager.first_time_ask_for_notification_permission().await?;
        if granted {
            println!("✅ Notification permission granted!");
        } else {
            println!("❌ Notification permission denied. Please enable notifications in System Preferences.");
            println!("💡 Go to System Preferences > Notifications > User Notify Test");
            return Ok(());
        }
    } else {
        println!("✅ Already have notification permission");
    }

    // Send basic notification with sound
    println!("📤 Sending basic notification with sound...");
    let notification = user_notify::NotificationBuilder::new()
        .title("🔊 Test Basic Notification")
        .body("This notification should have sound and appear in the top-right corner!")
        .sound("default")  // Add default system sound
        .set_thread_id("test-thread-basic")
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(notification).await?;
    println!("✅ Basic notification with sound sent successfully");

    // Wait a moment, then send another notification
    sleep(Duration::from_secs(3)).await;

    println!("📤 Sending second notification...");
    let notification2 = user_notify::NotificationBuilder::new()
        .title("🔔 Second Notification")
        .body("This is the second test notification")
        .subtitle("With subtitle")
        .sound("default")
        .set_thread_id("test-thread-basic-2")
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(notification2).await?;
    println!("✅ Second notification sent successfully");

    // Wait a moment, then send a notification with very long text
    sleep(Duration::from_secs(3)).await;

    println!("📤 Sending notification with very long text...");
    let long_text_notification = user_notify::NotificationBuilder::new()
        .title("📄 Long Text Test - This is a very long title that might get truncated or wrapped depending on the system notification display limits")
        .body("这是一个超长文本测试通知。This is a very long text notification test to see how the notification system handles extremely long content. We want to test if the text gets truncated, wrapped, or displayed in some other way. The notification system should handle this gracefully without breaking or causing issues. 这个通知包含了中英文混合的超长文本内容，用来测试通知系统对于长文本的处理能力。We're testing various scenarios: very long titles, very long body text, mixed languages (Chinese and English), special characters, emoji 🎉🔥💯, and other edge cases that might occur in real-world usage. This helps ensure our notification library is robust and can handle different types of content gracefully.")
        .subtitle("📏 Subtitle: Testing how subtitles work with extremely long notification content and whether they get proper formatting")
        .sound("default")
        .set_thread_id("test-thread-long-text")
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(long_text_notification).await?;
    println!("✅ Long text notification sent successfully");

    // Wait a bit to see the notification
    println!("⏱️ Waiting 10 seconds to observe the notifications...");
    println!("💡 Check your notification center and top-right corner of screen");
    sleep(Duration::from_secs(10)).await;

    println!("🎉 Basic notification test completed!");
    Ok(())
} 