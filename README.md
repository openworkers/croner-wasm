# croner-wasm

WebAssembly bindings for the [croner](https://github.com/Hexagon/croner-rust) cron expression parser.

## Features

- ✅ Parse and validate cron expressions
- ✅ Get human-readable descriptions
- ✅ Calculate next occurrences
- ✅ Support for seconds (6-field format)
- ✅ Quartz scheduler extensions (L, W, #)
- ✅ Zero dependencies (pure WASM)
- ✅ Works in Node.js, browsers, and bundlers

## Installation

```bash
npm install @openworkers/croner-wasm
# or
bun add @openworkers/croner-wasm
```

## Usage

### Node.js

```javascript
import { WasmCron } from '@openworkers/croner-wasm';

// Parse a cron expression
const cron = new WasmCron('0 * * * *');

// Get human-readable description
console.log(cron.describe());
// => "At minute 0 past every hour."

// Get next run time
const next = cron.nextRun();
console.log(next);
// => Date object

// Get multiple next runs
const nextRuns = cron.nextRuns(5);
console.log(nextRuns);
// => Array of 5 Date objects
```

### Browser (ES Modules)

```javascript
import init, { WasmCron } from '@openworkers/croner-wasm';

// Initialize WASM module (required in browser)
await init();

// Parse a cron expression
const cron = new WasmCron('0 * * * *');
console.log(cron.describe());
```

### Validation

```javascript
import { WasmCron } from '@openworkers/croner-wasm';

// Validate without creating an instance
if (WasmCron.validate('0 * * * *')) {
  console.log('Valid cron pattern!');
}

// Try-catch for parsing
try {
  const cron = new WasmCron('invalid pattern');
} catch (error) {
  console.error('Invalid cron:', error);
}
```

### Parse and Describe

```javascript
import { parseAndDescribe } from '@openworkers/croner-wasm';

const result = parseAndDescribe('0 0 * * FRI');
console.log(result);
// => { pattern: "0 0 * * FRI", description: "At 00:00 on Friday" }
```

### Advanced Usage

```javascript
const cron = new WasmCron('*/5 * * * *');

// Get pattern
console.log(cron.pattern());
// => "*/5 * * * *"

// Get next run from specific date
const from = new Date('2024-01-01T00:00:00Z');
const next = cron.nextRunFrom(from);

// Get multiple next runs from specific date
const nextRuns = cron.nextRunsFrom(10, from);

// Check if a date matches the pattern
const matches = cron.isMatch(new Date());
```

## Supported Cron Formats

**5-field format** (minute hour day month weekday):
```
0 * * * *        # Every hour
*/5 * * * *      # Every 5 minutes
0 0 * * FRI      # Every Friday at midnight
0 9-17 * * MON-FRI  # Every hour from 9-5, Mon-Fri
```

**6-field format** (second minute hour day month weekday):
```
0/30 * * * * *   # Every 30 seconds
*/5 * * * * *    # Every 5 seconds
0 0 * * * *      # Every hour (at 0 seconds)
```

**Quartz extensions**:
```
0 0 L * *        # Last day of the month
0 0 15W * *      # Nearest weekday to the 15th
0 0 * * 5L       # Last Friday of the month
0 0 * * 5#3      # Third Friday of the month
```

## API Reference

### `WasmCron` Class

#### Constructor
- `new WasmCron(pattern: string)` - Parse a cron expression (throws on invalid)

#### Static Methods
- `WasmCron.validate(pattern: string): boolean` - Validate a pattern

#### Instance Methods
- `describe(): string` - Get human-readable description
- `pattern(): string` - Get the original pattern
- `nextRun(): Date | null` - Get next occurrence from now
- `nextRunFrom(date: Date): Date | null` - Get next occurrence from date
- `nextRuns(count: number): Date[]` - Get N next occurrences from now
- `nextRunsFrom(count: number, date: Date): Date[]` - Get N next occurrences from date
- `isMatch(date: Date): boolean` - Check if date matches pattern

### Functions

- `parseAndDescribe(pattern: string): { pattern: string, description: string }` - Parse and describe in one call

## Package Exports

This package supports multiple environments:

```json
{
  "exports": {
    ".": {
      "node": "./dist/node/croner_wasm.js",
      "browser": "./dist/croner_wasm.js",
      "default": "./dist/bundler/croner_wasm.js"
    }
  }
}
```

- **Node.js**: Uses CommonJS build automatically
- **Browser**: Uses ES modules with WASM
- **Bundlers** (webpack, rollup, vite): Uses optimized build

## Building from Source

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))
- wasm-pack: `cargo install wasm-pack`

### Build Commands

```bash
# Build all targets (Node.js, browser, bundlers)
bun run build:all

# Or build individually
bun run build           # Web only
bun run build:node      # Node.js only
bun run build:bundler   # Bundlers only

# Run tests
bun run test            # Rust tests
bun run test:wasm       # WASM tests in browser
```

## File Size

- WASM binary: ~119 KB (optimized)
- JS glue code: ~12 KB
- Total: ~131 KB

## License

MIT

## Credits

Built on top of [croner](https://github.com/Hexagon/croner-rust) by Hexagon.
