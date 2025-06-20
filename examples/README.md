# User-Notify Test Examples

This directory contains standalone test examples that can be individually compiled and packaged as macOS app bundles for testing the `user-notify` library. On macOS, proper app bundles with Info.plist files are required for notifications to work correctly.

## Quick Start

### Build All Examples (Recommended)

Use the provided build script to create signed app bundles for all examples:

```bash
cd examples
./build_and_sign.sh
```

This will:

- Build all examples as release binaries
- Package them as proper macOS app bundles with Info.plist
- Apply Ad-Hoc signatures (no Developer ID required)
- Make them ready to run with full notification support

### Run Examples

After building, run examples using:

```bash
# Method 1: Using open command (recommended)
open examples/<example_name>/target/release/<example_name>.app

# Method 2: Direct execution
examples/<example_name>/target/release/<example_name>.app/Contents/MacOS/<example_name>

# Method 3: Double-click the .app file in Finder
```

## Available Test Examples

### 1. `test_permission_request` - Permission Request Test

Tests the notification permission request functionality (macOS only).

**Features tested:**

- Notification manager creation
- Category registration
- Permission request flow

**Usage:**

```bash
# Build and run as app bundle
./build_and_sign.sh
open test_permission_request/target/release/test_permission_request.app

# Or run directly during development
cd test_permission_request
cargo run
```

### 2. `test_basic_notification` - Basic Notification Test

Tests basic notification sending functionality.

**Features tested:**

- Basic notification creation and sending
- Action buttons
- Notification categories

**Usage:**

```bash
# Build and run as app bundle
./build_and_sign.sh
open test_basic_notification/target/release/test_basic_notification.app

# Or run directly during development
cd test_basic_notification
cargo run
```

### 3. `test_interactive_notification` - Interactive Notification Test

Tests interactive notification features including action buttons and text input.

**Features tested:**

- Action button notifications
- Text input notifications
- User interaction handling
- Custom user info

**Usage:**

```bash
# Build and run as app bundle
./build_and_sign.sh
open test_interactive_notification/target/release/test_interactive_notification.app

# Or run directly during development
cd test_interactive_notification
cargo run
```

### 4. `test_active_notifications` - Active Notification Management Test

Tests retrieving and managing active notifications.

**Features tested:**

- Sending multiple notifications
- Retrieving active notifications
- Notification verification with user info
- Active notification filtering

**Usage:**

```bash
# Build and run as app bundle
./build_and_sign.sh
open test_active_notifications/target/release/test_active_notifications.app

# Or run directly during development
cd test_active_notifications
cargo run
```

### 5. `test_full_integration` - Full Integration Test

A comprehensive test that demonstrates all notification features in sequence.

**Features tested:**

- Complete workflow from permission request to notification interaction
- All notification types
- Active notification management
- Response handling

**Usage:**

```bash
# Build and run as app bundle
./build_and_sign.sh
open test_full_integration/target/release/test_full_integration.app

# Or run directly during development
cd test_full_integration
cargo run
```

## Building and Running

### Prerequisites

- Rust toolchain
- macOS (for app bundle creation and signing)
- Xcode Command Line Tools (for codesign)

### Automated Building (Recommended)

Use the provided build script for the best experience:

```bash
# Build all examples with Ad-Hoc signing (default)
./build_and_sign.sh

# Build without signing
./build_and_sign.sh --no-sign
```

The script will:

1. Build each example in release mode
2. Create proper macOS app bundles with Info.plist
3. Apply Ad-Hoc signatures for local development
4. Verify signatures

### Manual Building

If you prefer to build individual examples:

```bash
cd examples/<example_name>
cargo build --release

# For full notification support, create app bundle manually
mkdir -p target/release/<example_name>.app/Contents/MacOS
mkdir -p target/release/<example_name>.app/Contents/Resources
cp target/release/<example_name> target/release/<example_name>.app/Contents/MacOS/
# Copy and customize Info.plist...
```

### Environment Variables

All examples support the following environment variables:

- `TEST_BUNDLE_ID`: Override the default bundle ID for testing
- `RUST_LOG`: Set logging level (e.g., `RUST_LOG=debug`)

Example:

```bash
TEST_BUNDLE_ID="com.yourcompany.testapp" RUST_LOG=debug cargo run
```

## macOS App Bundle and Signing

### Why App Bundles Are Required

macOS requires proper app bundles with Info.plist files for:

- Notification system integration
- Proper application identification
- Security and permission management
- User notification authorization

### Ad-Hoc Signing (Default)

The build script uses Ad-Hoc signing by default:

- ✅ No Apple Developer account required
- ✅ Works for local development and testing
- ✅ Satisfies macOS security requirements
- ⚠️ Cannot be distributed to other users

### Info.plist Configuration

Each app bundle includes an Info.plist with:

- `CFBundleIdentifier`: Unique bundle ID for each example
- `CFBundleExecutable`: Binary name
- `NSUserNotificationsUsageDescription`: Notification permission description
- Other required macOS bundle metadata

### Developer ID Signing (Optional)

For distribution, you can modify the script to use your Developer ID:

```bash
# Edit build_and_sign.sh and replace Ad-Hoc signing ("-") with your Developer ID
codesign --force --deep --sign "Developer ID Application: Your Name (TEAM_ID)" "$app_path"
```

## Bundle IDs

Each example uses a different default bundle ID to avoid conflicts:

- `test_permission_request`: `ai.gety.test.permission`
- `test_basic_notification`: `ai.gety.test.basic`
- `test_interactive_notification`: `ai.gety.test.interactive`
- `test_active_notifications`: `ai.gety.test.active`
- `test_full_integration`: `ai.gety.test.full`

You can override these using the `TEST_BUNDLE_ID` environment variable.

## Testing Flow

For comprehensive testing, run the examples in this order:

1. **Build All Examples**:

   ```bash
   ./build_and_sign.sh
   ```

2. **Permission Request**:

   ```bash
   open test_permission_request/target/release/test_permission_request.app
   ```

   - Establishes notification permissions

3. **Basic Functionality**:

   ```bash
   open test_basic_notification/target/release/test_basic_notification.app
   ```

   - Verifies basic notification sending

4. **Interactive Features**:

   ```bash
   open test_interactive_notification/target/release/test_interactive_notification.app
   ```

   - Tests user interaction capabilities

5. **Management Features**:

   ```bash
   open test_active_notifications/target/release/test_active_notifications.app
   ```

   - Tests notification management

6. **Full Integration**:
   ```bash
   open test_full_integration/target/release/test_full_integration.app
   ```
   - Comprehensive end-to-end test

## Troubleshooting

### Notifications Don't Appear

If notifications don't show up:

1. **Use App Bundles**: Always run the signed .app bundles, not raw binaries
2. **Check System Preferences**: Go to System Preferences/Settings → Notifications
3. **Verify Bundle ID**: Ensure each example has a unique bundle ID
4. **Check Permissions**: The app should request notification permission on first run
5. **Verify Info.plist**: Ensure the app bundle contains a proper Info.plist file

### Build Script Issues

If the build script fails:

```bash
# Make sure the script is executable
chmod +x build_and_sign.sh

# Check for bash compatibility issues
bash --version  # Should work with macOS default bash

# Run without signing to debug
./build_and_sign.sh --no-sign
```

### Code Signing Issues

If you get code signing errors:

1. **Ad-Hoc Signing**: The default Ad-Hoc signing should work without certificates
2. **Check Xcode Tools**: Ensure Xcode Command Line Tools are installed
3. **Certificate Issues**: For Developer ID signing, check: `security find-identity -v -p codesigning`

### Bundle ID Conflicts

If you get bundle ID conflicts:

1. Use the `TEST_BUNDLE_ID` environment variable
2. Clear notification settings in System Preferences for conflicting bundle IDs
3. Use unique bundle IDs for each test

### Permission Issues

If permission requests fail:

1. Reset notification permissions in System Preferences
2. Try running a fresh build with `./build_and_sign.sh`
3. Ensure the Info.plist contains `NSUserNotificationsUsageDescription`

## Development Notes

### App Bundle Structure

Each built example creates this structure:

```
<example_name>.app/
├── Contents/
│   ├── Info.plist          # Bundle metadata and permissions
│   ├── MacOS/
│   │   └── <example_name>  # The actual binary
│   ├── Resources/          # (Currently empty)
│   └── _CodeSignature/     # Ad-Hoc signature data
```

### Development vs. Production

- **Development**: Use `cargo run` for quick iteration
- **Testing**: Use app bundles for notification testing
- **Production**: Use proper Developer ID signing for distribution

These examples are designed to:

- Be independent and self-contained
- Demonstrate specific features clearly
- Provide verbose output for debugging
- Handle errors gracefully
- Work with macOS notification system requirements
- Support both development and production scenarios

Each example includes comprehensive logging and error handling to help with debugging and verification of the notification system functionality.
