use std::collections::HashMap;
use user_notify::{NotificationCategory, NotificationCategoryAction, get_notification_manager};
use tokio::time::{sleep, Duration};

const DEFAULT_BUNDLE_ID: &str = "ai.gety.test.full";
const ACTION_CATEGORY_ID: &str = "app.category.action";
const TEXT_INPUT_CATEGORY_ID: &str = "app.category.textinput";

fn init_logger() {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
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
    println!("🚀 Starting full integration test...");
    println!("🎯 This test will demonstrate all notification features:");
    println!("   • Permission request (macOS only)");
    println!("   • Category registration");
    println!("   • Basic notifications");
    println!("   • Interactive notifications");
    println!("   • Active notification management");
    println!();

    let bundle_id = get_test_bundle_id();
    println!("📱 Using Bundle ID: {}", bundle_id);
    
    let manager = get_notification_manager(bundle_id.clone(), None);
    let categories = create_test_categories();

    // Step 1: Register categories
    println!("📝 Step 1: Registering notification categories...");
    manager.register(
        Box::new(|response| {
            println!("📳 Received notification response: {response:?}");
        }),
        categories,
    )?;
    println!("✅ Categories registered successfully");
    println!();

    // Step 2: Request permission (macOS only)
    #[cfg(target_os = "macos")]
    {
        println!("🔐 Step 2: Requesting notification permission...");
        match manager.first_time_ask_for_notification_permission().await {
            Ok(_) => println!("✅ Permission request completed successfully"),
            Err(err) => {
                println!("⚠️ Permission request failed: {err:?}");
                println!("💡 You may need to grant permission manually in System Preferences");
            }
        }
        println!();
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("💡 Step 2: Non-macOS platform, skipping permission request");
        println!();
    }

    // Step 3: Send basic notification
    println!("📤 Step 3: Sending basic notification...");
    let notification1 = user_notify::NotificationBuilder::new()
        .title("Integration Test - Basic")
        .body("This is a basic notification with action buttons")
        .set_thread_id("integration-thread-basic")
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(notification1).await?;
    println!("✅ Basic notification sent");
    sleep(Duration::from_secs(2)).await;
    println!();

    // Step 4: Send notification with user info
    println!("📤 Step 4: Sending notification with user info...");
    let mut user_info = HashMap::new();
    user_info.insert("integration_test".to_owned(), "full_flow".to_owned());
    user_info.insert("step".to_owned(), "4".to_owned());
    user_info.insert("timestamp".to_owned(), format!("{}", chrono::Utc::now().timestamp()));

    let notification2 = user_notify::NotificationBuilder::new()
        .title("Integration Test - Interactive")
        .body("This notification has text input - try replying to it!")
        .set_thread_id("integration-thread-interactive")
        .set_user_info(user_info.clone())
        .set_category_id(TEXT_INPUT_CATEGORY_ID);

    manager.send_notification(notification2).await?;
    println!("✅ Interactive notification sent");
    sleep(Duration::from_secs(3)).await;
    println!();

    // Step 5: Check active notifications
    println!("📋 Step 5: Checking active notifications...");
    let active = manager.get_active_notifications().await?;
    println!("📊 Found {} active notifications", active.len());

    for (i, handle) in active.iter().enumerate() {
        let user_info = handle.get_user_info();
        println!("🔍 Notification {}: user info = {:?}", i + 1, user_info);
        
        if user_info.contains_key("integration_test") {
            println!("   🎯 This is our test notification!");
        }
    }

    let test_notifications: Vec<_> = active
        .iter()
        .filter(|handle| handle.get_user_info().contains_key("integration_test"))
        .collect();

    if !test_notifications.is_empty() {
        println!("✅ Successfully found {} test notifications in active list", test_notifications.len());
    } else {
        println!("⚠️ No test notifications found in active list - they may have been dismissed");
    }
    println!();

    // Step 6: Send a final notification
    println!("📤 Step 6: Sending completion notification...");
    let notification3 = user_notify::NotificationBuilder::new()
        .title("Integration Test Complete! 🎉")
        .body("All notification features have been tested successfully. Check your notification center!")
        .set_thread_id("integration-thread-complete")
        .set_category_id(ACTION_CATEGORY_ID);

    manager.send_notification(notification3).await?;
    println!("✅ Completion notification sent");
    println!();

    println!("🎊 Full integration test completed successfully!");
    println!("💡 Check your system notification center to see all the notifications");
    println!("🔔 Try interacting with the notifications to test the response handling");
    
    // Keep the program running for a bit to handle any responses
    println!("⏱️ Keeping program alive for 30 seconds to handle notification responses...");
    sleep(Duration::from_secs(30)).await;
    
    println!("👋 Test program finishing. Thank you!");
    Ok(())
} 