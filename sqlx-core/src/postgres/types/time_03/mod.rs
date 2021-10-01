mod date;
mod datetime;
mod time;

#[rustfmt::skip]
const PG_EPOCH: ::time_03::Date = ::time_03::macros::date!(2000-1-1);
