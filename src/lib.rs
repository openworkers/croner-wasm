use chrono::{DateTime, Utc};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// A WASM wrapper for the croner cron expression parser
#[wasm_bindgen]
pub struct WasmCron {
    inner: croner::Cron,
}

#[wasm_bindgen]
impl WasmCron {
    /// Parse a cron expression and return a WasmCron instance.
    /// Returns an error if the pattern is invalid.
    ///
    /// Supports standard 5-field and extended 6-field (with seconds) formats.
    #[wasm_bindgen(constructor)]
    pub fn new(pattern: &str) -> Result<WasmCron, JsValue> {
        let cron = croner::Cron::from_str(pattern)
            .map_err(|e| JsValue::from_str(&format!("Invalid cron pattern: {:?}", e)))?;

        Ok(WasmCron { inner: cron })
    }

    /// Validate a cron pattern without creating an instance
    /// Returns true if valid, false otherwise
    #[wasm_bindgen]
    pub fn validate(pattern: &str) -> bool {
        croner::Cron::from_str(pattern).is_ok()
    }

    /// Get a human-readable description of the cron pattern
    /// Example: "At 00:00 on Friday"
    #[wasm_bindgen]
    pub fn describe(&self) -> String {
        self.inner.describe()
    }

    /// Get the pattern string that was parsed
    #[wasm_bindgen]
    pub fn pattern(&self) -> String {
        self.inner.pattern.to_string()
    }

    /// Get the next occurrence from now
    /// Returns a JavaScript Date object or null if no next occurrence
    #[wasm_bindgen(js_name = nextRun)]
    pub fn next_run(&self) -> Option<js_sys::Date> {
        let now = Utc::now();
        self.inner
            .find_next_occurrence(&now, false)
            .ok()
            .map(|dt| js_sys::Date::new(&JsValue::from_f64(dt.timestamp_millis() as f64)))
    }

    /// Get the next occurrence from a specific date
    /// Returns a JavaScript Date object or null if no next occurrence
    #[wasm_bindgen(js_name = nextRunFrom)]
    pub fn next_run_from(&self, from: &js_sys::Date) -> Option<js_sys::Date> {
        let timestamp_ms = from.get_time() as i64;
        let from_dt = DateTime::from_timestamp_millis(timestamp_ms)
            .unwrap_or_else(Utc::now);

        self.inner
            .find_next_occurrence(&from_dt, false)
            .ok()
            .map(|dt| js_sys::Date::new(&JsValue::from_f64(dt.timestamp_millis() as f64)))
    }

    /// Get multiple next occurrences
    /// Returns an array of JavaScript Date objects
    #[wasm_bindgen(js_name = nextRuns)]
    pub fn next_runs(&self, count: usize) -> js_sys::Array {
        self.next_runs_from(count, &js_sys::Date::new_0())
    }

    /// Get multiple next occurrences from a specific date
    /// Returns an array of JavaScript Date objects
    #[wasm_bindgen(js_name = nextRunsFrom)]
    pub fn next_runs_from(&self, count: usize, from: &js_sys::Date) -> js_sys::Array {
        let timestamp_ms = from.get_time() as i64;
        let mut current = DateTime::from_timestamp_millis(timestamp_ms)
            .unwrap_or_else(Utc::now);

        let results = js_sys::Array::new();

        for _ in 0..count {
            match self.inner.find_next_occurrence(&current, false) {
                Ok(next) => {
                    let js_date =
                        js_sys::Date::new(&JsValue::from_f64(next.timestamp_millis() as f64));
                    results.push(&js_date);
                    current = next;
                }
                Err(_) => break,
            }
        }

        results
    }

    /// Check if a specific date matches the cron pattern
    #[wasm_bindgen(js_name = isMatch)]
    pub fn is_match(&self, date: &js_sys::Date) -> bool {
        let timestamp_ms = date.get_time() as i64;
        if let Some(dt) = DateTime::from_timestamp_millis(timestamp_ms) {
            self.inner.is_time_matching(&dt).unwrap_or(false)
        } else {
            false
        }
    }
}

/// Parse and describe a cron pattern in one call
/// Returns an object with pattern and description, or throws an error
#[wasm_bindgen(js_name = parseAndDescribe)]
pub fn parse_and_describe(pattern: &str) -> Result<JsValue, JsValue> {
    let cron = WasmCron::new(pattern)?;
    let description = cron.describe();

    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"pattern".into(), &pattern.into())?;
    js_sys::Reflect::set(&obj, &"description".into(), &description.into())?;

    Ok(obj.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        assert!(WasmCron::validate("0 * * * *"));
        assert!(WasmCron::validate("*/5 * * * *"));
        assert!(WasmCron::validate("0 0 * * FRI"));
        assert!(!WasmCron::validate("invalid pattern"));
    }

    // Note: Tests that use wasm_bindgen features (like Result<T, JsValue>)
    // need to be run with wasm-pack test instead of cargo test
}
