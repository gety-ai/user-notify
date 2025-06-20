#!/bin/bash

# Check macOS Notification Settings Script
# This script helps diagnose common notification issues on macOS

set -e

echo "🔍 macOS Notification Settings Diagnostic"
echo "==========================================="
echo

# Check Do Not Disturb status
echo "📵 Checking Do Not Disturb status..."
DND_STATUS=$(defaults read ~/Library/Preferences/ByHost/com.apple.notificationcenterui.plist doNotDisturb 2>/dev/null || echo "unknown")
if [ "$DND_STATUS" = "1" ]; then
    echo "⚠️  Do Not Disturb is ENABLED - this will prevent notifications from showing"
    echo "💡 To disable: Click the Control Center icon in menu bar and turn off Focus/Do Not Disturb"
elif [ "$DND_STATUS" = "0" ]; then
    echo "✅ Do Not Disturb is disabled"
else
    echo "❓ Could not determine Do Not Disturb status"
fi
echo

# Check Focus mode status (macOS 12+)
echo "🎯 Checking Focus mode status..."
FOCUS_STATUS=$(shortcuts run "Get Current Focus" 2>/dev/null || echo "unknown")
if [ "$FOCUS_STATUS" != "unknown" ] && [ "$FOCUS_STATUS" != "" ]; then
    echo "⚠️  Focus mode is active: $FOCUS_STATUS"
    echo "💡 Focus modes can block notifications. Check Control Center to disable."
else
    echo "✅ No active Focus mode detected"
fi
echo

# Check notification center settings
echo "🔔 Checking general notification settings..."
NOTIFICATION_CENTER=$(defaults read com.apple.ncprefs.plist 2>/dev/null || echo "unknown")
if [ "$NOTIFICATION_CENTER" != "unknown" ]; then
    echo "✅ Notification Center preferences accessible"
else
    echo "❓ Could not read Notification Center preferences"
fi
echo

# Check system notification permissions
echo "🔐 Checking system notification permissions..."
echo "💡 App-specific notification permissions must be checked manually:"
echo "   1. Open System Preferences/Settings"
echo "   2. Go to Notifications & Focus (or just Notifications)"
echo "   3. Look for 'User Notify Test' or your app's bundle ID"
echo "   4. Ensure notifications are enabled with alerts/banners"
echo

# System information
echo "💻 System Information:"
echo "   macOS Version: $(sw_vers -productVersion)"
echo "   Build: $(sw_vers -buildVersion)"
echo

# Check if running in Terminal vs app bundle
echo "🏃 Execution Context:"
if [ -n "$TERM" ]; then
    echo "   Running in Terminal - notifications may not work properly"
    echo "   💡 Use app bundles (.app) for proper notification support"
else
    echo "   Running as app bundle - proper notification context"
fi
echo

# Troubleshooting tips
echo "🛠️  Troubleshooting Tips:"
echo "   1. Ensure app is built as .app bundle with Info.plist"
echo "   2. Check System Preferences > Notifications for your app"
echo "   3. Try restarting Notification Center:"
echo "      sudo killall NotificationCenter"
echo "   4. Check Console.app for notification-related errors"
echo "   5. Ensure Do Not Disturb / Focus modes are disabled"
echo "   6. Make sure sound is not muted"
echo

echo "✨ Run this after checking settings:"
echo "   ./build_and_sign.sh --unified"
echo "   open test_basic_notification/target/release/test_basic_notification.app"
echo 