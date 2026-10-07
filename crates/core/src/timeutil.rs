//! Minimal UTC timestamp helpers (unix milliseconds <-> ISO 8601) without a date crate.

use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Days since 1970-01-01 for a proleptic Gregorian date (Howard Hinnant's algorithm).
pub fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

/// Formats as `YYYY-MM-DDTHH:MM:SS.mmmZ`.
pub fn to_iso(ms: i64) -> String {
    let days = ms.div_euclid(86_400_000);
    let rem = ms.rem_euclid(86_400_000);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}.{:03}Z",
        rem / 3_600_000,
        rem / 60_000 % 60,
        rem / 1000 % 60,
        rem % 1000
    )
}

/// Parses `YYYY-MM-DD[THH:MM[:SS[.fff…]]][Z|±HH:MM]` into unix milliseconds.
pub fn parse_iso(s: &str) -> Option<i64> {
    let s = s.trim();
    let num = |a: usize, b: usize| -> Option<i64> {
        let part = s.get(a..b)?;
        part.bytes()
            .all(|c| c.is_ascii_digit())
            .then(|| part.parse().ok())?
    };
    let (y, m, d) = (num(0, 4)?, num(5, 7)?, num(8, 10)?);
    if s.get(4..5)? != "-"
        || s.get(7..8)? != "-"
        || !(1..=12).contains(&m)
        || !(1..=31).contains(&d)
    {
        return None;
    }
    let mut ms = days_from_civil(y, m as u32, d as u32) * 86_400_000;
    let rest = &s[10..];
    if rest.is_empty() {
        return Some(ms);
    }
    let rest = rest.strip_prefix('T').or_else(|| rest.strip_prefix(' '))?;
    let time_end = rest.find(['Z', '+', '-']).unwrap_or(rest.len());
    let (time, zone) = rest.split_at(time_end);
    let mut parts = time.splitn(3, ':');
    let h: i64 = parts.next()?.parse().ok()?;
    let mi: i64 = parts.next()?.parse().ok()?;
    let sec: f64 = parts.next().map(|p| p.parse().ok()).unwrap_or(Some(0.0))?;
    if h > 23 || mi > 59 || !(0.0..61.0).contains(&sec) {
        return None;
    }
    ms += h * 3_600_000 + mi * 60_000 + (sec * 1000.0).round() as i64;
    match zone {
        "" | "Z" => Some(ms),
        z => {
            let sign = if z.starts_with('-') { -1 } else { 1 };
            let z = &z[1..];
            let (zh, zm) = z
                .split_once(':')
                .unwrap_or((z.get(..2)?, z.get(2..).unwrap_or("0")));
            let offset = zh.parse::<i64>().ok()? * 3_600_000 + zm.parse::<i64>().ok()? * 60_000;
            Some(ms - sign * offset)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iso_roundtrip() {
        for ms in [
            0,
            1_632_074_170_609,
            951_782_400_000,
            -86_400_000,
            4_102_444_799_999,
        ] {
            assert_eq!(parse_iso(&to_iso(ms)), Some(ms), "{}", to_iso(ms));
        }
        assert_eq!(to_iso(1_632_074_170_609), "2021-09-19T17:56:10.609Z");
        assert!(now_ms() > 1_700_000_000_000);
    }

    #[test]
    fn parses_variants() {
        let base = parse_iso("2026-07-04T12:39:04Z").unwrap();
        assert_eq!(parse_iso("2026-07-04T12:39:04.972594Z"), Some(base + 973));
        assert_eq!(parse_iso("2026-07-04 12:39:04"), Some(base));
        assert_eq!(parse_iso("2026-07-04T14:39:04+02:00"), Some(base));
        assert_eq!(parse_iso("2026-07-04T07:39:04-0500"), Some(base));
        assert_eq!(parse_iso("2026-07-04T12:39"), Some(base - 4000));
        assert_eq!(
            parse_iso("2026-07-04"),
            Some(base - (12 * 3600 + 39 * 60 + 4) * 1000)
        );
        for bad in [
            "",
            "2026/07/04",
            "2026-13-01",
            "2026-07-04X",
            "2026-07-04T25:00",
            "20a6-01-01",
            "2026-07-04T1:xx",
            "2026-07-04T10:00+zz",
        ] {
            assert_eq!(parse_iso(bad), None, "{bad}");
        }
    }
}
