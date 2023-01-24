// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `frame_benchmarking::baseline`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=frame_benchmarking::baseline
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/frame_benchmarking_baseline.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_benchmarking::baseline`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_benchmarking::baseline::WeightInfo for WeightInfo<T> {
	/// The range of component `i` is `[0, 1000000]`.
	fn addition(_i: u32, ) -> Weight {
		// Minimum execution time: 113 nanoseconds.
		Weight::from_ref_time(144_272)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn subtraction(_i: u32, ) -> Weight {
		// Minimum execution time: 113 nanoseconds.
		Weight::from_ref_time(149_024)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn multiplication(_i: u32, ) -> Weight {
		// Minimum execution time: 113 nanoseconds.
		Weight::from_ref_time(149_977)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn division(_i: u32, ) -> Weight {
		// Minimum execution time: 111 nanoseconds.
		Weight::from_ref_time(143_401)
	}
	/// The range of component `i` is `[0, 100]`.
	fn hashing(i: u32, ) -> Weight {
		// Minimum execution time: 24_005_194 nanoseconds.
		Weight::from_ref_time(24_049_452_700)
			// Standard Error: 88_954
			.saturating_add(Weight::from_ref_time(71_380).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[0, 100]`.
	fn sr25519_verification(i: u32, ) -> Weight {
		// Minimum execution time: 154 nanoseconds.
		Weight::from_ref_time(1_295_545)
			// Standard Error: 6_905
			.saturating_add(Weight::from_ref_time(55_595_381).saturating_mul(i.into()))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_read(i: u32, ) -> Weight {
		// Minimum execution time: 122 nanoseconds.
		Weight::from_ref_time(132_000)
			// Standard Error: 6_486
			.saturating_add(Weight::from_ref_time(2_458_058).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_write(i: u32, ) -> Weight {
		// Minimum execution time: 126 nanoseconds.
		Weight::from_ref_time(136_000)
			// Standard Error: 1_202
			.saturating_add(Weight::from_ref_time(388_630).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
}
