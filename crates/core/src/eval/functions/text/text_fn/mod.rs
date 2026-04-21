use chrono::Datelike;
use crate::display::display_number;
use crate::eval::coercion::to_string_val;
use crate::eval::functions::check_arity;
use crate::eval::functions::date::serial::serial_to_date;
use crate::types::Value;

/// Format an integer part with thousands separators.
fn format_with_commas(int_part: u64) -> String {
    let s = int_part.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut result = String::with_capacity(len + len / 3);
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(b as char);
    }
    result
}

/// Apply a format string to a number value, returning the formatted string.
fn apply_format(n: f64, fmt: &str) -> String {
    // ── Percentage format: ends with '%' ─────────────────────────────────────
    if let Some(pct_fmt) = fmt.strip_suffix('%') {
        let pct_val = n * 100.0;
        return format!("{}%", apply_format(pct_val, pct_fmt));
    }

    // ── Time format: contains time tokens (h, hh, ss, AM/PM) ─────────────────
    {
        let lower = fmt.to_lowercase();
        let is_time_fmt = lower.contains("hh") || lower.contains("ss")
            || lower.contains("am/pm") || lower.contains("a/p")
            || (lower.contains('h') && !lower.contains("yyyy") && !lower.contains("yy"));
        if is_time_fmt {
            // n is a fraction of a day; 0.5 = noon
            let total_secs = (n.fract().abs() * 86400.0).round() as u64;
            let hours = total_secs / 3600;
            let minutes = (total_secs % 3600) / 60;
            let seconds = total_secs % 60;

            let use_ampm = lower.contains("am/pm") || lower.contains("a/p");
            let (display_hours, ampm_str) = if use_ampm {
                let h12 = if hours == 0 { 12 } else if hours > 12 { hours - 12 } else { hours };
                let ap = if hours < 12 { "AM" } else { "PM" };
                (h12, ap)
            } else {
                (hours, "")
            };

            // Replace tokens in order: longest first to avoid partial replacements
            let mut out = fmt.to_string();
            // Remove AM/PM or A/P token first
            out = out.replace("AM/PM", ampm_str);
            out = out.replace("am/pm", ampm_str);
            out = out.replace("A/P", ampm_str);
            out = out.replace("a/p", ampm_str);
            // Replace hh before h
            out = out.replace("hh", &format!("{:02}", display_hours));
            out = out.replace('h', &display_hours.to_string());
            // Replace ss
            out = out.replace("ss", &format!("{:02}", seconds));
            // Replace mm (minutes in time context)
            out = out.replace("mm", &format!("{:02}", minutes));
            return out.trim().to_string();
        }
    }

    // ── Date format: contains date tokens ────────────────────────────────────
    {
        let lower = fmt.to_lowercase();
        if lower.contains("yyyy") || lower.contains("yy")
            || lower.contains("mm") || lower.contains("dd")
        {
            if let Some(date) = serial_to_date(n) {
                // Work on a lowercase copy so replacements are case-insensitive,
                // then return the normalised (lowercase) result.
                let mut out = lower;
                // Replace in order from longest to shortest to avoid partial replacements
                out = out.replace("yyyy", &format!("{:04}", date.year()));
                out = out.replace("yy", &format!("{:02}", date.year() % 100));
                out = out.replace("mm", &format!("{:02}", date.month()));
                out = out.replace("dd", &format!("{:02}", date.day()));
                return out;
            }
        }
    }

    // ── Scientific notation: e.g. "0.00E+00" ────────────────────────────────
    // Detect only when format has digit tokens before E and sign+digits after.
    {
        // Find 'E' or 'e' that is preceded by a digit token and followed by '+'/'-'
        let sci_pos = fmt.char_indices().find(|&(i, c)| {
            if c != 'E' && c != 'e' {
                return false;
            }
            let before = &fmt[..i];
            let after = &fmt[i + 1..];
            // before must contain at least one '0' or '#'
            let has_digit_token = before.contains('0') || before.contains('#');
            // after must start with '+' or '-'
            let has_sign = after.starts_with('+') || after.starts_with('-');
            has_digit_token && has_sign
        });

        if let Some((e_pos, _)) = sci_pos {
            // Count decimal places before E (e.g. "0.00E+00" → 2)
            let before_e = &fmt[..e_pos];
            let decimal_places = if let Some(dot_pos) = before_e.find('.') {
                before_e[dot_pos + 1..].len()
            } else {
                0
            };
            // Count exponent digits after E+/- sign
            let after_e = &fmt[e_pos + 1..];
            let exp_digits = after_e.trim_start_matches(['+', '-']).len();
            let exp_digits = exp_digits.max(2);

            // Compute mantissa and exponent
            let abs_n = n.abs();
            let (mantissa, exponent) = if abs_n == 0.0 {
                (0.0_f64, 0_i32)
            } else {
                let exp = abs_n.log10().floor() as i32;
                (abs_n / 10f64.powi(exp), exp)
            };
            let sign = if n < 0.0 { "-" } else { "" };
            let exp_sign = if exponent < 0 { "-" } else { "+" };
            return format!(
                "{}{:.prec$}E{}{:0>width$}",
                sign,
                mantissa,
                exp_sign,
                exponent.unsigned_abs(),
                prec = decimal_places,
                width = exp_digits,
            );
        }
    }

    // ── Fraction format: contains '/' with digit denominator ────────────────
    // e.g. "0/4" means "whole numerator/4"
    if let Some(slash_pos) = fmt.find('/') {
        let denom_str = fmt[slash_pos + 1..].trim();
        if let Ok(denom) = denom_str.parse::<u64>() {
            if denom > 0 {
                // Round to nearest multiple of 1/denom
                let numerator = (n * denom as f64).round() as u64;
                if numerator == 0 {
                    return "0".to_string();
                }
                let gcd_val = gcd(numerator, denom);
                let num = numerator / gcd_val;
                let den = denom / gcd_val;
                if den == 1 {
                    return format!("{}", num);
                }
                return format!("{}/{}", num, den);
            }
        }
    }

    // ── Comma + decimal format: e.g. "$#,##0.00", "#,##0.00", "#,##0" ───────
    let has_comma = fmt.contains(',');
    let negative = n < 0.0;
    let abs_n = n.abs();

    if has_comma {
        // Extract any currency prefix (characters before the first '#' or '0')
        let prefix: String = fmt.chars().take_while(|c| *c != '#' && *c != '0').collect();

        if let Some(dot_pos) = fmt.find('.') {
            let decimal_part = &fmt[dot_pos + 1..];
            if decimal_part.chars().all(|c| c == '0' || c == '#') {
                let places = decimal_part.len();
                let scale = 10f64.powi(places as i32);
                let rounded = (abs_n * scale).round() / scale;
                let int_part = rounded as u64;
                let frac = rounded - int_part as f64;
                let frac_digits = (frac * scale).round() as u64;
                let int_str = format_with_commas(int_part);
                let result = format!("{}{}.{:0>width$}", prefix, int_str, frac_digits, width = places);
                return if negative { format!("-{}", result) } else { result };
            }
        } else {
            // No decimal point — just comma grouping
            let int_part = abs_n.round() as u64;
            let result = format!("{}{}", prefix, format_with_commas(int_part));
            return if negative { format!("-{}", result) } else { result };
        }
    }

    // ── Simple decimal-only format: "0.00", "#.##" etc. ──────────────────────
    if let Some(dot_pos) = fmt.find('.') {
        let decimal_part = &fmt[dot_pos + 1..];
        if decimal_part.chars().all(|c| c == '0' || c == '#') {
            let places = decimal_part.len();
            return format!("{:.prec$}", n, prec = places);
        }
    } else if fmt.chars().all(|c| c == '0' || c == '#') {
        return format!("{:.0}", n);
    }

    // ── Fallback ─────────────────────────────────────────────────────────────
    display_number(n)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// `TEXT(value, format_text)` — converts a number to a formatted string.
pub fn text_fn(args: &[Value]) -> Value {
    if let Some(err) = check_arity(args, 2, 2) {
        return err;
    }
    // Preserve the original value for date detection
    let raw = args[0].clone();
    let is_date = matches!(raw, Value::Date(_));
    let n = match &raw {
        Value::Date(d) => *d,
        Value::Number(n) => *n,
        other => match crate::eval::coercion::to_number(other.clone()) {
            Ok(n) => n,
            Err(e) => return e,
        },
    };
    let format = match to_string_val(args[1].clone()) {
        Ok(s) => s,
        Err(e) => return e,
    };
    // For non-date values, strip date format tokens to avoid misdetection
    // (a plain number should not be formatted as a date unless the format
    // contains date tokens AND the value came from a date function)
    let _ = is_date; // currently unused; serial_to_date handles any f64
    Value::Text(apply_format(n, &format))
}

#[cfg(test)]
mod tests;
