mod date;
mod datetime;
mod time;

#[rustfmt::skip]
const PG_EPOCH: time_02::Date = time_02::internals::Date::from_yo_unchecked(2000, 1);
