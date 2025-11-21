use chrono::{DateTime, Utc};
use croner::parser::{CronParser, Seconds};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// A WASM wrapper for the croner cron expression parser
#[wasm_bindgen]
pub struct WasmCron {
    inner: croner::Cron,
    has_seconds: bool,
}

#[wasm_bindgen]
impl WasmCron {
    /// Create a new WasmCron instance from a pattern string.
    ///
    /// # Arguments
    /// * `pattern` - Cron expression (5 or 6 fields)
    /// * `options` - Optional configuration object with:
    ///   - `timezone?: string` - Timezone (reserved for future use)
    ///   - `seconds?: 'optional' | 'required' | 'disallowed'` - How to handle seconds field
    ///
    /// # Example
    /// ```javascript
    /// // Optional seconds (default)
    /// const cron1 = new WasmCron('0 * * * *');
    ///
    /// // Disallow seconds (force 5-field format)
    /// const cron2 = new WasmCron('0 * * * *', { seconds: 'disallowed' });
    ///
    /// // Require seconds (force 6-field format)
    /// const cron3 = new WasmCron('0 * * * * *', { seconds: 'required' });
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(pattern: &str, options: Option<js_sys::Object>) -> Result<WasmCron, JsValue> {
        // Parse seconds option from options object
        let seconds_policy = if let Some(opts) = options {
            if let Ok(seconds_value) = js_sys::Reflect::get(&opts, &"seconds".into()) {
                if !seconds_value.is_undefined() {
                    let seconds_str = seconds_value
                        .as_string()
                        .ok_or_else(|| JsValue::from_str("'seconds' option must be a string"))?;

                    match seconds_str.as_str() {
                        "optional" => Seconds::Optional,
                        "required" => Seconds::Required,
                        "disallowed" => Seconds::Disallowed,
                        _ => return Err(JsValue::from_str(
                            "'seconds' option must be 'optional', 'required', or 'disallowed'"
                        )),
                    }
                } else {
                    Seconds::Optional
                }
            } else {
                Seconds::Optional
            }
        } else {
            Seconds::Optional
        };

        // Build parser with seconds policy
        let parser = CronParser::builder()
            .seconds(seconds_policy)
            .build();

        let cron = parser
            .parse(pattern)
            .map_err(|e| JsValue::from_str(&format!("Invalid cron pattern: {:?}", e)))?;

        // Detect if pattern has seconds (6 fields) by counting non-empty parts
        let has_seconds = pattern.split_whitespace().count() == 6;

        Ok(WasmCron {
            inner: cron,
            has_seconds,
        })
    }

    /// Validate a cron pattern without creating an instance.
    /// Returns true if valid, false otherwise.
    ///
    /// # Arguments
    /// * `pattern` - Cron expression to validate
    /// * `options` - Optional configuration with `seconds` policy
    ///
    /// # Example
    /// ```javascript
    /// WasmCron.validate('0 * * * *');  // true
    /// WasmCron.validate('0 * * * *', { seconds: 'disallowed' });  // true
    /// WasmCron.validate('0 * * * * *', { seconds: 'disallowed' });  // false
    /// ```
    #[wasm_bindgen]
    pub fn validate(pattern: &str, options: Option<js_sys::Object>) -> bool {
        // Parse seconds option from options object
        let seconds_policy = if let Some(opts) = options {
            if let Ok(seconds_value) = js_sys::Reflect::get(&opts, &"seconds".into()) {
                if !seconds_value.is_undefined() {
                    if let Some(seconds_str) = seconds_value.as_string() {
                        match seconds_str.as_str() {
                            "optional" => Seconds::Optional,
                            "required" => Seconds::Required,
                            "disallowed" => Seconds::Disallowed,
                            _ => return false, // Invalid option = invalid pattern
                        }
                    } else {
                        return false; // Non-string option = invalid
                    }
                } else {
                    Seconds::Optional
                }
            } else {
                Seconds::Optional
            }
        } else {
            Seconds::Optional
        };

        // Build parser with seconds policy
        let parser = CronParser::builder()
            .seconds(seconds_policy)
            .build();

        parser.parse(pattern).is_ok()
    }

    /// Get a human-readable description of the cron pattern.
    #[wasm_bindgen]
    pub fn describe(&self) -> String {
        self.inner.describe()
    }

    /// Get the pattern string that was parsed.
    #[wasm_bindgen]
    pub fn pattern(&self) -> String {
        self.inner.pattern.to_string()
    }

    /// Check if the pattern uses seconds (6-field format).
    /// Returns true if the pattern has 6 fields (includes seconds), false for 5 fields.
    ///
    /// # Example
    /// ```javascript
    /// const cron5 = new WasmCron('0 * * * *');
    /// cron5.hasSeconds(); // false
    ///
    /// const cron6 = new WasmCron('0/30 * * * * *');
    /// cron6.hasSeconds(); // true
    /// ```
    #[wasm_bindgen(js_name = hasSeconds)]
    pub fn has_seconds(&self) -> bool {
        self.has_seconds
    }

    /// Get the next occurrence.
    ///
    /// # Arguments
    /// * `from` - Optional date to start from (defaults to now)
    ///
    /// # Example
    /// ```javascript
    /// cron.nextRun()                  // From now
    /// cron.nextRun(new Date())        // From specific date
    /// ```
    #[wasm_bindgen(js_name = nextRun)]
    pub fn next_run(&self, from: Option<js_sys::Date>) -> Option<js_sys::Date> {
        let start = if let Some(date) = from {
            let timestamp_ms = date.get_time() as i64;
            DateTime::from_timestamp_millis(timestamp_ms).unwrap_or_else(Utc::now)
        } else {
            Utc::now()
        };

        self.inner
            .find_next_occurrence(&start, false)
            .ok()
            .map(|dt| js_sys::Date::new(&JsValue::from_f64(dt.timestamp_millis() as f64)))
    }

    /// Get multiple next occurrences.
    ///
    /// # Arguments
    /// * `count` - Number of occurrences to return
    /// * `from` - Optional date to start from (defaults to now)
    ///
    /// # Example
    /// ```javascript
    /// cron.nextRuns(5)                    // 5 runs from now
    /// cron.nextRuns(5, new Date())        // 5 runs from specific date
    /// ```
    #[wasm_bindgen(js_name = nextRuns)]
    pub fn next_runs(&self, count: usize, from: Option<js_sys::Date>) -> js_sys::Array {
        let mut current = if let Some(date) = from {
            let timestamp_ms = date.get_time() as i64;
            DateTime::from_timestamp_millis(timestamp_ms).unwrap_or_else(Utc::now)
        } else {
            Utc::now()
        };

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

    /// Check if a specific date matches the cron pattern.
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

/// Parse and describe a cron pattern in one call.
/// Returns an object with pattern and description, or throws an error.
#[wasm_bindgen(js_name = parseAndDescribe)]
pub fn parse_and_describe(pattern: &str) -> Result<JsValue, JsValue> {
    let cron = WasmCron::new(pattern, None)?;
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
