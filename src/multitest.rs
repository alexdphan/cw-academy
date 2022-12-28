// used to make unit tests more compact and very expressive in terms of test scenarios they implement

pub mod contract; // import the contract module
#[cfg(test)] // only compile the tests module if we are running tests, builds only on our test run
mod tests; // import the tests module