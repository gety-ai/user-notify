use std::collections::HashMap;
use user_notify::{NotificationCategory, NotificationCategoryAction, get_notification_manager};
use tokio::time::{sleep, Duration};

const DEFAULT_BUNDLE_ID: &str = "ai.gety.test.interactive";
const TEXT_INPUT_CATEGORY_ID: &str = "app.category.textinput";
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
            ],
        },
        NotificationCategory {
            identifier: TEXT_INPUT_CATEGORY_ID.to_string(),
            actions: vec![NotificationCategoryAction::TextInputAction {
                identifier: format!("{}.button.send", TEXT_INPUT_CATEGORY_ID),
                title: "Reply".to_string(),
                input_button_title: "Send".to_string(),
                input_placeholder: "Type your message here...".to_string(),
            }],
        },
    ]
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    println!("🚀 Starting interactive notification test...");

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

    // Send notification with action buttons
    println!("📤 Sending notification with action buttons...");
    let notification1 = user_notify::NotificationBuilder::new()
        .title("Interactive Test - Action Buttons")
        .body("This notification has action buttons. Try clicking them!")
        .set_thread_id("test-thread-actions")
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(notification1).await?;
    println!("✅ Action button notification sent");

    // Wait a bit
    sleep(Duration::from_secs(3)).await;

    // Send notification with text input
    println!("📤 Sending notification with text input...");
    let mut user_info = HashMap::new();
    user_info.insert("test_type".to_owned(), "text_input".to_owned());
    user_info.insert("timestamp".to_owned(), chrono::Utc::now().to_rfc3339());

    let notification2 = user_notify::NotificationBuilder::new()
        .title("Interactive Test - Text Input")
        .body("This notification allows text input. Try replying to it!")
        .set_thread_id("test-thread-textinput")
        .set_user_info(user_info)
        .set_category_id(TEXT_INPUT_CATEGORY_ID);

    manager.send_notification(notification2).await?;
    println!("✅ Text input notification sent");

    // Wait longer to allow interaction
    println!("⏱️ Waiting 30 seconds for user interaction...");
    println!("💡 Try interacting with the notifications in the system notification center!");
    sleep(Duration::from_secs(30)).await;

    println!("🎉 Interactive notification test completed!");
    Ok(())
} 