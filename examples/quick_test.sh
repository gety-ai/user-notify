#!/bin/bash

# Quick notification test script for macOS
# This script automates the full testing process

set -e

echo "🧪 Quick Notification Test for macOS"
echo "===================================="
echo

# Step 1: Build with unified bundle ID
echo "📦 Step 1: Building examples with unified bundle ID..."
./build_and_sign.sh --unified
echo "✅ Build completed"
echo

# Step 2: Restart notification center
echo "🔄 Step 2: Restarting Notification Center..."
echo "💡 This helps macOS recognize the new app bundle"
sudo killall NotificationCenter 2>/dev/null || echo "NotificationCenter was not running"
sleep 2
echo "✅ Notification Center restarted"
echo

# Step 3: Open System Preferences
echo "⚙️ Step 3: Opening System Preferences..."
echo "💡 Please check Notifications settings for 'User Notify Test'"
echo "   - Enable 'Allow Notifications'"
echo "   - Set style to 'Alerts' or 'Banners'"  
echo "   - Enable 'Sounds'"
echo "   - Enable 'Show in Notification Center'"
echo
echo "🖱️ Opening System Preferences in 3 seconds..."
sleep 3
open "x-apple.systempreferences:com.apple.preference.notifications"
echo

# Step 4: Wait for user to configure settings
echo "⏱️ Step 4: Waiting for you to configure notification settings..."
echo "Please:"
echo "1. Find 'User Notify Test' or 'ai.gety.user-notify.examples' in the left panel"
echo "2. Enable all notification options (Alerts, Sounds, etc.)"
echo "3. Close System Preferences when done"
echo
read -p "Press Enter when you've finished configuring notifications..."
echo

# Step 5: Test permission request
echo "🔐 Step 5: Testing permission request..."
echo "This will request notification permission if not already granted"
open test_permission_request/target/release/test_permission_request.app
echo "💡 If permission dialog appears, click 'Allow'"
echo
read -p "Press Enter after handling the permission dialog..."
echo

# Step 6: Test basic notification
echo "🔔 Step 6: Testing basic notification with sound..."
echo "This should show notifications in the top-right corner with sound"
open test_basic_notification/target/release/test_basic_notification.app
echo "💡 Watch for:"
echo "   - Notification banner in top-right corner"
echo "   - Notification sound"
echo "   - Entry in Notification Center"
echo

# Step 7: Final verification
echo "✨ Step 7: Final verification"
echo "If notifications still don't work, check:"
echo "1. System volume is not muted"
echo "2. Do Not Disturb is disabled"
echo "3. Focus mode is not active"
echo "4. App appears in System Preferences > Notifications"
echo "5. Console.app for any error messages"
echo
echo "🎉 Test completed! Check if notifications are now working properly." 