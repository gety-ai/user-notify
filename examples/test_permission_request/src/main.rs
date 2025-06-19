use std::collections::HashMap;
use user_notify::{NotificationCategory, NotificationCategoryAction, get_notification_manager};

const DEFAULT_BUNDLE_ID: &str = "ai.gety.test.permission";
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
    ]
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    println!("🚀 Starting permission request test...");

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

    // Request permission
    #[cfg(target_os = "macos")]
    {
        println!("🔐 Requesting notification permission...");
        match manager.first_time_ask_for_notification_permission().await {
            Ok(_) => println!("✅ Permission request completed successfully"),
            Err(err) => {
                println!("❌ Permission request failed: {err:?}");
                return Err(err.into());
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("💡 Non-macOS platform, permission request not required");
    }

    println!("🎉 Permission request test completed!");
    Ok(())
} 