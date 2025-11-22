use chrono::{DateTime, Utc};
use croner::parser::{CronParser, Seconds, Year};
use wasm_bindgen::prelude::*;

/// Parse the seconds option from a JS options object
fn parse_seconds_option(options: Option<js_sys::Object>) -> Result<Seconds, JsValue> {
    let Some(opts) = options else {
        return Ok(Seconds::Optional);
    };

    let Ok(seconds_value) = js_sys::Reflect::get(&opts, &"seconds".into()) else {
        return Ok(Seconds::Optional);
    };

    if seconds_value.is_undefined() {
        return Ok(Seconds::Optional);
    }

    let seconds_str = seconds_value
        .as_string()
        .ok_or_else(|| js_sys::Error::new("'seconds' option must be a string"))?;

    match seconds_str.as_str() {
        "optional" => Ok(Seconds::Optional),
        "required" => Ok(Seconds::Required),
        "disallowed" => Ok(Seconds::Disallowed),
        _ => Err(js_sys::Error::new(
            "'seconds' option must be 'optional', 'required', or 'disallowed'",
        ).into()),
    }
}

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
        let seconds_policy = parse_seconds_option(options)?;

        // Build parser with seconds policy and year disabled
        let parser = CronParser::builder()
            .seconds(seconds_policy)
            .year(Year::Disallowed)
            .build();

        let cron = parser
            .parse(pattern)
            .map_err(|e| js_sys::Error::new(&format!("Invalid cron pattern: {:?}", e)))?;

        // Detect if pattern has seconds (6 fields) using the parsed pattern from the library
        // Year is disabled, so we can only have 5 fields (no seconds) or 6 fields (with seconds)
        let has_seconds = cron.pattern.to_string().split_whitespace().count() == 6;

        Ok(WasmCron {
            inner: cron,
            has_seconds,
        })
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
    // Note: Tests that use wasm_bindgen features (like Result<T, JsValue>)
    // need to be run with wasm-pack test instead of cargo test
}
