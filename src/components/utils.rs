use rust_i18n::t;

use super::InterpolationType;

#[cfg(feature = "numbers")]
pub(super) fn f64_to_fd(value: f64) -> fixed_decimal::FixedDecimal {
    fixed_decimal::FixedDecimal::try_from_f64(value, fixed_decimal::FloatPrecision::Floating)
        .expect(format!("Failed to parse FixedDecimal from f64: {}", value).as_str())
}

#[cfg(feature = "numbers")]
pub(super) fn resolve_locale(locale: &String, label: impl ToString) -> icu_locid::Locale {
    locale
        .parse()
        .expect(format!("Invalid locale: {} for key: {}", locale, label.to_string()).as_str())
}

#[cfg(feature = "numbers")]
pub(super) fn get_formatter(
    locale: &String,
    label: impl ToString,
) -> icu_decimal::FixedDecimalFormatter {
    let label_string = label.to_string();
    let locale = resolve_locale(locale, label);
    let locale_string = locale.to_string();
    icu_decimal::FixedDecimalFormatter::try_new(&locale.into(), Default::default()).expect(
        format!(
            "Failed to create FixedDecimalFormatter for number: {} with locale: {}",
            label_string, locale_string,
        )
        .as_str(),
    )
}

pub(super) fn translate_by_key(
    locale: &String,
    key: &String,
    args: &Vec<(String, InterpolationType)>,
) -> String {
    let key = key.as_str();

    #[cfg(feature = "numbers")]
    let fdf = super::utils::get_formatter(locale, key);

    let (patterns, values): (Vec<&str>, Vec<String>) = args
        .iter()
        .map(|(k, interpolation_type)| {
            let value = match interpolation_type {
                InterpolationType::String(v) => v.clone(),
                #[cfg(feature = "numbers")]
                InterpolationType::Number(v) => fdf.format_to_string(v),
            };
            (k.as_str(), value)
        })
        .unzip();
    let translated = t!(key, locale = locale);

    let val = rust_i18n::replace_patterns(&translated, patterns.as_slice(), values.as_slice());
    val
}
