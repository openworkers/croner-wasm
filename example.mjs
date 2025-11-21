// Node.js example for croner-wasm
// Run with: node example.mjs

import pkg from './pkg-node/croner_wasm.js';
const { WasmCron, parseAndDescribe } = pkg;

console.log('=== Croner WASM Node.js Example ===\n');

// Example 1: Validate patterns
console.log('1. Validation:');
const patterns = [
    '0 * * * *',
    '*/5 * * * *',
    '0 0 * * FRI',
    'invalid pattern'
];

patterns.forEach(pattern => {
    const isValid = WasmCron.validate(pattern);
    console.log(`  "${pattern}" -> ${isValid ? '✓ Valid' : '✗ Invalid'}`);
});

// Example 1b: Validation with seconds option
console.log('\n1b. Validation with Seconds Option:');
console.log(`  "0 * * * *" (disallowed): ${WasmCron.validate('0 * * * *', { seconds: 'disallowed' }) ? '✓' : '✗'}`);
console.log(`  "0 * * * * *" (disallowed): ${WasmCron.validate('0 * * * * *', { seconds: 'disallowed' }) ? '✓' : '✗'}`);
console.log(`  "0 * * * *" (required): ${WasmCron.validate('0 * * * *', { seconds: 'required' }) ? '✓' : '✗'}`);
console.log(`  "0 * * * * *" (required): ${WasmCron.validate('0 * * * * *', { seconds: 'required' }) ? '✓' : '✗'}`);

// Example 2: Parse and describe with options
console.log('\n2. Parse and Describe:');
try {
    const cron = new WasmCron('0 9 * * MON-FRI', { timezone: 'UTC' });
    console.log(`  Pattern: ${cron.pattern()}`);
    console.log(`  Has seconds: ${cron.hasSeconds()}`);
    console.log(`  Description: ${cron.describe()}`);
} catch (error) {
    console.error(`  Error: ${error}`);
}

// Example 2b: Pattern with seconds
console.log('\n2b. Pattern with Seconds:');
try {
    const cron = new WasmCron('0/30 * * * * *');
    console.log(`  Pattern: ${cron.pattern()}`);
    console.log(`  Has seconds: ${cron.hasSeconds()}`);
    console.log(`  Description: ${cron.describe()}`);
} catch (error) {
    console.error(`  Error: ${error}`);
}

// Example 2c: Seconds policy
console.log('\n2c. Seconds Policy:');
try {
    // Disallow seconds - only 5-field patterns
    const cron5 = new WasmCron('0 * * * *', { seconds: 'disallowed' });
    console.log(`  ✓ 5-field with disallowed: ${cron5.pattern()}`);

    try {
        new WasmCron('0 * * * * *', { seconds: 'disallowed' });
    } catch (e) {
        console.log(`  ✗ 6-field with disallowed: Error (expected)`);
    }

    // Require seconds - only 6-field patterns
    const cron6 = new WasmCron('0 * * * * *', { seconds: 'required' });
    console.log(`  ✓ 6-field with required: ${cron6.pattern()}`);

    try {
        new WasmCron('0 * * * *', { seconds: 'required' });
    } catch (e) {
        console.log(`  ✗ 5-field with required: Error (expected)`);
    }
} catch (error) {
    console.error(`  Error: ${error}`);
}

// Example 3: Get next occurrences
console.log('\n3. Next Occurrences:');
try {
    const cron = new WasmCron('0 * * * *');
    console.log(`  Pattern: "${cron.pattern()}"`);
    console.log(`  Description: ${cron.describe()}`);

    // No argument = from now
    const next = cron.nextRun();
    console.log(`  Next run: ${next ? next.toISOString() : 'None'}`);

    // Optional date argument
    const specificDate = new Date('2024-06-15T10:30:00Z');
    const nextFromDate = cron.nextRun(specificDate);
    console.log(`  Next run from ${specificDate.toISOString()}: ${nextFromDate?.toISOString()}`);

    const nextRuns = cron.nextRuns(5);
    console.log(`  Next 5 runs:`);
    nextRuns.forEach((date, index) => {
        console.log(`    ${index + 1}. ${date.toISOString()}`);
    });
} catch (error) {
    console.error(`  Error: ${error}`);
}

// Example 4: Check if date matches
console.log('\n4. Check Date Matching:');
try {
    const cron = new WasmCron('0 0 * * *'); // Midnight every day
    const midnight = new Date();
    midnight.setHours(0, 0, 0, 0);

    const matches = cron.isMatch(midnight);
    console.log(`  Does ${midnight.toISOString()} match "0 0 * * *"? ${matches ? 'Yes' : 'No'}`);
} catch (error) {
    console.error(`  Error: ${error}`);
}

// Example 5: Parse and describe helper
console.log('\n5. Parse and Describe Helper:');
try {
    const result = parseAndDescribe('0/30 * * * * *');
    console.log(`  Pattern: ${result.pattern}`);
    console.log(`  Description: ${result.description}`);
} catch (error) {
    console.error(`  Error: ${error}`);
}

// Example 6: Advanced patterns with Quartz extensions
console.log('\n6. Advanced Patterns (Quartz Extensions):');
const advancedPatterns = [
    ['0 0 L * *', 'Last day of month'],
    ['0 0 15W * *', 'Nearest weekday to 15th'],
    ['0 0 * * 5L', 'Last Friday of month'],
    ['0 0 * * 5#3', 'Third Friday of month']
];

advancedPatterns.forEach(([pattern, expected]) => {
    try {
        const cron = new WasmCron(pattern);
        console.log(`  ${pattern}: ${cron.describe()}`);
    } catch (error) {
        console.log(`  ${pattern}: Error - ${error}`);
    }
});

console.log('\n=== Done! ===');
