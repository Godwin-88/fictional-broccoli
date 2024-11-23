use std::f64::consts::PI;

/// Cumulative distribution function for the standard normal distribution
fn cdf(x: f64) -> f64 {
    (1.0 + (x / (2.0_f64).sqrt()).erf()) / 2.0
}

/// Standard normal probability density function
fn pdf(x: f64) -> f64 {
    (-x * x / 2.0).exp() / (2.0 * PI).sqrt()
}

/// Calculates d1 in the Black-Scholes formula
fn d1(spot: f64, strike: f64, time: f64, rate: f64, volatility: f64) -> f64 {
    ((spot / strike).ln() + (rate + 0.5 * volatility * volatility) * time) / (volatility * time.sqrt())
}

/// Calculates d2 in the Black-Scholes formula
fn d2(d1: f64, volatility: f64, time: f64) -> f64 {
    d1 - volatility * time.sqrt()
}

/// Calculates the price of a European call option using the Black-Scholes formula
pub fn call_price(spot: f64, strike: f64, time: f64, rate: f64, volatility: f64) -> f64 {
    let d1 = d1(spot, strike, time, rate, volatility);
    let d2 = d2(d1, volatility, time);
    spot * cdf(d1) - strike * (-rate * time).exp() * cdf(d2)
}

/// Calculates the price of a European put option using the Black-Scholes formula
pub fn put_price(spot: f64, strike: f64, time: f64, rate: f64, volatility: f64) -> f64 {
    let d1 = d1(spot, strike, time, rate, volatility);
    let d2 = d2(d1, volatility, time);
    strike * (-rate * time).exp() * cdf(-d2) - spot * cdf(-d1)
}




/*
Explanation:

Cumulative Distribution Function (cdf): Calculates the cumulative distribution function for the standard normal distribution, which is essential for the Black-Scholes formula.

Probability Density Function (pdf): Computes the standard normal probability density function, used in calculating option Greeks.

d1 and d2 Functions: Compute intermediary variables d1 and d2 used in the Black-Scholes formula.

call_price Function: Calculates the price of a European call option based on the input parameters:

spot: Current price of the underlying asset.
strike: Strike price of the option.
time: Time to expiration in years.
rate: Risk-free interest rate (annualized).
volatility: Volatility of the underlying asset (annualized).
put_price Function: Calculates the price of a European put option using similar parameters as call_price.
*/
