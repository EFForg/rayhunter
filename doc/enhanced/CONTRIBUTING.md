# Contributing to Rayhunter Enhanced

Thank you for your interest in contributing to Rayhunter Enhanced! This project aims to advance defensive cellular security research while maintaining the highest ethical standards.

## 🤝 Code of Conduct

### Our Pledge
- **Defensive Security**: Focus on protective and educational capabilities only
- **Responsible Research**: Follow ethical guidelines for security research
- **Legal Compliance**: Ensure all contributions comply with applicable laws
- **Respectful Community**: Maintain a welcoming environment for all contributors

### Prohibited Contributions
- Tools or techniques for **unauthorized surveillance**
- Methods for **illegal interception** of communications
- **Offensive capabilities** without clear defensive purpose
- Code that **violates privacy** or telecommunications laws
- **GPS tracking tools** for unauthorized location monitoring

## 🚀 Getting Started

### Development Environment Setup

#### Option 1: Docker Environment (Recommended)
```bash
# Clone the repository
git clone https://github.com/[your-org]/rayhunter-enhanced.git
cd rayhunter-enhanced

# Start Docker environment
./docker-build.sh up
./docker-build.sh shell

# Inside container - setup environment
./setup_ubuntu_ci.sh
./fetch_source.sh
```

#### Option 2: Local Development
```bash
# Clone the repository
git clone https://github.com/[your-org]/rayhunter-enhanced.git
cd rayhunter-enhanced

# Install dependencies
./setup_local_deps.sh  # No root required
# OR
./setup_ubuntu_ci.sh   # System-wide (requires sudo)
```

#### Option 3: Manual Setup
```bash
# Rust (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# ARM cross-compilation target
rustup target add armv7-unknown-linux-musleabihf

# Python dependencies
pip3 install -r requirements.txt
```

### Build and Test
```bash
# Test cross-compilation environment
./test_cross_compilation.sh

# Build everything
./build_all.sh

# Run tests
cargo test
python3 -m pytest tools/tests/
```

## 📋 Contribution Guidelines

### Types of Contributions Welcome

#### 🔍 **Security Research**
- New attack detection algorithms
- Improved cellular protocol analysis
- Enhanced forensic capabilities
- Documentation of attack patterns
- GPS correlation analysis improvements

#### 🛠️ **Tool Improvements**
- Performance optimizations
- New export formats
- Better error handling
- Cross-platform compatibility
- Docker environment enhancements
- GPS API improvements

#### 📖 **Documentation**
- Usage examples with sanitized data
- Technical analysis guides
- Legal compliance documentation
- Installation and setup improvements
- GPS integration guides
- Docker environment documentation

#### 🧪 **Testing**
- Unit tests for analysis functions
- Integration tests with sample data
- Validation of detection accuracy
- Performance benchmarks
- GPS API testing
- Cross-compilation verification

### Contribution Process

1. **Check existing issues** - Avoid duplicate work
2. **Create an issue** - Discuss your idea before coding
3. **Fork the repository** - Create your feature branch
4. **Develop your changes** - Follow coding standards
5. **Test thoroughly** - Include tests for new functionality
6. **Submit a Pull Request** - Follow the PR template

### Pull Request Requirements

#### Code Quality
- **Rust code**: Follow `rustfmt` formatting
- **Python code**: Follow PEP 8 style guidelines
- **Documentation**: Include inline comments and README updates
- **Tests**: Add appropriate test coverage
- **Cross-compilation**: Test ARM builds

#### Security Review
- **No sensitive data** in commits or tests
- **Input validation** for all user-provided data
- **Error handling** for malformed inputs
- **Memory safety** considerations
- **GPS data validation** for location information

#### Legal Compliance
- **Defensive purpose** clearly documented
- **No illegal capabilities** included
- **Proper attribution** for external code
- **License compatibility** verified
- **Privacy compliance** for GPS features

## 🔧 Development Standards

### Code Style

#### Rust
```rust
// Use descriptive function names
fn analyze_cellular_downgrade_patterns(qmdl_data: &[u8]) -> Result<AttackPattern, Error> {
    // Clear error handling
    validate_input(qmdl_data)?;
    
    // Documented algorithms
    let patterns = extract_rrc_patterns(qmdl_data)?;
    Ok(analyze_patterns(patterns))
}

// GPS correlation function example
fn correlate_gps_cellular_data(
    gps_points: &[GpsPoint], 
    cell_observations: &[CellObservation],
    time_threshold: Duration
) -> Result<Vec<CorrelatedObservation>, Error> {
    validate_gps_data(gps_points)?;
    validate_cellular_data(cell_observations)?;
    
    let correlations = find_correlations(gps_points, cell_observations, time_threshold)?;
    Ok(correlations)
}
```

#### Python
```python
def correlate_gps_cellular_data(gps_points: List[GpsPoint], 
                               cell_observations: List[CellObservation],
                               time_threshold: int = 30) -> List[CorrelatedObservation]:
    """
    Correlate GPS coordinates with cellular observations.
    
    Args:
        gps_points: List of GPS coordinates with timestamps
        cell_observations: List of cellular network observations
        time_threshold: Maximum time difference for correlation (seconds)
    
    Returns:
        List of correlated observations with distance calculations
    """
```

### Testing Requirements

#### Unit Tests
- **Test all analysis functions** with known inputs/outputs
- **Edge cases**: Empty data, malformed inputs, extreme values
- **Performance tests**: Large file handling, memory usage
- **GPS validation**: Coordinate range and format testing

#### Integration Tests
- **End-to-end workflows** with sanitized sample data
- **Tool interoperability** (SCAT integration, export formats)
- **Cross-platform compatibility** testing
- **Docker environment** testing
- **GPS API** integration testing

### Documentation Standards

#### Function Documentation
- **Purpose**: What the function does
- **Parameters**: Type and meaning of each parameter
- **Returns**: Type and meaning of return value
- **Raises**: Exceptions that may be thrown
- **Example**: Usage example with sample data

#### API Documentation
- **Update GPS_API_DOCUMENTATION.md** for new APIs
- **Include security considerations** for each function
- **Provide usage examples** with sanitized data
- **Document Docker environment** setup and usage

## 📊 Sample Data Guidelines

### Creating Test Data
- **Never use real captures** containing personal information
- **Generate synthetic data** that mimics real patterns
- **Sanitize existing data** by removing identifiable information
- **Document data sources** and sanitization methods
- **Use fake GPS coordinates** for testing (e.g., 0.0, 0.0 or test ranges)

### Example Test Data Structure
```python
# Good: Synthetic test data
test_cell_observation = CellObservation(
    timestamp=1642694400,  # Fixed timestamp
    cell_id=123456,        # Fake cell ID
    mcc=001,              # Test MCC
    mnc=01,               # Test MNC
    source="test_data"
)

# Good: Synthetic GPS data
test_gps_point = GpsPoint(
    timestamp=1642694400,  # Fixed timestamp
    latitude=0.0,         # Test coordinates
    longitude=0.0,        # Test coordinates
    source="test_data"
)

# Bad: Real data
real_observation = CellObservation(
    timestamp=actual_timestamp,
    cell_id=real_cell_id,  # Contains real network info
    mcc=310,              # Real US MCC
    mnc=260               # Real T-Mobile MNC
)
```

## 🔒 Security Considerations

### Data Handling
- **Input validation** for all file formats
- **Boundary checking** for array access
- **Memory management** to prevent leaks
- **Sanitized output** options
- **GPS coordinate validation** (range and format)

### Error Handling
```rust
// Good: Proper error handling
match parse_qmdl_message(data) {
    Ok(message) => process_message(message),
    Err(ParseError::InvalidFormat) => {
        log::warn!("Invalid QMDL format, skipping message");
        continue;
    },
    Err(e) => return Err(e),
}

// Good: GPS validation
match validate_gps_coordinates(lat, lon) {
    Ok(coords) => save_gps_data(coords),
    Err(GpsError::InvalidLatitude) => {
        log::warn!("Invalid latitude: {}", lat);
        return Err(GpsError::InvalidLatitude);
    },
    Err(e) => return Err(e),
}

// Bad: Panicking on errors
let message = parse_qmdl_message(data).unwrap(); // Could panic!
```

### GPS Data Security
- **Validate all GPS coordinates** before processing
- **Sanitize location data** before export
- **Use synthetic coordinates** for testing
- **Implement rate limiting** for GPS API endpoints
- **Log GPS data access** for security auditing

## 🐳 Docker Development

### Docker Environment Features
- **Isolated build environment** with Ubuntu 22.04
- **Persistent storage** for development work
- **USB device access** for testing
- **Pre-configured toolchains** for ARM cross-compilation
- **Consistent development environment** across contributors

### Docker Development Workflow
```bash
# Start development environment
./docker-build.sh up
./docker-build.sh shell

# Setup environment (first time only)
./setup_ubuntu_ci.sh

# Get latest source
./fetch_source.sh

# Build and test
./build_all.sh
cargo test

# Deploy to device for testing
./deploy.sh
```

## 🏆 Recognition

### Contributor Hall of Fame
Outstanding contributors will be recognized in:
- **README.md** acknowledgments section
- **Release notes** for significant contributions
- **Conference presentations** (with permission)

### Types of Recognition
- **Security Research**: Novel attack detection methods
- **Tool Development**: Major feature implementations
- **Documentation**: Comprehensive guides and examples
- **Community**: Helping other contributors and users
- **GPS Integration**: Location-based analysis improvements
- **Docker Environment**: Build system enhancements

## 📞 Getting Help

### Communication Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Pull Request Reviews**: Code-specific feedback

### Mentorship
- **New contributors welcome**: We'll help you get started
- **Code reviews**: Learn best practices through feedback
- **Pair programming**: Available for complex features
- **Docker environment support**: Help with containerized development

## 📄 Legal and Licensing

### Contributor License Agreement
By contributing, you agree that:
- Your contributions are your original work
- You have the right to license your contributions
- Your contributions will be licensed under the project's MIT license
- You understand the defensive security focus of this project

### Compliance Requirements
- **No illegal surveillance tools**
- **Respect telecommunications laws**
- **Follow responsible disclosure practices**
- **Maintain ethical research standards**
- **Comply with GPS data protection regulations**
- **Respect privacy laws** for location data

---

**Questions?** Open an issue or start a discussion. We're here to help make cellular communications more secure through responsible research and development.

**Remember**: Every contribution should make mobile communications safer and more secure for everyone.
