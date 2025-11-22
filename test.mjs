// Test suite for croner-wasm
// Run with: node test.mjs

import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import pkg from './pkg-node/croner_wasm.js';

const { WasmCron, parseAndDescribe } = pkg;

describe('WasmCron', () => {
  describe('constructor', () => {
    it('should parse valid patterns', () => {
      assert.ok(new WasmCron('0 * * * *'));
      assert.ok(new WasmCron('*/5 * * * *'));
      assert.ok(new WasmCron('0 0 * * FRI'));
      assert.ok(new WasmCron('0 9-17 * * MON-FRI'));
    });

    it('should parse 6-field patterns with seconds', () => {
      assert.ok(new WasmCron('0/30 * * * * *'));
      assert.ok(new WasmCron('*/5 * * * * *'));
      assert.ok(new WasmCron('0 0 * * * *'));
    });

    it('should parse Quartz extensions', () => {
      assert.ok(new WasmCron('0 0 L * *')); // Last day of month
      assert.ok(new WasmCron('0 0 15W * *')); // Nearest weekday to 15th
      assert.ok(new WasmCron('0 0 * * 5L')); // Last Friday
      assert.ok(new WasmCron('0 0 * * 5#3')); // 3rd Friday
    });

    it('should throw on invalid patterns', () => {
      assert.throws(() => new WasmCron('invalid'));
      assert.throws(() => new WasmCron('99 * * * *'));
      assert.throws(() => new WasmCron(''));
      assert.throws(() => new WasmCron('* * * *')); // Too few fields
    });

    it('should throw Error objects with proper properties', () => {
      try {
        new WasmCron('invalid pattern');
        assert.fail('Should have thrown');
      } catch (e) {
        assert.ok(e instanceof Error);
        assert.ok(e.message.length > 0);
        assert.ok(e.stack);
      }
    });

    describe('seconds option', () => {
      it('should accept optional seconds by default', () => {
        const cron5 = new WasmCron('0 * * * *');
        assert.ok(cron5);

        const cron6 = new WasmCron('0 * * * * *');
        assert.ok(cron6);
      });

      it('should accept optional seconds explicitly', () => {
        const cron5 = new WasmCron('0 * * * *', { seconds: 'optional' });
        assert.ok(cron5);

        const cron6 = new WasmCron('0 * * * * *', { seconds: 'optional' });
        assert.ok(cron6);
      });

      it('should require seconds when seconds: "required"', () => {
        // 6-field pattern should work
        const cron6 = new WasmCron('0 * * * * *', { seconds: 'required' });
        assert.ok(cron6);

        // 5-field pattern should fail
        assert.throws(() => {
          new WasmCron('0 * * * *', { seconds: 'required' });
        });
      });

      it('should disallow seconds when seconds: "disallowed"', () => {
        // 5-field pattern should work
        const cron5 = new WasmCron('0 * * * *', { seconds: 'disallowed' });
        assert.ok(cron5);

        // 6-field pattern should fail
        assert.throws(() => {
          new WasmCron('0 * * * * *', { seconds: 'disallowed' });
        });
      });

      it('should throw on invalid seconds option', () => {
        assert.throws(() => {
          new WasmCron('0 * * * *', { seconds: 'invalid' });
        }, /must be 'optional', 'required', or 'disallowed'/);
      });

      it('should throw on non-string seconds option', () => {
        assert.throws(() => {
          new WasmCron('0 * * * *', { seconds: 123 });
        }, /'seconds' option must be a string/);
      });
    });
  });

  describe('pattern()', () => {
    it('should return the original pattern', () => {
      const pattern = '0 * * * *';
      const cron = new WasmCron(pattern);
      assert.equal(cron.pattern(), pattern);
    });
  });

  describe('hasSeconds()', () => {
    it('should return false for 5-field patterns', () => {
      const patterns = [
        '0 * * * *',
        '*/5 * * * *',
        '0 0 * * FRI',
        '0 9-17 * * MON-FRI',
      ];

      patterns.forEach(pattern => {
        const cron = new WasmCron(pattern);
        assert.equal(cron.hasSeconds(), false, `Pattern "${pattern}" should not have seconds`);
      });
    });

    it('should return true for 6-field patterns', () => {
      const patterns = [
        '0 * * * * *',
        '0/30 * * * * *',
        '*/5 * * * * *',
        '0 0 0 * * *',
      ];

      patterns.forEach(pattern => {
        const cron = new WasmCron(pattern);
        assert.equal(cron.hasSeconds(), true, `Pattern "${pattern}" should have seconds`);
      });
    });
  });

  describe('describe()', () => {
    it('should return human-readable description', () => {
      const cron = new WasmCron('0 * * * *');
      const desc = cron.describe();
      assert.ok(typeof desc === 'string');
      assert.ok(desc.length > 0);
    });

    it('should describe various patterns correctly', () => {
      const patterns = [
        ['0 * * * *', 'minute'],
        ['0 0 * * *', '00:00'],
        ['0 0 * * FRI', 'Friday'],
        ['*/5 * * * *', 'every'],
      ];

      patterns.forEach(([pattern, expectedWord]) => {
        const cron = new WasmCron(pattern);
        const desc = cron.describe().toLowerCase();
        assert.ok(desc.includes(expectedWord.toLowerCase()),
          `Description "${desc}" should contain "${expectedWord}"`);
      });
    });
  });

  describe('nextRun()', () => {
    it('should return next occurrence from now', () => {
      const cron = new WasmCron('0 * * * *'); // Every hour
      const next = cron.nextRun();

      assert.ok(next instanceof Date);
      assert.ok(next > new Date());
    });

    it('should return next occurrence from specific date', () => {
      const cron = new WasmCron('0 0 * * *'); // Midnight
      const from = new Date('2024-06-15T10:00:00Z');
      const next = cron.nextRun(from);

      assert.ok(next instanceof Date);
      assert.ok(next > from);
      assert.equal(next.getUTCHours(), 0);
      assert.equal(next.getUTCMinutes(), 0);
    });

    it('should handle patterns with seconds', () => {
      const cron = new WasmCron('0/30 * * * * *'); // Every 30 seconds
      const next = cron.nextRun();

      assert.ok(next instanceof Date);
      assert.ok([0, 30].includes(next.getUTCSeconds()));
    });
  });

  describe('nextRuns()', () => {
    it('should return multiple next occurrences', () => {
      const cron = new WasmCron('0 * * * *'); // Every hour
      const runs = cron.nextRuns(5);

      assert.ok(Array.isArray(runs));
      assert.equal(runs.length, 5);

      // Check all are Date objects
      runs.forEach(date => assert.ok(date instanceof Date));

      // Check they are in ascending order
      for (let i = 1; i < runs.length; i++) {
        assert.ok(runs[i] > runs[i-1]);
      }
    });

    it('should return next occurrences from specific date', () => {
      const cron = new WasmCron('0 0 * * *'); // Midnight
      const from = new Date('2024-01-01T00:00:00Z');
      const runs = cron.nextRuns(3, from);

      assert.equal(runs.length, 3);

      // All should be at midnight
      runs.forEach(date => {
        assert.equal(date.getUTCHours(), 0);
        assert.equal(date.getUTCMinutes(), 0);
      });
    });

    it('should handle edge case of requesting 0 runs', () => {
      const cron = new WasmCron('0 * * * *');
      const runs = cron.nextRuns(0);
      assert.equal(runs.length, 0);
    });
  });

  describe('isMatch()', () => {
    it('should match dates that match the pattern', () => {
      const cron = new WasmCron('0 0 * * *'); // Midnight
      const midnight = new Date('2024-06-15T00:00:00Z');

      assert.ok(cron.isMatch(midnight));
    });

    it('should not match dates that do not match the pattern', () => {
      const cron = new WasmCron('0 0 * * *'); // Midnight
      const notMidnight = new Date('2024-06-15T10:00:00Z');

      assert.ok(!cron.isMatch(notMidnight));
    });

    it('should match with minute precision', () => {
      const cron = new WasmCron('30 10 * * *'); // 10:30
      const matching = new Date('2024-06-15T10:30:00Z');
      const notMatching = new Date('2024-06-15T10:31:00Z');

      assert.ok(cron.isMatch(matching));
      assert.ok(!cron.isMatch(notMatching));
    });

    it('should match specific weekdays', () => {
      const cron = new WasmCron('0 0 * * FRI'); // Midnight on Fridays
      const friday = new Date('2024-06-14T00:00:00Z'); // Known Friday
      const saturday = new Date('2024-06-15T00:00:00Z'); // Known Saturday

      assert.ok(cron.isMatch(friday));
      assert.ok(!cron.isMatch(saturday));
    });
  });

  describe('parseAndDescribe()', () => {
    it('should parse and describe in one call', () => {
      const result = parseAndDescribe('0 * * * *');

      assert.ok(result);
      assert.equal(result.pattern, '0 * * * *');
      assert.ok(typeof result.description === 'string');
      assert.ok(result.description.length > 0);
    });

    it('should throw on invalid patterns', () => {
      assert.throws(() => parseAndDescribe('invalid'));
    });
  });

  describe('Complex patterns', () => {
    it('should handle ranges', () => {
      const cron = new WasmCron('0 9-17 * * MON-FRI');
      const desc = cron.describe().toLowerCase();

      assert.ok(desc.includes('monday') || desc.includes('mon'));
      assert.ok(desc.includes('friday') || desc.includes('fri'));
    });

    it('should handle steps', () => {
      const cron = new WasmCron('*/15 * * * *'); // Every 15 minutes
      const now = new Date('2024-06-15T10:00:00Z');
      const next = cron.nextRun(now);

      assert.ok([0, 15, 30, 45].includes(next.getUTCMinutes()));
    });

    it('should handle specific values', () => {
      const cron = new WasmCron('0 0,12 * * *'); // Midnight and noon
      const from = new Date('2024-06-15T00:00:00Z');
      const runs = cron.nextRuns(4, from);

      runs.forEach(date => {
        assert.ok([0, 12].includes(date.getUTCHours()));
      });
    });

    it('should handle last day of month', () => {
      const cron = new WasmCron('0 0 L * *');
      const desc = cron.describe().toLowerCase();

      assert.ok(desc.includes('last'));
    });

    it('should handle nth weekday of month', () => {
      const cron = new WasmCron('0 0 * * 5#3'); // 3rd Friday
      const desc = cron.describe().toLowerCase();

      assert.ok(desc.includes('3'));
      assert.ok(desc.includes('friday') || desc.includes('fri'));
    });
  });
});

console.log('\n✅ All tests completed!');
