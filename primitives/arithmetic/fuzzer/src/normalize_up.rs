// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//! # Running
//! Running this fuzzer can be done with `cargo hfuzz run normalize_up`. `honggfuzz` CLI options can
//! be used by setting `HFUZZ_RUN_ARGS`, such as `-n 4` to use 4 threads.
//!
//! # Debugging a panic
//! Once a panic is found, it can be debugged with
//! `cargo hfuzz run-debug normanormalize_uplize hfuzz_workspace/normalize_up/*.fuzz`.

use honggfuzz::fuzz;
use sp_arithmetic::Normalizable;
use std::convert::TryInto;

type Ty = u64;

fn main() {
	let len_limit: usize = Ty::max_value().try_into().unwrap();

	fuzz!(|data: (Vec<Ty>, Ty)| {
		let (data, norm) = data;
		if data.len() == 0 { return; }
		let pre_sum: u128 = data.iter().map(|x| *x as u128).sum();

		let normalized = data.normalize_up(norm);

		// error cases.
		let sum_limit = norm;
		if pre_sum > sum_limit.into() || data.len() > len_limit {
			// noop
			assert_eq!(normalized, data);
		} else {
			// if sum goes beyond u128, panic.
			let sum: u128 = normalized.iter().map(|x| *x as u128).sum();

			assert_eq!(
				sum,
				norm as u128,
				"sums don't match {:?}, {}",
				normalized,
				norm,
			);
		}
	})
}
