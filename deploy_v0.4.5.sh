#!/bin/bash -e

# Rayhunter v0.4.5 Deployment Script
# Deploys enhanced cellular data extraction with GPS integration

set -euo pipefail

echo "🦎 Rayhunter v0.4.5 Deployment Script"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if ADB is available
check_adb() {
    if ! command -v adb &> /dev/null; then
        print_error "ADB is not installed or not in PATH"
        print_status "Please install Android SDK Platform Tools"
        exit 1
    fi
}

# Check for connected devices
check_devices() {
    print_status "Checking for connected devices..."
    
    local devices=$(adb devices | grep -v "List of devices" | grep "device$" | wc -l)
    
    if [ "$devices" -eq 0 ]; then
        print_error "No devices connected"
        print_status "Please connect your device and enable USB debugging"
        exit 1
    fi
    
    print_success "Found $devices device(s) connected"
    adb devices
}

# Stop existing rayhunter daemon
stop_daemon() {
    print_status "Stopping existing rayhunter daemon..."
    adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon stop"' || true
    sleep 2
}

# Deploy rootshell
deploy_rootshell() {
    print_status "Deploying rootshell..."
    
    # Push rootshell to device
    adb push target/armv7-unknown-linux-musleabihf/release/rootshell /tmp/rootshell
    
    # Install rootshell with proper permissions
    adb shell '/bin/rootshell -c "cp /tmp/rootshell /bin/rootshell"'
    adb shell '/bin/rootshell -c "chown root /bin/rootshell"'
    adb shell '/bin/rootshell -c "chmod 4755 /bin/rootshell"'
    
    # Test rootshell
    adb shell '/bin/rootshell -c "id"' | grep -q "uid=0" && print_success "Rootshell deployed successfully" || print_error "Rootshell deployment failed"
}

# Deploy rayhunter daemon
deploy_daemon() {
    print_status "Deploying rayhunter daemon v0.4.5..."
    
    # Create rayhunter directory
    adb shell '/bin/rootshell -c "mkdir -p /data/rayhunter"'
    
    # Push daemon to device
    adb push target/armv7-unknown-linux-musleabihf/release/rayhunter-daemon /tmp/rayhunter-daemon
    
    # Install daemon
    adb shell '/bin/rootshell -c "mv /tmp/rayhunter-daemon /data/rayhunter/rayhunter-daemon"'
    adb shell '/bin/rootshell -c "chmod 755 /data/rayhunter/rayhunter-daemon"'
    
    print_success "Rayhunter daemon v0.4.5 deployed successfully"
}

# Deploy web interface
deploy_web() {
    print_status "Deploying web interface..."
    
    # Create web directory
    adb shell '/bin/rootshell -c "mkdir -p /data/rayhunter/web"'
    
    # Push web files
    adb push bin/web/build/ /tmp/web/
    adb shell '/bin/rootshell -c "cp -r /tmp/web/* /data/rayhunter/web/"'
    
    print_success "Web interface deployed successfully"
}

# Deploy init scripts
deploy_init_scripts() {
    print_status "Deploying init scripts..."
    
    # Check if init scripts exist
    if [ -f "scripts/rayhunter_daemon" ]; then
        adb push scripts/rayhunter_daemon /tmp/rayhunter_daemon
        adb shell '/bin/rootshell -c "mv /tmp/rayhunter_daemon /etc/init.d/rayhunter_daemon"'
        adb shell '/bin/rootshell -c "chmod 755 /etc/init.d/rayhunter_daemon"'
        print_success "Rayhunter daemon init script deployed"
    else
        print_warning "Init script not found, creating basic one..."
        
        # Create basic init script
        cat > /tmp/rayhunter_daemon << 'EOF'
#!/bin/sh /etc/rc.common

START=99
STOP=10

start() {
    echo "Starting rayhunter daemon v0.4.5..."
    /data/rayhunter/rayhunter-daemon &
}

stop() {
    echo "Stopping rayhunter daemon..."
    killall rayhunter-daemon || true
}
EOF
        
        adb push /tmp/rayhunter_daemon /tmp/rayhunter_daemon
        adb shell '/bin/rootshell -c "mv /tmp/rayhunter_daemon /etc/init.d/rayhunter_daemon"'
        adb shell '/bin/rootshell -c "chmod 755 /etc/init.d/rayhunter_daemon"'
        print_success "Basic init script created and deployed"
    fi
}

# Deploy configuration
deploy_config() {
    print_status "Deploying configuration..."
    
    # Check if config exists
    if [ -f "config.toml.example" ]; then
        adb push config.toml.example /tmp/config.toml
        adb shell '/bin/rootshell -c "mv /tmp/config.toml /data/rayhunter/config.toml"'
        print_success "Configuration deployed"
    else
        print_warning "No configuration file found, creating basic one..."
        
        # Create basic config
        cat > /tmp/config.toml << 'EOF'
# Rayhunter v0.4.5 Configuration
[daemon]
port = 8080
host = "0.0.0.0"
log_level = "info"

[analysis]
enable_cellular_extraction = true
enable_gps_integration = true
enable_security_analysis = true
export_ndjson = true
ndjson_timestamp_format = "unix"

[cellular]
extract_neighbors = true
extract_signal_quality = true
extract_network_info = true

[gps]
enable_location_tracking = true
correlate_with_cellular = true
EOF
        
        adb push /tmp/config.toml /tmp/config.toml
        adb shell '/bin/rootshell -c "mv /tmp/config.toml /data/rayhunter/config.toml"'
        print_success "Basic configuration created and deployed"
    fi
}

# Start daemon
start_daemon() {
    print_status "Starting rayhunter daemon v0.4.5..."
    adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon start"'
    sleep 3
    
    # Check if daemon is running
    if adb shell 'pgrep rayhunter-daemon' > /dev/null 2>&1; then
        print_success "Rayhunter daemon v0.4.5 started successfully"
    else
        print_error "Failed to start rayhunter daemon"
        exit 1
    fi
}

# Test deployment
test_deployment() {
    print_status "Testing deployment..."
    
    # Test web interface
    adb forward tcp:8080 tcp:8080
    sleep 2
    
    if curl -s http://localhost:8080 > /dev/null 2>&1; then
        print_success "Web interface accessible at http://localhost:8080"
    else
        print_warning "Web interface not accessible, daemon may still be starting"
    fi
    
    # Test daemon process
    if adb shell 'pgrep rayhunter-daemon' > /dev/null 2>&1; then
        print_success "Daemon process is running"
    else
        print_error "Daemon process not found"
    fi
}

# Main deployment process
main() {
    print_status "Starting rayhunter v0.4.5 deployment..."
    
    check_adb
    check_devices
    stop_daemon
    deploy_rootshell
    deploy_daemon
    deploy_web
    deploy_init_scripts
    deploy_config
    start_daemon
    test_deployment
    
    echo ""
    print_success "🎉 Rayhunter v0.4.5 deployment completed successfully!"
    echo ""
    echo "📱 Device Information:"
    adb shell 'uname -a'
    echo ""
    echo "🌐 Web Interface: http://localhost:8080"
    echo "📊 Enhanced Features:"
    echo "   - Cellular data extraction (SCAT-compatible)"
    echo "   - GPS location integration"
    echo "   - Security threat analysis"
    echo "   - Neighbor cell detection"
    echo "   - NDJSON export with Unix timestamps"
    echo ""
    echo "🔧 To check logs: adb shell 'tail -f /var/log/rayhunter.log'"
    echo "🔧 To restart: adb shell '/etc/init.d/rayhunter_daemon restart'"
}

# Run main function
main "$@" 