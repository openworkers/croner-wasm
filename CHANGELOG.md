# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-11-21

### Added

- `hasSeconds(): boolean` - New method to detect if pattern uses seconds (6-field format)
- `seconds` option - Control seconds field handling (important for OpenWorkers):
  - Available in constructor: `new WasmCron(pattern, { seconds: '...' })`
  - Available in validate: `WasmCron.validate(pattern, { seconds: '...' })`
  - Options:
    - `'optional'` (default) - Accept both 5 and 6-field patterns
    - `'required'` - Only accept 6-field patterns
    - `'disallowed'` - Only accept 5-field patterns

### Changed - BREAKING CHANGES

**API simplified to match croner JavaScript library:**

- Constructor now accepts optional second parameter: `new WasmCron(pattern, { timezone: 'UTC' })`
- `nextRun(from?)` - Optional date parameter instead of separate `nextRunFrom()`
- `nextRuns(count, from?)` - Optional date parameter instead of separate `nextRunsFrom()`

**Migration guide from 0.1.0:**

```javascript
// Before (0.1.0)
const cron = new WasmCron('0 * * * *');
const next = cron.nextRunFrom(someDate);
const runs = cron.nextRunsFrom(5, someDate);

// After (0.2.0)
const cron = new WasmCron('0 * * * *', { timezone: 'UTC' });
const next = cron.nextRun(someDate);  // or cron.nextRun() for now
const runs = cron.nextRuns(5, someDate);  // or cron.nextRuns(5) for now
```

### Fixed

- Removed `Symbol.dispose` from TypeScript definitions for better compatibility with older TS versions

## [0.1.0] - 2025-11-21

### Added

- Initial release
- WebAssembly bindings for croner cron parser
- Support for 5 and 6-field cron expressions
- Quartz extensions (L, W, #)
- Human-readable descriptions
- Next occurrence calculation
- Multi-platform support (Node.js, browser, bundlers)
