# Safety & Privacy Features

**Last Updated:** 2025-10-08

## Core Philosophy

> "Your data stays yours. Always."

Robert is designed with **privacy-first principles**. All inference runs locally by default. If you choose to use cloud AI (optional), we automatically protect your sensitive information.

## Local-First Inference

### Default: 100% Local Execution

**All AI inference runs on your device:**
- Voice-to-Markdown script generation
- Natural language understanding
- Automation step planning
- Error recovery suggestions

**What this means:**
- ✅ Zero data sent to cloud
- ✅ Complete privacy
- ✅ Works offline
- ✅ No API keys required
- ✅ No data retention by third parties
- ✅ No usage tracking

**Local AI Models:**
- Runs on your Mac's Neural Engine (when available)
- Uses efficient, quantized models
- Fast inference (<1 second for most operations)
- Models included with application (no downloads)

### Why Local-First?

**Privacy:**
- Your automations may handle sensitive data (credentials, personal info, financial data)
- You shouldn't have to trust a third party with your automation workflows
- Local execution means no data ever leaves your device

**Reliability:**
- Works without internet connection
- No API rate limits
- No service outages
- No dependency on external services

**Performance:**
- No network latency
- Instant responses
- Parallel operations without API throttling

**Cost:**
- Zero ongoing costs
- No subscription for basic features
- No pay-per-token charges

---

## Optional Cloud Inference

### Advanced Feature: Cloud AI Providers

**Users can optionally enable cloud inference for:**
- More powerful models (GPT-4, Claude 3.5 Sonnet, etc.)
- Complex automation generation
- Advanced error recovery
- Natural language queries

**Supported providers (opt-in):**
- OpenAI (GPT-4, GPT-4 Turbo)
- Anthropic (Claude 3.5 Sonnet, Claude 3 Opus)
- Custom API endpoints

**When you might want this:**
- Creating very complex automations
- Using advanced AI reasoning
- Generating scripts from lengthy descriptions
- Debugging complex failures

---

## Safety Features: Automatic Data Obfuscation

### The Problem

Cloud AI providers see everything you send. This could include:
- Passwords visible in forms
- Credit card numbers on checkout pages
- API keys in configuration panels
- Personal information (SSN, addresses, etc.)
- Private messages or emails
- Sensitive business data

### Our Solution: Multi-Layer Obfuscation

Robert automatically **scans and obfuscates sensitive information** before sending anything to cloud providers.

## Text Obfuscation

### What We Scan For

**Credentials:**
- Passwords (in forms, configuration)
- API keys and tokens
- Session cookies
- Authentication headers

**Financial Information:**
- Credit card numbers (PAN)
- CVV codes
- Bank account numbers
- Routing numbers
- Cryptocurrency addresses

**Personal Identifiable Information (PII):**
- Social Security Numbers (SSN)
- Driver's license numbers
- Passport numbers
- Phone numbers
- Email addresses (when sensitive)
- Physical addresses

**Sensitive Patterns:**
- Private keys (SSH, PGP, SSL)
- AWS/Azure/GCP credentials
- Database connection strings
- Environment variables

### How Text Obfuscation Works

**1. Pattern Detection:**
```
Original: "My password is SuperSecret123!"
Detected: Password field content
```

**2. Classification:**
```
Type: Password
Sensitivity: HIGH
Context: Form input field
```

**3. Obfuscation:**
```
Obfuscated: "My password is [PASSWORD_REDACTED_8CHARS]"
Token stored: LOCAL_TOKEN_a7f3b9c2
```

**4. Sent to Cloud:**
```
"User wants to fill form with email and [PASSWORD_REDACTED_8CHARS]"
```

**5. Response Processing:**
```
Cloud returns: "Fill input#password with [PASSWORD_REDACTED_8CHARS]"
Robert replaces: [PASSWORD_REDACTED_8CHARS] → SuperSecret123!
```

### Obfuscation Techniques

**Redaction:**
```
Before: password123
After:  [PASSWORD_REDACTED]
```

**Tokenization:**
```
Before: 4532-1234-5678-9012
After:  [CREDIT_CARD_TOKEN_1]
Stored locally: Token maps to original
```

**Pattern Masking:**
```
Before: john.doe@company.com
After:  [EMAIL_1]@[DOMAIN_1]
Preserves structure: yes
```

**Character Preservation (when needed):**
```
Before: API key sk-proj-abc123xyz789
After:  [API_KEY_12CHARS]
Preserves length: for context
```

---

## Image Obfuscation

### Visual Content Scanning

Screenshots and page images may contain sensitive information. Robert uses **computer vision** to detect and blur sensitive data before sending to cloud.

### What We Detect in Images

**Text in Images:**
- OCR extraction of all visible text
- Same pattern matching as text obfuscation
- Detected sensitive text gets blurred

**UI Elements:**
- Password fields (masked dots/circles)
- Credit card forms
- Personal information fields
- Private messages/emails

**Visual Patterns:**
- Credit card layouts (distinctive 4x4 digit patterns)
- Document headers (SSN forms, licenses)
- Banking interfaces
- Medical records

### Image Obfuscation Process

**1. Screenshot Captured:**
```
User automation includes taking screenshot
Image contains: Login form with username "john@example.com"
                and password "Secret123"
```

**2. OCR Extraction:**
```
Extracted text: "john@example.com", "Secret123", "Remember me", "Sign In"
```

**3. Pattern Matching:**
```
Detected: Email address in username field
Detected: Password in password field
```

**4. Bounding Box Calculation:**
```
Email field: x=100, y=200, width=300, height=40
Password field: x=100, y=260, width=300, height=40
```

**5. Visual Obfuscation:**
```
Apply Gaussian blur to regions
Blur strength: High (password) vs Medium (email)
Alternative: Black rectangles for maximum privacy
```

**6. Safe Image Created:**
```
[IMAGE with blurred password field]
Email shows: "john@[DOMAIN_REDACTED]"
Password shows: [BLURRED REGION]
```

**7. Sent to Cloud:**
```
Image sent with blurred regions
Context: "User filling login form"
Cloud AI never sees actual credentials
```

### Image Obfuscation Techniques

**Gaussian Blur:**
```
Effect: Smooth blur over sensitive regions
Use case: Partial privacy (shows layout, hides content)
Strength: Adjustable (medium to high)
```

**Pixelation:**
```
Effect: Large pixels obscure details
Use case: Maintaining rough layout while hiding text
Strength: 16x16 to 32x32 pixel blocks
```

**Black Boxes:**
```
Effect: Complete redaction
Use case: Maximum privacy for highly sensitive data
Color: Solid black or branded color
```

**Text Overlay:**
```
Effect: Replace with [REDACTED] label
Use case: Clear indication of obfuscation
Maintains: Original text position and size
```

---

## Obfuscation Architecture

### Three-Layer Protection

**Layer 1: Pre-Capture Protection**
```
Before screenshot/text capture:
- Warn if sensitive page detected
- Offer to skip capture
- Suggest alternative approaches
```

**Layer 2: Capture-Time Obfuscation**
```
During capture:
- Real-time pattern detection
- Immediate obfuscation
- Secure local storage of originals
```

**Layer 3: Pre-Transmission Protection**
```
Before sending to cloud:
- Final scan for missed patterns
- Verify all obfuscation applied
- User confirmation for sensitive contexts
```

### Local Token Storage

**How Tokens Work:**

```
Original sensitive data → Encrypted local storage
                       ↓
            Generate unique token
                       ↓
        Token sent to cloud instead
                       ↓
    Cloud returns automation with tokens
                       ↓
        Robert replaces tokens with originals
                       ↓
            Automation executes safely
```

**Security:**
- Tokens stored in encrypted keychain (macOS Keychain)
- Per-session tokens (cleared after automation runs)
- Never logged or persisted long-term
- Tokens are meaningless without local key

### Encryption

**Data at Rest:**
- Sensitive data encrypted using macOS Keychain
- AES-256 encryption
- Hardware-backed keys (Secure Enclave when available)

**Data in Transit (Local Only):**
- Internal IPC uses secure channels
- No network transmission in default mode

**Data in Transit (Cloud Opt-In):**
- TLS 1.3 for all API calls
- Certificate pinning
- Obfuscated data only

---

## User Controls

### Obfuscation Settings

**Sensitivity Levels:**

**Maximum (Default):**
- Obfuscate all potential sensitive data
- Conservative pattern matching
- May over-redact to ensure safety

**Balanced:**
- Obfuscate clear sensitive data
- Preserve more context for AI
- Good for complex automations

**Minimal:**
- Only obfuscate obvious secrets (passwords, credit cards)
- Maximum context for AI
- User takes more responsibility

### Manual Review

**Before Sending to Cloud:**
```
┌─────────────────────────────────────────┐
│  Review Before Sending to Cloud         │
├─────────────────────────────────────────┤
│                                          │
│  [IMAGE PREVIEW with obfuscated regions] │
│                                          │
│  Detected sensitive data:                │
│  • 1 password field → REDACTED           │
│  • 1 email address → OBFUSCATED          │
│  • 0 credit cards                        │
│                                          │
│  Text being sent:                        │
│  "User wants to log in to site with     │
│   [EMAIL_1] and [PASSWORD_REDACTED]"    │
│                                          │
│  [ ] Don't ask again for this session   │
│                                          │
│  [Cancel]  [Send Safely] ←             │
└─────────────────────────────────────────┘
```

### Audit Log

**What Gets Logged (Locally):**
- When cloud inference is used
- What was obfuscated
- Which provider received data
- Timestamp and context

**Audit Log Location:**
```
~/Library/Application Support/Robert/audit_log.json
```

**Never Logged:**
- Original sensitive data
- Decrypted tokens
- Cloud API responses with sensitive content

---

## Privacy Guarantees

### What We Never Do

❌ **Send data to cloud by default** - 100% local first
❌ **Store sensitive data long-term** - Cleared after session
❌ **Log passwords or keys** - Only obfuscated versions
❌ **Share data with third parties** - You control cloud provider choice
❌ **Train on your data** - No data retention
❌ **Track your usage** - No telemetry without opt-in

### What We Always Do

✅ **Local inference by default** - No cloud required
✅ **Obfuscate before cloud** - Multi-layer protection
✅ **Encrypt sensitive data** - Keychain integration
✅ **Give user control** - Clear opt-in for cloud
✅ **Transparent logging** - Audit what was shared
✅ **Open source** - Code is auditable

---

## Comparison with Competitors

| Feature | **Robert** | **Herd** | **Monitoro** | **Zapier** | **Claude/GPT** |
|---------|-----------|----------|--------------|------------|----------------|
| **Default Inference** | Local | Local | Cloud | Cloud | Cloud |
| **Cloud Opt-In** | Yes, optional | N/A | Required | Required | Required |
| **Auto Obfuscation** | ✅ Text + Images | ❌ | ❌ | ❌ | ❌ |
| **User Control** | Full control | Full control | Limited | Limited | None |
| **Audit Log** | ✅ Local | ⚠️ Limited | ❌ | ❌ | ❌ |
| **Open Source** | ✅ Auditable | ❌ | ❌ | ❌ | ❌ |

---

## Technical Implementation

### Pattern Detection (Text)

**Regular Expressions:**
```rust
// Password patterns
PASSWORD: r"password[:\s]*([^\s]+)"
API_KEY: r"(sk|pk)_[a-zA-Z0-9]{32,}"
CREDIT_CARD: r"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b"
SSN: r"\b\d{3}-\d{2}-\d{4}\b"
```

**Context-Aware Detection:**
```rust
// Check HTML context
if element.type == "password" {
    obfuscate(content, ObfuscationType::Password)
}

// Check nearby labels
if label.text.contains("credit card") {
    obfuscate(content, ObfuscationType::CreditCard)
}
```

### Computer Vision (Images)

**OCR Engine:**
- Tesseract OCR for text extraction
- ML-based field detection (TensorFlow Lite)
- Layout analysis for context

**Blur Implementation:**
```rust
// Gaussian blur for sensitive regions
fn blur_region(image: &Image, bbox: BoundingBox, strength: f32) {
    let kernel_size = calculate_kernel(strength);
    apply_gaussian_blur(image, bbox, kernel_size);
}
```

**Alternatives:**
- Metal shaders (GPU acceleration on macOS)
- CoreImage filters (native macOS)
- OpenCV (cross-platform fallback)

### Token Management

**Token Generation:**
```rust
use rand::Rng;
use aes_gcm::{Aes256Gcm, KeyInit};

fn generate_token(sensitive_data: &str) -> Token {
    let token_id = uuid::Uuid::new_v4();
    let encrypted = encrypt_to_keychain(sensitive_data);

    Token {
        id: token_id,
        placeholder: format!("[TOKEN_{}]", token_id),
        encrypted_ref: encrypted,
    }
}
```

**Token Replacement:**
```rust
fn replace_tokens(text: &str, tokens: &[Token]) -> String {
    let mut result = text.to_string();
    for token in tokens {
        let original = decrypt_from_keychain(&token.encrypted_ref);
        result = result.replace(&token.placeholder, &original);
    }
    result
}
```

---

## User Education

### Onboarding Flow

**First Time Cloud Inference:**
```
┌─────────────────────────────────────────┐
│  🔒 Privacy & Safety                     │
├─────────────────────────────────────────┤
│  Robert can use cloud AI for more       │
│  powerful automation generation.        │
│                                          │
│  When you use cloud AI:                 │
│  ✅ We automatically obfuscate passwords │
│  ✅ We blur sensitive data in images     │
│  ✅ You can review before sending        │
│  ✅ Everything stays local by default    │
│                                          │
│  You control which provider:            │
│  • OpenAI (GPT-4)                       │
│  • Anthropic (Claude)                   │
│  • Custom API                           │
│                                          │
│  [Learn More] [Continue] [Stay Local]   │
└─────────────────────────────────────────┘
```

### In-App Indicators

**Privacy Status Bar:**
```
┌─────────────────────────────────────────┐
│  🟢 Local Inference | No data sent      │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│  🟡 Cloud AI Active | Data obfuscated   │
└─────────────────────────────────────────┘
```

### Documentation

- **Privacy Policy** - Clear explanation of data handling
- **Safety Guide** - How obfuscation works
- **Video Tutorial** - Visual demonstration
- **FAQ** - Common privacy questions

---

## Future Enhancements

### Planned Features

**Advanced ML Detection:**
- Custom ML models for sensitive data detection
- Industry-specific patterns (healthcare, finance)
- User-trained patterns (learn what you consider sensitive)

**Selective Cloud Usage:**
```
"Use cloud AI for planning, but local for execution"
"Use cloud only for non-sensitive steps"
```

**Privacy Scoring:**
```
Automation Risk Level: Low
• No sensitive data detected
• Safe to use cloud inference

Automation Risk Level: High
• 3 passwords detected
• Credit card form present
• Recommend: Local inference only
```

**Community Patterns:**
- Share obfuscation patterns (anonymously)
- Improve detection over time
- Industry-specific pattern libraries

---

## Compliance

### Standards Alignment

**GDPR (European Union):**
- Data minimization ✅
- User consent (opt-in) ✅
- Right to erasure ✅
- Transparent processing ✅

**CCPA (California):**
- Consumer rights ✅
- Opt-out mechanisms ✅
- Data protection ✅

**SOC 2 (Future):**
- Security controls ✅
- Privacy controls ✅
- Audit logging ✅

---

## Summary

**Robert's Privacy Promise:**

1. **Local by default** - 100% of inference runs on your device
2. **You choose cloud** - Opt-in only, never required
3. **Auto-protection** - We obfuscate before sending
4. **Multi-layer security** - Text + images + context
5. **Full transparency** - Open source, audit logs
6. **User control** - You decide what's sensitive
7. **No surprises** - Clear indicators of what's sent where

**Bottom line:** Your automation, your data, your device. Always.

---

**Document Version:** 1.0
**Last Updated:** 2025-10-08
**Next Review:** After beta launch
