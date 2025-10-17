# User Profiles, Privacy, and Customization
## A Guide for Robert Users

## Overview

Robert is designed for **families and shared computers**. Each person gets their own private workspace with personalized browser profiles, custom commands, and encrypted storage. This guide explains how to use profiles effectively while maintaining your privacy.

## Why User Profiles?

**Scenario:** Alice and Bob share a family computer.

**Without Profiles:**
- Alice's shopping history mixes with Bob's
- Bob sees Alice's saved passwords and cookies
- Commands created by Alice appear for Bob
- No privacy or personalization

**With Profiles:**
- Alice has her own encrypted workspace
- Bob has his own encrypted workspace
- Browser data never mixes
- Commands are private to each user
- Personalized AI interactions

## Key Concepts

### User Profile

Your **user profile** is your personal account in Robert. It includes:

- **Username and password**: How you log in
- **Encrypted storage**: All your data is encrypted with your password
- **Browser profiles**: Your saved browser sessions
- **Commands**: Your custom automation workflows
- **Preferences**: Your AI agent knows your style and goals

### Browser Profile

A **browser profile** is like having multiple browsers, each with its own:

- Cookies and logged-in accounts
- Browsing history
- Bookmarks and extensions
- Cache and saved data

**Why use browser profiles?**

- **Separate work and personal**: Keep work accounts separate from personal shopping
- **Different personas**: One profile for writing, one for research
- **Privacy**: Use clean ephemeral profiles for privacy-sensitive tasks

**Types of browser profiles:**

1. **Ephemeral (default)**: Clean, temporary browser with no history. Deleted after you close it.
2. **Default**: Your main persistent browser with saved logins and history.
3. **Named profiles**: Special-purpose browsers like "shopping" or "work".

### Commands

A **command** is a custom automation you create with AI help. Examples:

- "Search for winter jackets under $200"
- "Check prices on my wishlist items"
- "Research competitors in the fashion industry"

Commands are:
- **Reusable**: Create once, run many times
- **Parameterized**: Fill in details each time (budget, size, color, etc.)
- **Self-improving**: Gets better with your feedback
- **Private**: Only you can see and use your commands

## Getting Started

### First Time Setup

**If you're the only user:**

1. Launch Robert for the first time
2. App auto-creates a default profile for you
3. Enter a username (e.g., "Alice")
4. Create a strong password (12+ characters)
5. You're in! Robert loads with a clean ephemeral browser

**If multiple people use the computer:**

1. First person follows the steps above
2. Other users click "Add Profile" button
3. Each person creates their own username and password
4. Next time, use the profile selector to log in

### Logging In

**Profile Selector:**

```
┌───────────────────────────────┐
│   Welcome to Robert           │
│                               │
│   Select Your Profile:        │
│   ┌─────────────────────┐    │
│   │ Alice            ▼  │    │
│   │ Bob                 │    │
│   │                     │    │
│   └─────────────────────┘    │
│                               │
│   Password:                   │
│   ┌─────────────────────┐    │
│   │ ●●●●●●●●●●●         │    │
│   └─────────────────────┘    │
│                               │
│   [  Login  ]  [Add Profile]  │
└───────────────────────────────┘
```

1. Select your username from dropdown
2. Enter your password
3. Click "Login"
4. Robert decrypts your workspace and loads your preferences

## Working with Browser Profiles

### Ephemeral Profiles (Privacy Mode)

**When to use:**
- Browsing without leaving a trace
- Testing automations
- One-off tasks
- Avoiding cookies and tracking

**How it works:**
1. Start Robert without selecting a browser profile
2. App creates a clean temporary browser
3. Do your work
4. Close browser when done
5. Everything is deleted (cookies, history, cache)

**Example:** "I want to search for a gift without it showing up in my shopping history."

### Creating a Named Profile

**When to use:**
- You want to stay logged in to accounts
- You have a specific workflow (shopping, work, writing)
- You want to save bookmarks and extensions

**How to create:**
1. Go to Settings → Browser Profiles
2. Click "New Browser Profile"
3. Enter a name (e.g., "shopping" or "work")
4. Profile is created and ready to use

**Example profiles:**
- **shopping**: Logged into retailers, saved cart items, wishlist
- **work**: Work email, Slack, project management tools
- **writing**: Research tools, writing apps, saved drafts

### Setting a Default Profile

**Why set a default?**
- Skip choosing a profile every time
- Always use your main browser with saved logins
- Commands inherit your default unless specified otherwise

**How to set:**
1. Go to Settings → Browser Profiles
2. Find the profile you want (e.g., "shopping")
3. Click "Set as Default"
4. Future sessions use this profile automatically

**Note:** You can still override the default by selecting a different profile manually.

### Choosing a Profile for Each Session

When you start a new session, you can choose which profile to use:

```
┌───────────────────────────────┐
│   Start New Session           │
│                               │
│   Browser Profile:            │
│   ┌─────────────────────┐    │
│   │ Ephemeral (Clean) ▼ │    │
│   │ Default             │    │
│   │ Shopping            │    │
│   │ Work                │    │
│   └─────────────────────┘    │
│                               │
│   [  Start  ]                 │
└───────────────────────────────┘
```

## Working with Commands

### What Are Commands?

Commands are **reusable automations** you create with AI help. Think of them as personalized shortcuts for tasks you do repeatedly.

**Example command: "Clothing Search"**

- **What it does:** Searches multiple retailers for clothing items
- **Parameters:** Outfit type, size, budget, color
- **Rules:** Never shop on Amazon, prioritize free returns
- **Output:** Top 5 options with images, prices, and shipping info

### Creating a Command

**Step 1: Describe your task**

Open the chat interface and describe what you want:

```
You: "I want to search for winter jackets under $200 on multiple
     websites, but never on Amazon. Show me the top 5 options
     sorted by price."
```

**Step 2: AI generates a command**

The AI creates a command with:
- Parameter form (budget, size, color)
- Rules (no Amazon, min 4-star ratings)
- Checklist (search 3 retailers, capture images, etc.)

**Step 3: Review and approve**

You see a preview:

```
Command Name: winter-jacket-search
Description: Search for winter jackets across retailers

Parameters:
  - Max price: $200
  - Size: [dropdown]

Rules:
  - Exclude Amazon
  - 4+ star ratings only

Would you like to save this command?
[Yes, Save It]  [Request Changes]
```

**Step 4: Command is saved**

Your command is saved to `~/.robert/users/alice/commands/winter-jacket-search.md` and appears in your commands dropdown.

### Running a Command

**Step 1: Select command**

Click the commands dropdown in the chat interface:

```
┌───────────────────────────────┐
│   Commands                 ▼  │
│   ────────────────────────    │
│   winter-jacket-search        │
│   clothing-search             │
│   research-topic              │
│   check-prices                │
└───────────────────────────────┘
```

**Step 2: Fill in parameters**

A form appears with your command's parameters:

```
┌───────────────────────────────┐
│   Winter Jacket Search        │
│                               │
│   What are you looking for?   │
│   ┌─────────────────────┐    │
│   │ winter parka        │    │
│   └─────────────────────┘    │
│                               │
│   Size:                       │
│   ┌─────────────────────┐    │
│   │ M                ▼  │    │
│   └─────────────────────┘    │
│                               │
│   Max Budget: $150            │
│   [●─────────────────] $500   │
│                               │
│   [  Run Command  ]           │
└───────────────────────────────┘
```

**Step 3: Watch it run**

Browser opens and automation starts. You see:
- Real-time progress
- Screenshots being captured
- Results as they're found

**Step 4: Review results**

Command completes and shows:
- Top 5 jacket options
- Images, prices, and links
- Shipping information

**Step 5: Provide feedback**

```
Results look good?
[👍 Yes]  [👎 No]
```

### Refining Commands

Commands improve over time based on your feedback.

**When to refine:**
- Command didn't work as expected
- You want to add new features
- Error occurred during execution

**How to refine:**

1. Click 👎 after a command runs
2. Describe what went wrong: "Timeout was too short, pages didn't load"
3. AI suggests improvements
4. Review suggestions and approve
5. Command is updated (version 1.1.0)

**Example evolution:**

```
Version 1.0.0: Initial creation
Version 1.1.0: Increased timeout from 3s to 10s
Version 1.2.0: Added color filter parameter
Version 1.3.0: Improved price comparison logic
```

### Command Parameters and Generative UI

Commands use **generative UI** - dynamic forms that adapt to your needs.

**Parameter types:**

| Type | Example |
|------|---------|
| Text input | "What outfit are you looking for?" |
| Dropdown | Size: S, M, L, XL |
| Slider | Budget: $0 - $500 |
| Color picker | Preferred color |
| Date picker | Delivery by date |
| Checkbox | Free shipping only |

**Example form:**

```
┌───────────────────────────────┐
│   Clothing Search             │
│                               │
│   Outfit Type:                │
│   ┌─────────────────────┐    │
│   │ winter jacket       │    │
│   └─────────────────────┘    │
│                               │
│   Size:  [M ▼]                │
│                               │
│   Max Budget: $150            │
│   [●─────────────────] $500   │
│                               │
│   Preferred Color:            │
│   [🎨]  (optional)            │
│                               │
│   ☑ Free shipping only        │
│                               │
│   [  Run Command  ]           │
└───────────────────────────────┘
```

The AI generates this form automatically based on your command's needs. You can refine the UI by asking: "Add a color picker for the color preference."

## Privacy and Security

### Password Protection

**Your password protects everything:**
- All files in your user directory are encrypted
- Without your password, no one can access your data
- Lost password = lost data (no recovery possible)

**Password best practices:**
- Use 12+ characters
- Mix uppercase, lowercase, numbers, symbols
- Don't reuse passwords from other services
- Store in a password manager

**What's encrypted:**
- `user.json` (your settings)
- `user-profile.md` (your AI context)
- All commands (`commands/*.md`)

**What's NOT encrypted:**
- Browser profile data (Chromium handles its own encryption)
- Temporary ephemeral profiles (deleted after use anyway)

### How Encryption Works

**Technical details** (for the curious):

```
Your Password
    ↓
Argon2id Key Derivation
    ↓
256-bit Encryption Key
    ↓
AES-256-GCM Encryption
    ↓
Encrypted Files on Disk
```

**What this means:**
- Industry-standard encryption (Argon2id + AES-256)
- Brute force attacks are impractical (would take centuries)
- Your data is secure even if someone steals your computer

### Profile Isolation

**How users are separated:**

1. **Encrypted directories**: Each user's files are encrypted separately
2. **UI restrictions**: You only see your own profiles, commands, and browser profiles
3. **Single-user lock**: Only one user can be logged in at a time

**What this prevents:**
- Alice can't see Bob's commands
- Bob can't access Alice's browser profiles
- No cross-contamination of cookies or history

**Note on file system:**
- Encryption is the primary security mechanism
- File system permissions are NOT enforced
- If someone has physical access and your password, they can access your data

### Browser Profile Privacy

**Ephemeral profiles:**
- Leave no trace (cookies, history, cache deleted)
- Perfect for privacy-sensitive tasks
- Can't be recovered after session ends

**Named profiles:**
- Persistent across sessions
- Stored in your encrypted user directory
- Isolated from system browser and other users
- Only accessible while you're logged in

### What Robert Collects

**Local-first philosophy:**
- ALL data stays on your computer
- No telemetry sent to servers
- No usage tracking
- No account registration

**Optional cloud inference:**
- If you enable cloud AI (GPT-4, Claude), prompts are sent to APIs
- Screenshots and page data are obfuscated (sensitive data redacted)
- You can audit what's sent before transmission
- Local inference is always available as alternative

## Customization

### User Profile Context

Your `user-profile.md` file helps the AI understand you better.

**What it contains:**

```markdown
# User Profile: Alice

## Preferences
- Tech-savvy, comfortable with technical terms
- Values privacy and data minimization
- Prefers detailed explanations

## Goals
- Automate repetitive research tasks
- Streamline online shopping workflows

## Language Style
- Professional and direct
- No emojis or casual language
- Prefers bullet points over paragraphs
```

**How it's used:**
- Included with every AI prompt as context
- AI adapts responses to your style
- Commands reflect your preferences

**How to customize:**
1. Go to Settings → User Profile
2. Edit your profile document
3. Add preferences, goals, language style
4. Save changes

### Command Customization

**Customize commands to match your needs:**

**Rules:** Set constraints
```markdown
## Rules
- Never shop on Amazon
- Prioritize sustainable brands
- Only 4+ star ratings
```

**Checklists:** Define success criteria
```markdown
## Checklist
- [ ] Search at least 3 retailers
- [ ] Check return policies
- [ ] Compare shipping costs
```

**Generative UI:** Request specific form elements
```
You: "Add a color picker to the clothing search command"
AI: Updates generative UI JSON with color picker component
```

### App Preferences

**Settings you can customize:**

**Theme:**
- Light mode
- Dark mode
- System preference (auto)

**Inference:**
- Local (on-device, private)
- Cloud (GPT-4, Claude, etc.)

**Timeouts:**
- Default timeout for page loads (milliseconds)
- Automation retry attempts

**Language:**
- UI language (English, Spanish, etc.)

**To change settings:**
1. Click Settings icon
2. Adjust preferences
3. Changes save automatically

## Best Practices

### For Families

**Create separate profiles for each person:**
- Parents: "Mom", "Dad"
- Kids: "Emma", "Noah"

**Use appropriate browser profiles:**
- Kids: Ephemeral for privacy and safety
- Parents: Named profiles for convenience

**Set up parental controls** (future feature):
- Limit which websites kids can access
- Approve commands before execution
- Monitor automation activity

### For Privacy

**Use ephemeral profiles:**
- Shopping for gifts (avoid recommendation pollution)
- Researching sensitive topics
- Testing websites without leaving traces

**Create purpose-specific named profiles:**
- "banking": Only financial websites
- "health": Medical research and portals
- "shopping": Retailers and review sites

**Regular cleanup:**
- Delete unused browser profiles
- Review and remove old commands
- Clear temporary files from Settings

### For Productivity

**Create commands for repeated tasks:**
- Daily news check
- Price monitoring
- Competitor research
- Social media posting

**Organize browser profiles by workflow:**
- "morning-routine": News, email, calendar
- "deep-work": Focus apps, no distractions
- "admin": Banking, bills, errands

**Use default profile wisely:**
- Set your most-used profile as default
- Override when needed for specific tasks
- Don't mix work and personal in default

## Troubleshooting

### "Incorrect Password"

**Cause:** Wrong password entered
**Solution:** Re-enter password carefully. Remember: passwords are case-sensitive.

**If you forgot your password:**
- There is NO password recovery
- Your encrypted data cannot be accessed
- You'll need to create a new profile (old data lost)

### "Browser Profile Not Found"

**Cause:** Command specifies a profile that doesn't exist
**Solution:**
1. Click "Refine Command"
2. AI will suggest changing to an existing profile or creating the missing one
3. Approve changes

### "User Already Active"

**Cause:** Another user is logged in
**Solution:**
1. Other user must log out first
2. Or click "Close All Sessions and Switch" to force logout

### Commands Not Appearing

**Cause:** Commands are user-specific
**Solution:** Make sure you're logged in as the correct user. Commands from other users won't appear.

### Ephemeral Profile Not Cleaning Up

**Cause:** Browser didn't close properly
**Solution:**
1. Manually close browser windows
2. Restart Robert
3. App will clean up orphaned ephemeral profiles

## FAQ

**Q: Can I share commands with other users?**
A: Not in v1.0. Commands are private to each user. Future versions may support command templates (sanitized for sharing).

**Q: Can multiple users be logged in at once?**
A: No. Only one user can have active sessions at a time. This prevents confusion about whose automation is running.

**Q: Can I have multiple browser profiles open simultaneously?**
A: Yes! You can have "shopping" and "work" profiles open at the same time as the same user.

**Q: What happens if I forget my password?**
A: Your encrypted data cannot be recovered. You'll need to create a new profile. **Back up important commands** to avoid data loss.

**Q: Can I import Chrome bookmarks or history?**
A: Not currently. Browser profiles start fresh. You can manually bookmark sites or use a named profile and log in to Chrome sync.

**Q: How do I export a command?**
A: Commands are Markdown files. You can copy them from `~/.robert/users/{username}/commands/` but be careful not to share sensitive data.

**Q: Can I use Robert without creating a user profile?**
A: On first launch, a default profile is auto-created. You'll need to set a password for security.

**Q: Does Robert track my browsing?**
A: No. Browsing happens in isolated Chromium instances. Robert only captures data you explicitly request (screenshots, text extraction).

**Q: Can I use my regular Chrome profile in Robert?**
A: Not recommended. Robert's browser profiles are separate to maintain isolation. Mixing could cause conflicts.

## Summary

**User profiles** give you:
- Privacy through encryption
- Personalization via user-profile.md
- Isolation from other users on shared computers

**Browser profiles** give you:
- Clean ephemeral browsers for privacy
- Persistent named profiles for convenience
- Separate contexts for different workflows

**Commands** give you:
- Reusable automations
- Parameterized workflows
- Self-improving through feedback

**Together**, these features make Robert a powerful, private, personalized automation tool for everyone in your family.

---

**Questions or feedback?**
- Open an issue: https://github.com/yourusername/robert/issues
- Read the docs: https://docs.robert.app

**Happy automating!**
