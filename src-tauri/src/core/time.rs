use time::format_description::well_known::Rfc3339;

pub fn now_utc_rfc3339() -> String {
    time::OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .expect("rfc3339 formatting should succeed")
}
