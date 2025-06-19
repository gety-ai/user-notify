# User-Notify Test Examples

This directory contains standalone test examples that can be individually compiled and signed for testing the `user-notify` library, especially on macOS where code signing is required.

## Available Test Examples

### 1. `test_permission_request` - Permission Request Test
Tests the notification permission request functionality (macOS only).

**Features tested:**
- Notification manager creation
- Category registration
- Permission request flow

**Usage:**
```bash
cd examples/test_permission_request
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
cd examples/test_basic_notification
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
cd examples/test_interactive_notification
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
cd examples/test_active_notifications
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
cd examples/test_full_integration
cargo run
```

## Building and Running

### Prerequisites
- Rust toolchain
- On macOS: Code signing certificate (for production deployment)

### Building Individual Examples
Each example can be built independently:

```bash
cd examples/<example_name>
cargo build --release
```

### Running Examples
```bash
cd examples/<example_name>
cargo run
```

### Environment Variables
All examples support the following environment variables:

- `TEST_BUNDLE_ID`: Override the default bundle ID for testing
- `RUST_LOG`: Set logging level (e.g., `RUST_LOG=debug`)

Example:
```bash
TEST_BUNDLE_ID="com.yourcompany.testapp" RUST_LOG=debug cargo run
```

## macOS Code Signing

For macOS deployment, you'll need to sign the binaries:

### 1. Build the example
```bash
cd examples/<example_name>
cargo build --release
```

### 2. Sign the binary
```bash
codesign --force --sign "Your Developer ID" target/release/<example_name>
```

### 3. Verify the signature
```bash
codesign --verify --verbose target/release/<example_name>
```

### 4. Run the signed binary
```bash
./target/release/<example_name>
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

1. **Permission Request**: `test_permission_request`
   - Establishes notification permissions

2. **Basic Functionality**: `test_basic_notification`
   - Verifies basic notification sending

3. **Interactive Features**: `test_interactive_notification`
   - Tests user interaction capabilities

4. **Management Features**: `test_active_notifications`
   - Tests notification management

5. **Full Integration**: `test_full_integration`
   - Comprehensive end-to-end test

## Troubleshooting

### macOS Permission Issues
If notifications don't appear:
1. Check System Preferences → Notifications
2. Ensure the app bundle ID is allowed
3. Try running with `sudo` for testing (not recommended for production)

### Code Signing Issues
If you get code signing errors:
1. Ensure you have a valid Developer ID certificate
2. Check certificate validity: `security find-identity -v -p codesigning`
3. Use the exact certificate name from the output

### Bundle ID Conflicts
If you get bundle ID conflicts:
1. Use unique bundle IDs for each test
2. Set `TEST_BUNDLE_ID` environment variable
3. Clear notification settings for conflicting bundle IDs

## Development Notes

These examples are designed to:
- Be independent and self-contained
- Demonstrate specific features clearly
- Provide verbose output for debugging
- Handle errors gracefully
- Support both development and production scenarios

Each example includes comprehensive logging and error handling to help with debugging and verification of the notification system functionality. 