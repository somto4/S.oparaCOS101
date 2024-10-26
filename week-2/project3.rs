fn  main () {
	let principal: f64 = 210_000.0;
	let rate: f64 = 5.0;
	let years: f64 = 3.0;

	let amount = principal * (1.0 - rate / 100.0).powf(years);

		let compound_interest = amount - principal;

		println!("the compund compound_interest after {} years is:₦{:.2}", years, compound_interest);
		println!("The total amount after {} years is :₦{:.2}", years, amount);


}