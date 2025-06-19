#!/bin/bash

# Build and Sign Script for User-Notify Test Examples
# Usage: ./build_and_sign.sh [DEVELOPER_ID]
# If DEVELOPER_ID is not provided, binaries will be built but not signed

set -e

DEVELOPER_ID="${1:-}"
EXAMPLES=(
    "test_permission_request"
    "test_basic_notification"
    "test_interactive_notification"
    "test_active_notifications"
    "test_full_integration"
)

echo "🚀 Building and signing user-notify test examples..."
echo "Developer ID: ${DEVELOPER_ID:-"Not provided - skipping signing"}"
echo

for example in "${EXAMPLES[@]}"; do
    echo "📦 Building $example..."
    cd "$example"
    
    # Build the example
    cargo build --release
    
    if [ -n "$DEVELOPER_ID" ]; then
        echo "✍️ Signing $example..."
        # Sign the binary
        codesign --force --sign "$DEVELOPER_ID" "target/release/$example"
        
        # Verify the signature
        echo "🔍 Verifying signature for $example..."
        if codesign --verify --verbose "target/release/$example" 2>/dev/null; then
            echo "✅ $example signed and verified successfully"
        else
            echo "❌ Failed to verify signature for $example"
            exit 1
        fi
    else
        echo "⚠️ Skipping signing for $example (no Developer ID provided)"
    fi
    
    echo "📍 Binary location: $(pwd)/target/release/$example"
    echo
    cd ..
done

echo "🎉 All examples built successfully!"

if [ -n "$DEVELOPER_ID" ]; then
    echo "✅ All examples signed with Developer ID: $DEVELOPER_ID"
    echo
    echo "🔧 To run a signed example:"
    echo "   cd examples/<example_name>"
    echo "   ./target/release/<example_name>"
else
    echo "💡 To sign the examples, run:"
    echo "   ./build_and_sign.sh \"Your Developer ID Application: Your Name (TEAM_ID)\""
    echo
    echo "🔧 To run an unsigned example:"
    echo "   cd examples/<example_name>"
    echo "   cargo run"
fi

echo
echo "📚 See examples/README.md for detailed usage instructions" 