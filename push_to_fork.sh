#!/bin/bash

echo "🚀 Rayhunter Enhanced Fork Push Script"
echo "======================================"
echo ""
echo "This script will push your enhanced rayhunter code to your fork."
echo "Make sure you have created the fork at: https://github.com/drinkingc0ffee/rayhunter"
echo ""

# Check if fork exists
echo "Checking if fork exists..."
if curl -s -o /dev/null -w "%{http_code}" https://github.com/drinkingc0ffee/rayhunter | grep -q "200"; then
    echo "✅ Fork found! Proceeding with push..."
    echo ""
    
    # Push the feature branch
    echo "📤 Pushing feature/gps-api-integration branch..."
    git push origin feature/gps-api-integration
    
    # Push the tag
    echo "🏷️  Pushing v0.4.5 tag..."
    git push origin v0.4.5
    
    echo ""
    echo "🎉 Success! Your enhanced rayhunter is now available at:"
    echo "   https://github.com/drinkingc0ffee/rayhunter"
    echo ""
    echo "📋 What was pushed:"
    echo "   - Enhanced GPS integration with timestamp correlation"
    echo "   - Complete NDJSON export with SCAT compatibility"
    echo "   - All EFF suspicious cell algorithms integrated"
    echo "   - Deployment automation for ARM devices"
    echo "   - Comprehensive documentation"
    echo ""
    echo "🔗 Fork URL: https://github.com/drinkingc0ffee/rayhunter"
    echo "🏷️  Tag: v0.4.5"
    echo "🌿 Branch: feature/gps-api-integration"
    
else
    echo "❌ Fork not found!"
    echo ""
    echo "Please create the fork first:"
    echo "1. Go to: https://github.com/EFForg/rayhunter"
    echo "2. Click the 'Fork' button"
    echo "3. Wait for the fork to be created"
    echo "4. Run this script again"
    echo ""
    echo "Or run these commands manually after creating the fork:"
    echo "   git push origin feature/gps-api-integration"
    echo "   git push origin v0.4.5"
fi 