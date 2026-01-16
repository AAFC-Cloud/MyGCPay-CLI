//! <https://pipsc.ca/member-tools/downloadable-calendar>
//! <https://www.canada.ca/en/treasury-board-secretariat/topics/pay/collective-agreements/it.html#toc45543245549> Article 12: designated paid holidays

use jiff::civil::Date;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PipscCalendarDateType {
    Holiday,
    PayDate,
    CivicHoliday,
    NationalQuebecHoliday,
}

macro_rules! pipsc_date_annot {
    // Accept entries like: 2025 - 01 - 01 => Holiday, PayDate; 2026 - 02 - 02 => PayDate;
    ( $( $y:tt - $m:tt - $d:tt => $($kind:ident),* );* $(;)? ) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            let mut set = ::std::collections::HashSet::new();
            $( set.insert(PipscCalendarDateType::$kind); )*
            map.insert(::jiff::civil::Date::constant($y, $m, $d), set);
        )*
        map
    }};
}

pub static PIPSC_CALENDAR_DAY_ANNOTATIONS: LazyLock<HashMap<Date, HashSet<PipscCalendarDateType>>> =
    LazyLock::new(|| {
        pipsc_date_annot! {
            // 2024
            2024 - 01 - 01 => Holiday;
            2024 - 01 - 03 => PayDate;
            2024 - 01 - 17 => PayDate;
            2024 - 01 - 31 => PayDate;

            2024 - 02 - 14 => PayDate;
            2024 - 02 - 28 => PayDate;

            2024 - 03 - 13 => PayDate;
            2024 - 03 - 27 => PayDate;
            2024 - 03 - 29 => Holiday;

            2024 - 04 - 01 => Holiday;
            2024 - 04 - 10 => PayDate;
            2024 - 04 - 24 => PayDate;

            2024 - 05 - 08 => PayDate;
            2024 - 05 - 20 => Holiday;
            2024 - 05 - 22 => PayDate;

            2024 - 06 - 05 => PayDate;
            2024 - 06 - 19 => PayDate;
            2024 - 06 - 24 => NationalQuebecHoliday;

            2024 - 07 - 01 => Holiday;
            2024 - 07 - 03 => PayDate;
            2024 - 07 - 17 => PayDate;
            2024 - 07 - 31 => PayDate;

            2024 - 08 - 05 => CivicHoliday;
            2024 - 08 - 14 => PayDate;
            2024 - 08 - 28 => PayDate;

            2024 - 09 - 02 => Holiday;
            2024 - 09 - 11 => PayDate;
            2024 - 09 - 25 => PayDate;
            2024 - 09 - 30 => Holiday;

            2024 - 10 - 09 => PayDate;
            2024 - 10 - 14 => Holiday;
            2024 - 10 - 23 => PayDate;

            2024 - 11 - 06 => PayDate;
            2024 - 11 - 11 => Holiday;
            2024 - 11 - 20 => PayDate;

            2024 - 12 - 04 => PayDate;
            2024 - 12 - 18 => PayDate;
            2024 - 12 - 25 => Holiday;
            2024 - 12 - 26 => Holiday;

            // 2025
            2025 - 01 - 01 => PayDate, Holiday;
            2025 - 01 - 15 => PayDate;
            2025 - 01 - 29 => PayDate;

            2025 - 02 - 12 => PayDate;
            2025 - 02 - 26 => PayDate;

            2025 - 03 - 12 => PayDate;
            2025 - 03 - 26 => PayDate;

            2025 - 04 - 09 => PayDate;
            2025 - 04 - 18 => Holiday;
            2025 - 04 - 21 => Holiday;
            2025 - 04 - 23 => PayDate;

            2025 - 05 - 07 => PayDate;
            2025 - 05 - 19 => Holiday;
            2025 - 05 - 21 => PayDate;

            2025 - 06 - 04 => PayDate;
            2025 - 06 - 18 => PayDate;
            2025 - 06 - 24 => NationalQuebecHoliday;

            2025 - 07 - 01 => Holiday;
            2025 - 07 - 02 => PayDate;
            2025 - 07 - 16 => PayDate;
            2025 - 07 - 30 => PayDate;

            2025 - 08 - 04 => CivicHoliday;
            2025 - 08 - 13 => PayDate;
            2025 - 08 - 27 => PayDate;

            2025 - 09 - 01 => Holiday;
            2025 - 09 - 10 => PayDate;
            2025 - 09 - 24 => PayDate;
            2025 - 09 - 30 => Holiday;

            2025 - 10 - 08 => PayDate;
            2025 - 10 - 13 => Holiday;
            2025 - 10 - 22 => PayDate;

            2025 - 11 - 05 => PayDate;
            2025 - 11 - 11 => Holiday;
            2025 - 11 - 19 => PayDate;

            2025 - 12 - 03 => PayDate;
            2025 - 12 - 17 => PayDate;
            2025 - 12 - 25 => Holiday;
            2025 - 12 - 26 => Holiday;
            2025 - 12 - 31 => PayDate;

            // 2026
            2026 - 01 - 01 => Holiday;
            2026 - 01 - 14 => PayDate;
            2026 - 01 - 28 => PayDate;

            2026 - 02 - 11 => PayDate;
            2026 - 02 - 25 => PayDate;

            2026 - 03 - 11 => PayDate;
            2026 - 03 - 25 => PayDate;

            2026 - 04 - 03 => Holiday;
            2026 - 04 - 06 => Holiday;
            2026 - 04 - 08 => PayDate;
            2026 - 04 - 22 => PayDate;

            2026 - 05 - 06 => PayDate;
            2026 - 05 - 18 => Holiday;
            2026 - 05 - 20 => PayDate;

            2026 - 06 - 03 => PayDate;
            2026 - 06 - 17 => PayDate;
            2026 - 06 - 24 => NationalQuebecHoliday;

            2026 - 07 - 01 => PayDate, Holiday;
            2026 - 07 - 15 => PayDate;
            2026 - 07 - 29 => PayDate;

            2026 - 08 - 03 => CivicHoliday;
            2026 - 08 - 12 => PayDate;
            2026 - 08 - 26 => PayDate;

            2026 - 09 - 07 => Holiday;
            2026 - 09 - 09 => PayDate;
            2026 - 09 - 23 => PayDate;
            2026 - 09 - 30 => Holiday;

            2026 - 10 - 07 => PayDate;
            2026 - 10 - 12 => Holiday;
            2026 - 10 - 21 => PayDate;

            2026 - 11 - 04 => PayDate;
            2026 - 11 - 11 => Holiday;
            2026 - 11 - 18 => PayDate;

            2026 - 12 - 02 => PayDate;
            2026 - 12 - 16 => PayDate;
            2026 - 12 - 25 => Holiday;
            2026 - 12 - 28 => Holiday;
            2026 - 12 - 30 => PayDate;

            // 2027
            2027 - 01 - 01 => Holiday;
            2027 - 01 - 13 => PayDate;
            2027 - 01 - 27 => PayDate;

            2027 - 02 - 10 => PayDate;
            2027 - 02 - 24 => PayDate;

            2027 - 03 - 10 => PayDate;
            2027 - 03 - 24 => PayDate;
            2027 - 03 - 26 => Holiday;
            2027 - 03 - 29 => Holiday;

            2027 - 04 - 07 => PayDate;
            2027 - 04 - 21 => PayDate;

            2027 - 05 - 05 => PayDate;
            2027 - 05 - 19 => PayDate;
            2027 - 05 - 24 => Holiday;

            2027 - 06 - 02 => PayDate;
            2027 - 06 - 16 => PayDate;
            2027 - 06 - 24 => NationalQuebecHoliday;
            2027 - 06 - 30 => PayDate;

            2027 - 07 - 01 => Holiday;
            2027 - 07 - 14 => PayDate;
            2027 - 07 - 28 => PayDate;

            2027 - 08 - 02 => CivicHoliday;
            2027 - 08 - 11 => PayDate;
            2027 - 08 - 25 => PayDate;

            2027 - 09 - 06 => Holiday;
            2027 - 09 - 08 => PayDate;
            2027 - 09 - 22 => PayDate;
            2027 - 09 - 30 => Holiday;

            2027 - 10 - 06 => PayDate;
            2027 - 10 - 11 => Holiday;
            2027 - 10 - 20 => PayDate;

            2027 - 11 - 03 => PayDate;
            2027 - 11 - 11 => Holiday;
            2027 - 11 - 17 => PayDate;

            2027 - 12 - 01 => PayDate;
            2027 - 12 - 15 => PayDate;
            2027 - 12 - 27 => Holiday;
            2027 - 12 - 28 => Holiday;
            2027 - 12 - 29 => PayDate;
        }
    });
