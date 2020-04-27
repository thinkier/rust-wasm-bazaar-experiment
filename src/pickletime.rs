use web_sys::Document;

use crate::scheduler::Interval;

pub fn hook(doc: &Document) {
	if let (Some(date_elem), Some(time_elem)) = (doc.get_element_by_id("skyblock-date"), doc.get_element_by_id("skyblock-time")) {
		let mut last_dt = SkyblockDateTime::default();
		Interval::new(20000 / 24, move || {
			let dt = SkyblockDateTime::now();

			let date_str = dt.date();
			if last_dt.date() != date_str {
				date_elem.set_inner_html(&format!("Year {}, {}", dt.year, date_str));
			}

			time_elem.set_inner_html(&dt.time());

			last_dt = dt;
		}).run_now().make_permanent();
	} else {
		console_error!("failed to get the date and time elements, widget not hooked");
	}
}

#[derive(Default)]
pub struct SkyblockDateTime {
	pub year: i32,
	pub month: i32,
	pub day: i32,
	pub hour: i32,
	pub minute: i32,
}

impl SkyblockDateTime {
	pub fn now() -> SkyblockDateTime {
		let unix_secs = js_sys::Date::now() / 1000.;
		let time = unix_secs - 1559792100.;

		let year = (time / 12. / 31. / 1200.).floor() as i32;
		let month = (time / 31. / 1200.).floor() as i32 % 12;
		let day = (time / 1200.).floor() as i32 % 31 + 1;
		let hour = ((time / 1200.) * 24.).floor() as i32 % 24;
		let minute = ((time / 1200.) * 24. * 60.).floor() as i32 % 60;

		SkyblockDateTime {
			year,
			month,
			day,
			hour,
			minute,
		}
	}

	pub fn date(&self) -> String {
		format!("{} {}{}", month_string(self.month), self.day, num_suffixes(self.day))
	}

	pub fn time(&self) -> String {
		let mut hour = self.hour % 12;
		if hour == 0 {
			hour = 12;
		}

		let mut meridiem = 'a';
		if self.hour >= 12 {
			meridiem = 'p';
		}

		format!("{}:{:0>2}{}m", hour, self.minute, meridiem)
	}
}

fn month_string(month: i32) -> &'static str {
	match month {
		1 => "Early Spring",
		2 => "Spring",
		3 => "Late Spring",
		4 => "Early Summer",
		5 => "Summer",
		6 => "Late Summer",
		7 => "Early Autumn",
		8 => "Autumn",
		9 => "Late Autumn",
		10 => "Early Winter",
		11 => "Winter",
		_ => "Late Winter",
	}
}

fn num_suffixes(num: i32) -> &'static str {
	let teens = (num % 100 - 10) < 10;

	match (teens, num % 10) {
		(false, 1) => "st",
		(false, 2) => "nd",
		(false, 3) => "rd",
		_ => "th"
	}
}
