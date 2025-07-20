# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.4.x-enhanced | :white_check_mark: |
| 0.4.x   | :white_check_mark: |
| 0.3.x   | :x:                |
| < 0.3   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue in Rayhunter Enhanced, please follow responsible disclosure:

### How to Report

1. **DO NOT** open a public GitHub issue for security vulnerabilities
2. Use GitHub's [private vulnerability reporting tool](https://github.com/[your-org]/rayhunter-enhanced/security/advisories/new)
3. **Include** the following information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact assessment
   - Suggested fix (if available)

### What to Expect

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity (Critical: 7 days, High: 30 days, Medium: 90 days)

### Security Best Practices for Users

#### Data Handling
- **Never commit** real QMDL files containing personal data
- **Sanitize** all data before sharing or publishing
- **Use isolated environments** for analysis of sensitive captures
- **Secure GPS data** - location information is highly sensitive
- **Validate GPS coordinates** before submission via API

#### Legal Compliance
- **Obtain explicit permission** before analyzing cellular networks
- **Comply with local laws** regarding telecommunications monitoring
- **Use only for authorized security research** and testing
- **Respect privacy laws** when collecting location data
- **Follow GPS data protection regulations** in your jurisdiction

#### Tool Security
- **Keep tools updated** to the latest version
- **Verify file integrity** before analyzing unknown captures
- **Run in sandboxed environments** when possible
- **Use Docker environment** for isolated builds and testing
- **Secure API endpoints** when deploying GPS features

#### GPS Data Security
- **Validate GPS coordinates** before API submission
- **Secure GPS API endpoints** with proper authentication if needed
- **Sanitize location data** before export or sharing
- **Use HTTPS** for GPS API communication when possible
- **Implement rate limiting** for GPS coordinate submissions

## Security Measures in Rayhunter Enhanced

### Built-in Protections
- **Input validation** for all file formats
- **Memory safety** through Rust implementation
- **Sanitized output** options for sensitive data
- **Configurable security settings**
- **GPS data validation** and coordinate range checking
- **API endpoint security** with input sanitization
- **Cross-compilation security** with isolated build environments

### GPS API Security Features
- **Coordinate validation** (latitude: -90 to 90, longitude: -180 to 180)
- **Format validation** for GPS coordinate submission
- **Server-side timestamp generation** to prevent timestamp manipulation
- **Input sanitization** for all GPS API endpoints
- **Error handling** without information disclosure

### Docker Environment Security
- **Isolated build environment** with Ubuntu 22.04
- **Persistent storage** with user-controlled data retention
- **USB device access** with proper permissions
- **No root access** required for local dependency installation
- **Environment isolation** prevents system contamination

### Enhanced Build System Security
- **Cross-compilation isolation** prevents ARM/host compiler conflicts
- **Environment verification** scripts ensure secure builds
- **Local dependency installation** without system modifications
- **Build artifact validation** before deployment

### Recommended Usage
- **Isolated analysis environment** (VM or container)
- **Docker environment** for secure builds and testing
- **Regular security updates**
- **Authorized use only** with proper permissions
- **Data destruction** after analysis completion
- **Secure GPS data handling** with proper validation

### GPS Data Protection
- **Local processing only** - no cloud connectivity
- **User-controlled data retention** and export
- **Automatic data validation** for all GPS coordinates
- **Secure export formats** (CSV, JSON, GPX)
- **Timestamp correlation** without external dependencies

## Privacy and Ethics

### GPS Data Privacy
- **Local storage only** - GPS data never leaves the device
- **User-controlled exports** - manual export required
- **No tracking** - coordinates only stored when explicitly submitted
- **Automatic correlation** with cellular captures for analysis only

### Responsible Use Guidelines
This tool is intended for:
- ✅ **Security research and education**
- ✅ **Network analysis and troubleshooting**  
- ✅ **Personal privacy protection**
- ✅ **Academic research with proper consent**
- ✅ **Authorized penetration testing**

**NOT intended for:**
- ❌ Illegal surveillance or interception
- ❌ Unauthorized monitoring of others
- ❌ Commercial espionage
- ❌ Violation of privacy laws
- ❌ Stalking or harassment using GPS data

---

**Remember**: This tool is designed for defensive security research. Misuse for unauthorized surveillance or interception is strictly prohibited and may violate local and international laws. GPS data collection must comply with all applicable privacy regulations.
