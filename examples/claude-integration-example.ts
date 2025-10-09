/**
 * Claude Integration Examples
 *
 * These examples demonstrate how to use Claude CLI integration
 * in the Robert browser automation app.
 */

import { invoke } from '@tauri-apps/api/core';

// Type definitions
interface ClaudeRequest {
  prompt: string;
  screenshot_path?: string;
  include_html: boolean;
  model?: string;
}

interface ClaudeResponse {
  text: string;
  metadata?: any;
}

/**
 * Example 1: Simple page analysis
 * Ask Claude about the current page without any manual setup
 */
async function example1_simpleAnalysis() {
  console.log('Example 1: Simple Page Analysis');

  // Launch browser and navigate
  await invoke('launch_browser');
  await invoke('navigate_to_url', { url: 'https://example.com' });

  // Ask Claude about the page (auto-captures screenshot and HTML)
  const response: ClaudeResponse = await invoke('ask_claude_about_page', {
    prompt: 'What are the main sections on this page? Describe the layout.',
    model: 'sonnet'
  });

  console.log('Claude says:', response.text);
}

/**
 * Example 2: Extract structured data
 * Use Claude to extract product information in JSON format
 */
async function example2_dataExtraction() {
  console.log('Example 2: Data Extraction');

  await invoke('navigate_to_url', { url: 'https://example.com/products' });

  const response: ClaudeResponse = await invoke('ask_claude_about_page', {
    prompt: `Extract all products from this page in JSON format.

    Return a JSON array with this structure:
    [
      {
        "name": "Product name",
        "price": "Price as string",
        "inStock": true/false
      }
    ]`,
    model: 'sonnet'
  });

  try {
    const products = JSON.parse(response.text);
    console.log('Extracted products:', products);
  } catch (e) {
    console.log('Raw response:', response.text);
  }
}

/**
 * Example 3: Form filling automation
 * Use Claude to identify form fields and their purposes
 */
async function example3_formAnalysis() {
  console.log('Example 3: Form Analysis');

  await invoke('navigate_to_url', { url: 'https://example.com/contact' });

  const response: ClaudeResponse = await invoke('ask_claude_about_page', {
    prompt: `Analyze the form on this page and return a JSON object describing each field:

    {
      "fields": [
        {
          "selector": "CSS selector for the field",
          "label": "Field label or purpose",
          "type": "text/email/textarea/etc",
          "required": true/false
        }
      ]
    }`,
    model: 'sonnet'
  });

  console.log('Form structure:', response.text);
}

/**
 * Example 4: Manual screenshot with custom analysis
 * Take screenshot explicitly and use it with custom prompt
 */
async function example4_customScreenshot() {
  console.log('Example 4: Custom Screenshot Analysis');

  await invoke('navigate_to_url', { url: 'https://example.com' });

  // Take screenshot manually
  const screenshotPath = await invoke('take_screenshot', {
    outputPath: '/tmp/robert-screenshot.png'
  });

  console.log('Screenshot saved to:', screenshotPath);

  // Use the screenshot with Claude
  const response: ClaudeResponse = await invoke('ask_claude', {
    request: {
      prompt: 'Describe the visual design of this page. What colors are used? What is the overall aesthetic?',
      screenshot_path: screenshotPath,
      include_html: false, // Only use screenshot, not HTML
      model: 'opus' // Use Opus for detailed visual analysis
    }
  });

  console.log('Visual analysis:', response.text);
}

/**
 * Example 5: Accessibility audit
 * Use Claude to check for accessibility issues
 */
async function example5_accessibilityAudit() {
  console.log('Example 5: Accessibility Audit');

  await invoke('navigate_to_url', { url: 'https://example.com' });

  const response: ClaudeResponse = await invoke('ask_claude_about_page', {
    prompt: `Perform an accessibility audit on this page. Check for:

    1. Images missing alt text
    2. Form inputs missing labels
    3. Poor color contrast
    4. Improper heading hierarchy
    5. Missing ARIA attributes where needed

    Return findings in this JSON format:
    {
      "score": "A-F grade",
      "issues": [
        {
          "type": "missing-alt/poor-contrast/etc",
          "severity": "critical/warning/info",
          "description": "What's wrong",
          "element": "CSS selector or description"
        }
      ],
      "summary": "Overall assessment"
    }`,
    model: 'opus' // Use Opus for thorough analysis
  });

  console.log('Accessibility report:', response.text);
}

/**
 * Example 6: Multi-page analysis
 * Navigate through multiple pages and aggregate insights
 */
async function example6_multiPageAnalysis() {
  console.log('Example 6: Multi-Page Analysis');

  const pages = [
    'https://example.com',
    'https://example.com/about',
    'https://example.com/products',
    'https://example.com/contact'
  ];

  const insights: string[] = [];

  for (const url of pages) {
    await invoke('navigate_to_url', { url });

    const response: ClaudeResponse = await invoke('ask_claude_about_page', {
      prompt: 'In one sentence, what is the main purpose of this page?',
      model: 'sonnet'
    });

    insights.push(`${url}: ${response.text}`);
  }

  console.log('Page insights:', insights);

  // Now ask Claude to summarize the whole site
  const summary: ClaudeResponse = await invoke('ask_claude_about_page', {
    prompt: `Based on these page descriptions, summarize the overall purpose and structure of this website:

    ${insights.join('\n')}`,
    model: 'sonnet'
  });

  console.log('Site summary:', summary.text);
}

/**
 * Example 7: Interactive automation
 * Use Claude to guide automation decisions
 */
async function example7_interactiveAutomation() {
  console.log('Example 7: Interactive Automation');

  await invoke('navigate_to_url', { url: 'https://example.com/search' });

  // Step 1: Ask Claude to find the search input
  const findInput: ClaudeResponse = await invoke('ask_claude_about_page', {
    prompt: 'What is the CSS selector for the main search input field? Return ONLY the selector, nothing else.',
    model: 'sonnet'
  });

  const selector = findInput.text.trim();
  console.log('Search input selector:', selector);

  // Step 2: Use the selector to interact with the page
  // (Note: You would need to implement JS execution in ChromeDriver)
  // await invoke('execute_script', {
  //   script: `document.querySelector('${selector}').value = 'test query'`
  // });

  // Step 3: Ask Claude what to do next
  const nextStep: ClaudeResponse = await invoke('ask_claude_about_page', {
    prompt: 'I just filled in the search field. What should I do next to submit the search? Return the CSS selector of the submit button.',
    model: 'sonnet'
  });

  console.log('Next step:', nextStep.text);
}

/**
 * Example 8: Screenshot-only analysis (no HTML)
 * Useful for visual testing or when HTML is too large
 */
async function example8_screenshotOnly() {
  console.log('Example 8: Screenshot-Only Analysis');

  await invoke('navigate_to_url', { url: 'https://example.com' });

  const screenshot = await invoke('take_screenshot', {
    outputPath: '/tmp/visual-test.png'
  });

  const response: ClaudeResponse = await invoke('ask_claude', {
    request: {
      prompt: `Compare this screenshot to the expected design:

      Expected: Blue header, white background, centered logo, 3-column layout

      Does the actual page match? Report any visual differences.`,
      screenshot_path: screenshot,
      include_html: false,
      model: 'sonnet'
    }
  });

  console.log('Visual test result:', response.text);
}

/**
 * Example 9: Error detection and recovery
 * Use Claude to identify and suggest fixes for errors
 */
async function example9_errorDetection() {
  console.log('Example 9: Error Detection');

  try {
    await invoke('navigate_to_url', { url: 'https://example.com/broken-page' });
  } catch (e) {
    console.log('Navigation failed, continuing anyway...');
  }

  const response: ClaudeResponse = await invoke('ask_claude_about_page', {
    prompt: `Does this page show any error messages? If so:
    1. What is the error?
    2. What likely caused it?
    3. How can it be fixed?

    Return in JSON format:
    {
      "hasError": true/false,
      "errorMessage": "...",
      "likelyCause": "...",
      "suggestedFix": "..."
    }`,
    model: 'sonnet'
  });

  console.log('Error analysis:', response.text);
}

/**
 * Example 10: Comparative analysis
 * Compare two different pages or states
 */
async function example10_comparativeAnalysis() {
  console.log('Example 10: Comparative Analysis');

  // Capture first state
  await invoke('navigate_to_url', { url: 'https://example.com' });
  const before = await invoke('take_screenshot', {
    outputPath: '/tmp/before.png'
  });

  // Make some change (e.g., click a button, would need JS execution)
  // await invoke('execute_script', { script: "document.querySelector('button').click()" });

  // Wait a bit for changes
  await new Promise(resolve => setTimeout(resolve, 2000));

  // Capture second state
  const after = await invoke('take_screenshot', {
    outputPath: '/tmp/after.png'
  });

  // Ask Claude to compare (note: would need multi-image support)
  const response: ClaudeResponse = await invoke('ask_claude', {
    request: {
      prompt: 'Describe what changed on this page after the interaction.',
      screenshot_path: after,
      include_html: true,
      model: 'sonnet'
    }
  });

  console.log('Changes detected:', response.text);
}

// Export all examples
export const examples = {
  example1_simpleAnalysis,
  example2_dataExtraction,
  example3_formAnalysis,
  example4_customScreenshot,
  example5_accessibilityAudit,
  example6_multiPageAnalysis,
  example7_interactiveAutomation,
  example8_screenshotOnly,
  example9_errorDetection,
  example10_comparativeAnalysis,
};

// Run all examples (for testing)
export async function runAllExamples() {
  for (const [name, example] of Object.entries(examples)) {
    console.log(`\n${'='.repeat(60)}`);
    console.log(`Running: ${name}`);
    console.log('='.repeat(60));

    try {
      await example();
      console.log('✓ Success');
    } catch (error) {
      console.error('✗ Error:', error);
    }
  }
}

// For direct execution
if (typeof window !== 'undefined') {
  // Running in Tauri app
  console.log('Claude Integration Examples loaded');
  console.log('Available examples:', Object.keys(examples));
  console.log('Run any example: await examples.example1_simpleAnalysis()');
  console.log('Run all: await runAllExamples()');
}
