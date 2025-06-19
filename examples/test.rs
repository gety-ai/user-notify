use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};

use tokio::{signal::ctrl_c, spawn, time::sleep};
use user_notify::{NotificationCategory, NotificationCategoryAction, get_notification_manager};

const DEFAULT_BUNDLE_ID: &str = "ai.gety";

const ACTION_CATEGORY_ID: &str = "app.category.action";
const TEXT_INPUT_CATEGORY_ID: &str = "app.category.textinput";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace)
        .init();
    log::debug!("Logger system initialized, starting test flow");

    log::info!("Please enter Bundle ID (e.g. {}): ", DEFAULT_BUNDLE_ID);
    io::stdout().flush()?;

    let mut bundle_id = String::new();
    io::stdin().read_line(&mut bundle_id)?;
    let bundle_id = bundle_id.trim();

    let bundle_id = if bundle_id.is_empty() {
        log::info!(
            "Bundle ID is empty, using default value: {}",
            DEFAULT_BUNDLE_ID
        );
        DEFAULT_BUNDLE_ID.to_string()
    } else {
        bundle_id.to_string()
    };

    log::info!("Using Bundle ID: {}", bundle_id);

    log::debug!("Creating notification manager...");
    let manager = get_notification_manager(bundle_id.clone(), None);
    log::info!(
        "Notification manager created successfully, bundle ID: {}",
        bundle_id
    );

    log::debug!("Preparing to register notification categories...");
    let categories = vec![
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
    ];

    log::debug!(
        "Registering notification categories, total {} categories",
        categories.len()
    );
    for (i, category) in categories.iter().enumerate() {
        log::trace!(
            "Category {}: {} (contains {} actions)",
            i + 1,
            category.identifier,
            category.actions.len()
        );
    }

    manager.register(
        Box::new(|response| {
            log::info!("📳 Received notification response: {response:?}");
        }),
        categories,
    )?;
    log::info!("✅ Notification categories registered successfully");

    log::debug!("Requesting notification permission...");
    #[cfg(target_os = "macos")]
    {
        let manager_clone = manager.clone();
        if let Err(err) = spawn(async move {
            log::debug!("Executing permission request async task");
            let result = manager_clone
                .first_time_ask_for_notification_permission()
                .await;
            log::debug!("Permission request task completed, result: {:?}", result);
            result
        })
        .await
        {
            log::error!("❌ Permission request failed: {err:?}");
            println!("failed to ask for notification permission: {err:?}");
        } else {
            log::info!("✅ Notification permission request completed");
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        log::info!("💡 Non-macOS platform, skipping permission request");
    }

    log::debug!("Preparing to send first notification...");
    let mut notification = user_notify::NotificationBuilder::new();

    notification = notification
        .title("my title")
        .body("my body")
        .set_thread_id(&format!("thread-id"))
        .set_category_id(ACTION_CATEGORY_ID);

    log::debug!(
        "First notification built - title: 'my title', body: 'my body', category: '{}'",
        ACTION_CATEGORY_ID,
    );
    log::debug!("Sending first notification...");
    manager.send_notification(notification).await?;
    log::info!("✅ First notification sent successfully");

    let mut info = HashMap::new();
    info.insert("hey".to_owned(), "hi".to_owned());
    log::debug!("Created user info map: {:?}", info);

    log::debug!("Preparing to send second notification (with user info and category)...");
    let notification = user_notify::NotificationBuilder::new()
        .title("my title")
        .body("my body")
        .set_thread_id(&format!("thread-id"))
        .set_user_info(info.clone())
        .set_category_id(TEXT_INPUT_CATEGORY_ID);

    log::debug!(
        "Second notification built - title: 'my title', body: 'my body', category: '{}'",
        TEXT_INPUT_CATEGORY_ID,
    );
    log::debug!("User info: {:?}", info);
    log::debug!("Sending second notification asynchronously...");

    let manager_clone = manager.clone();
    spawn(async move {
        log::debug!("Sending notification in async task");
        let result = manager_clone.send_notification(notification).await;
        log::debug!("Async notification send result: {:?}", result);
        result
    })
    .await??;
    log::info!("✅ Second notification sent successfully");

    log::debug!("Waiting 2 seconds to ensure notifications are processed by system...");
    sleep(Duration::from_secs(2)).await;
    log::debug!("Wait completed");

    log::debug!("Getting list of active notifications...");
    let active = manager.get_active_notifications().await?;
    log::info!("📋 Retrieved {} active notifications", active.len());

    for (i, handle) in active.iter().enumerate() {
        log::debug!(
            "Notification {}: user info = {:?}",
            i + 1,
            handle.get_user_info()
        );
    }

    log::debug!("Complete active notifications list: {active:?}");

    log::debug!("Verifying existence of notification containing 'hey' key...");
    let found_notification = active
        .iter()
        .find(|handle| handle.get_user_info().contains_key("hey"));

    match found_notification {
        Some(handle) => {
            log::info!(
                "✅ Found notification with expected user info: {:?}",
                handle.get_user_info()
            );
        }
        None => {
            log::error!("❌ No notification containing 'hey' key found!");
            for (i, handle) in active.iter().enumerate() {
                log::error!(
                    "Notification {} user info: {:?}",
                    i + 1,
                    handle.get_user_info()
                );
            }
        }
    }

    assert!(
        found_notification.is_some(),
        "Should exist notification containing 'hey' key"
    );
    log::info!("✅ Assertion validation passed");

    log::info!("Waiting for Ctrl+C signal to exit program...");
    log::info!("💡 Press Ctrl+C to exit the program");
    let _ = ctrl_c().await;
    log::info!("🔚 Exit signal received, program ending");

    Ok(())
}
