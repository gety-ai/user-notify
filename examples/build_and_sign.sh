#!/bin/bash

# Build and Sign Script for User-Notify Test Examples (Ad-Hoc Signing)
# Usage: ./build_and_sign.sh [--no-sign] [--unified-bundle-id BUNDLE_ID]
# By default, binaries will be built, packaged as app bundles, and ad-hoc signed
# Use --no-sign to skip signing entirely
# Use --unified-bundle-id to use the same bundle ID for all examples

set -e

SKIP_SIGNING=false
UNIFIED_BUNDLE_ID=""
DEFAULT_UNIFIED_BUNDLE_ID="ai.gety.user-notify.examples"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-sign)
            SKIP_SIGNING=true
            shift
            ;;
        --unified-bundle-id)
            UNIFIED_BUNDLE_ID="$2"
            shift 2
            ;;
        --unified)
            UNIFIED_BUNDLE_ID="$DEFAULT_UNIFIED_BUNDLE_ID"
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--no-sign] [--unified-bundle-id BUNDLE_ID] [--unified]"
            echo "  --no-sign: Skip code signing"
            echo "  --unified-bundle-id BUNDLE_ID: Use specified bundle ID for all examples"
            echo "  --unified: Use default unified bundle ID ($DEFAULT_UNIFIED_BUNDLE_ID)"
            exit 1
            ;;
    esac
done

EXAMPLES=(
    "test_permission_request"
    "test_basic_notification"
    "test_interactive_notification"
    "test_active_notifications"
    "test_full_integration"
)

# Function to get bundle ID for each example
get_bundle_id() {
    if [ -n "$UNIFIED_BUNDLE_ID" ]; then
        echo "$UNIFIED_BUNDLE_ID"
        return
    fi
    
    case "$1" in
        "test_permission_request")
            echo "ai.gety.test.permission"
            ;;
        "test_basic_notification")
            echo "ai.gety.test.basic"
            ;;
        "test_interactive_notification")
            echo "ai.gety.test.interactive"
            ;;
        "test_active_notifications")
            echo "ai.gety.test.active"
            ;;
        "test_full_integration")
            echo "ai.gety.test.full"
            ;;
        *)
            echo ""
            ;;
    esac
}

echo "🚀 Building and packaging user-notify test examples as macOS app bundles..."
if [ "$SKIP_SIGNING" = true ]; then
    echo "Signing: Disabled (--no-sign flag used)"
else
    echo "Signing: Ad-Hoc (no Developer ID required)"
fi

if [ -n "$UNIFIED_BUNDLE_ID" ]; then
    echo "Bundle ID Mode: Unified ($UNIFIED_BUNDLE_ID)"
    echo "💡 All examples will share the same Bundle ID and notification permissions"
else
    echo "Bundle ID Mode: Individual (each example has its own Bundle ID)"
fi
echo

create_app_bundle() {
    local example_name="$1"
    local bundle_id="$2"
    local binary_path="target/release/$example_name"
    local app_name="${example_name}.app"
    local app_path="target/release/$app_name"
    
    echo "📦 Creating app bundle for $example_name..."
    
    # Create app bundle structure
    mkdir -p "$app_path/Contents/MacOS"
    mkdir -p "$app_path/Contents/Resources"
    
    # Copy binary to bundle
    cp "$binary_path" "$app_path/Contents/MacOS/$example_name"
    
    # Create Info.plist from template
    sed -e "s/EXECUTABLE_NAME/$example_name/g" \
        -e "s/BUNDLE_ID/$bundle_id/g" \
        ../Info.plist >"$app_path/Contents/Info.plist"
    
    echo "✅ App bundle created: $app_path"
    return 0
}

for example in "${EXAMPLES[@]}"; do
    echo "📦 Building $example..."
    cd "$example"
    
    bundle_id=$(get_bundle_id "$example")
    if [ -z "$bundle_id" ]; then
        echo "❌ No bundle ID found for $example"
        exit 1
    fi
    
    # Build the example
    cargo build --release
    
    # Create app bundle
    create_app_bundle "$example" "$bundle_id"
    
    app_path="target/release/${example}.app"
    
    if [ "$SKIP_SIGNING" = false ]; then
        echo "✍️ Ad-hoc signing $example app bundle..."
        # Ad-hoc sign the entire app bundle
        codesign --force --deep --sign "-" "$app_path"
        
        # Verify the signature
        echo "🔍 Verifying ad-hoc signature for $example..."
        if codesign --verify --deep --verbose "$app_path" 2>/dev/null; then
            echo "✅ $example app bundle ad-hoc signed and verified successfully"
        else
            echo "❌ Failed to verify ad-hoc signature for $example"
            exit 1
        fi
    else
        echo "⚠️ Skipping signing for $example (--no-sign flag used)"
    fi
    
    echo "📍 App bundle location: $(pwd)/$app_path"
    echo "📍 Bundle ID: $bundle_id"
    echo
    cd ..
done

echo "🎉 All examples built and packaged successfully!"

if [ "$SKIP_SIGNING" = false ]; then
    echo "✅ All app bundles ad-hoc signed (local development signing)"
    echo
    echo "🔧 To run a signed app bundle:"
    echo "   open examples/<example_name>/target/release/<example_name>.app"
    echo "   # or double-click the .app file in Finder"
    echo
    echo "🔧 Alternative command line execution:"
    echo "   examples/<example_name>/target/release/<example_name>.app/Contents/MacOS/<example_name>"
    echo
    echo "ℹ️ Ad-hoc signatures are valid for local execution but cannot be distributed"
else
    echo "💡 To build with ad-hoc signing, run:"
    echo "   ./build_and_sign.sh"
    echo
    echo "🔧 To run an unsigned app bundle:"
    echo "   open examples/<example_name>/target/release/<example_name>.app"
fi

echo
echo "📚 See examples/README.md for detailed usage instructions"
echo "💡 App bundles with Info.plist are required for macOS notifications to work properly"

if [ -n "$UNIFIED_BUNDLE_ID" ]; then
    echo
    echo "🔄 Unified Bundle ID Benefits:"
    echo "   • All examples share the same notification permissions"
    echo "   • Grant permission once, applies to all test apps"
    echo "   • Consistent notification center grouping"
    echo
    echo "💡 Usage examples:"
    echo "   ./build_and_sign.sh --unified                           # Use default unified bundle ID"
    echo "   ./build_and_sign.sh --unified-bundle-id com.yourapp.id  # Use custom unified bundle ID"
fi
