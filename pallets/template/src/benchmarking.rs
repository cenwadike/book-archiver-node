//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet;
use frame_benchmarking::v2::*;
use frame_support::inherent::Vec;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn archive_book() {
		// method params
		let title: Vec<u8> = "title".into();
		let author: Vec<u8> = "author".into();
		let url: Vec<u8> = "url".into();

		// signed origin
		let caller: T::AccountId = whitelisted_caller();

		// archive book entrisic call
		#[extrinsic_call]
		archive_book(RawOrigin::Signed(caller), title, author, url);
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
