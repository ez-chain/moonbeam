// Copyright 2019-2021 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

use crate::mock::{
	evm_test_context, precompile_address, ExtBuilder, Precompiles, TestAccount::Alice,
};
use crate::test_relay_runtime::TestEncoder;
use crate::StakeEncodeCall;
use crate::*;
use crate::{AvailableStakeCalls, PrecompileOutput};
use pallet_evm::{ExitError, ExitSucceed, PrecompileSet};
use pallet_staking::RewardDestination;
use pallet_staking::ValidatorPrefs;
use precompile_utils::{Bytes, EvmDataWriter};
use sp_core::{H256, U256};
use sp_runtime::Perbill;

#[test]
fn selector_less_than_four_bytes() {
	ExtBuilder::default().build().execute_with(|| {
		// This selector is only three bytes long when four are required.
		let bogus_selector = vec![1u8, 2u8, 3u8];

		// Expected result is an error stating there are too few bytes
		let expected_result = Some(Err(ExitError::Other(
			"tried to parse selector out of bounds".into(),
		)));

		assert_eq!(
			Precompiles::execute(
				precompile_address(),
				&bogus_selector,
				None,
				&evm_test_context(),
			),
			expected_result
		);
	});
}

#[test]
fn no_selector_exists_but_length_is_right() {
	ExtBuilder::default().build().execute_with(|| {
		let bogus_selector = vec![1u8, 2u8, 3u8, 4u8];

		// Expected result is an error stating there are such a selector does not exist
		let expected_result = Some(Err(ExitError::Other("unknown selector".into())));

		assert_eq!(
			Precompiles::execute(
				precompile_address(),
				&bogus_selector,
				None,
				&evm_test_context(),
			),
			expected_result
		);
	});
}

#[test]
fn test_encode_bond() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000)])
		.build()
		.execute_with(|| {
			let controller_address: H256 = [1u8; 32].into();
			let amount: U256 = 100u32.into();
			let reward_dest = 2u8;
			let reward_dest_address: H256 = [0u8; 32].into();

			let input_data = EvmDataWriter::new()
				.write_selector(Action::EncodeBond)
				.write(controller_address)
				.write(amount)
				.write(reward_dest)
				.write(reward_dest_address)
				.build();

			let expected_bytes: Bytes = TestEncoder::encode_call(AvailableStakeCalls::Bond(
				[1u8; 32].into(),
				100u32.into(),
				RewardDestination::Controller,
			))
			.as_slice()
			.into();

			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: EvmDataWriter::new().write(expected_bytes).build(),
				cost: Default::default(),
				logs: Default::default(),
			}));

			assert_eq!(
				Precompiles::execute(precompile_address(), &input_data, None, &evm_test_context()),
				expected_result
			);
		});
}

#[test]
fn test_encode_bond_more() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000)])
		.build()
		.execute_with(|| {
			let amount: U256 = 100u32.into();
			let input_data = EvmDataWriter::new()
				.write_selector(Action::EncodeBondExtra)
				.write(amount)
				.build();

			let expected_bytes: Bytes =
				TestEncoder::encode_call(AvailableStakeCalls::BondExtra(100u32.into()))
					.as_slice()
					.into();

			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: EvmDataWriter::new().write(expected_bytes).build(),
				cost: Default::default(),
				logs: Default::default(),
			}));

			assert_eq!(
				Precompiles::execute(precompile_address(), &input_data, None, &evm_test_context()),
				expected_result
			);
		});
}

#[test]
fn test_encode_chill() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000)])
		.build()
		.execute_with(|| {
			let input_data = EvmDataWriter::new()
				.write_selector(Action::EncodeChill)
				.build();

			let expected_bytes: Bytes = TestEncoder::encode_call(AvailableStakeCalls::Chill)
				.as_slice()
				.into();

			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: EvmDataWriter::new().write(expected_bytes).build(),
				cost: Default::default(),
				logs: Default::default(),
			}));

			assert_eq!(
				Precompiles::execute(precompile_address(), &input_data, None, &evm_test_context()),
				expected_result
			);
		});
}

#[test]
fn test_encode_nominate() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000)])
		.build()
		.execute_with(|| {
			let array: Vec<H256> = vec![[1u8; 32].into(), [2u8; 32].into()];

			let input_data = EvmDataWriter::new()
				.write_selector(Action::EncodeNominate)
				.write(array)
				.build();

			println!("{:?}", input_data);
			let expected_bytes: Bytes =
				TestEncoder::encode_call(AvailableStakeCalls::Nominate(vec![
					[1u8; 32].into(),
					[2u8; 32].into(),
				]))
				.as_slice()
				.into();

			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: EvmDataWriter::new().write(expected_bytes).build(),
				cost: Default::default(),
				logs: Default::default(),
			}));

			assert_eq!(
				Precompiles::execute(precompile_address(), &input_data, None, &evm_test_context()),
				expected_result
			);
		});
}

#[test]
fn test_encode_rebond() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000)])
		.build()
		.execute_with(|| {
			let amount: U256 = 100u32.into();

			let input_data = EvmDataWriter::new()
				.write_selector(Action::EncodeRebond)
				.write(amount)
				.build();

			let expected_bytes: Bytes =
				TestEncoder::encode_call(AvailableStakeCalls::Rebond(100u128))
					.as_slice()
					.into();

			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: EvmDataWriter::new().write(expected_bytes).build(),
				cost: Default::default(),
				logs: Default::default(),
			}));

			assert_eq!(
				Precompiles::execute(precompile_address(), &input_data, None, &evm_test_context()),
				expected_result
			);
		});
}

#[test]
fn test_encode_set_controller() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000)])
		.build()
		.execute_with(|| {
			let controller: H256 = [1u8; 32].into();

			let input_data = EvmDataWriter::new()
				.write_selector(Action::EncodeSetController)
				.write(controller)
				.build();

			let expected_bytes: Bytes =
				TestEncoder::encode_call(AvailableStakeCalls::SetController([1u8; 32].into()))
					.as_slice()
					.into();

			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: EvmDataWriter::new().write(expected_bytes).build(),
				cost: Default::default(),
				logs: Default::default(),
			}));

			assert_eq!(
				Precompiles::execute(precompile_address(), &input_data, None, &evm_test_context()),
				expected_result
			);
		});
}

#[test]
fn test_encode_set_payee() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000)])
		.build()
		.execute_with(|| {
			let reward_dest = 2u8;
			let reward_dest_address: H256 = [0u8; 32].into();

			let input_data = EvmDataWriter::new()
				.write_selector(Action::EncodeSetPayee)
				.write(reward_dest)
				.write(reward_dest_address)
				.build();

			let expected_bytes: Bytes = TestEncoder::encode_call(AvailableStakeCalls::SetPayee(
				RewardDestination::Controller,
			))
			.as_slice()
			.into();

			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: EvmDataWriter::new().write(expected_bytes).build(),
				cost: Default::default(),
				logs: Default::default(),
			}));

			assert_eq!(
				Precompiles::execute(precompile_address(), &input_data, None, &evm_test_context()),
				expected_result
			);
		});
}

#[test]
fn test_encode_unbond() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000)])
		.build()
		.execute_with(|| {
			let amount: U256 = 100u32.into();

			let input_data = EvmDataWriter::new()
				.write_selector(Action::EncodeUnbond)
				.write(amount)
				.build();

			let expected_bytes: Bytes =
				TestEncoder::encode_call(AvailableStakeCalls::Unbond(100u32.into()))
					.as_slice()
					.into();

			// Expected result is one
			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: EvmDataWriter::new().write(expected_bytes).build(),
				cost: Default::default(),
				logs: Default::default(),
			}));

			// Assert that no props have been opened.
			assert_eq!(
				Precompiles::execute(precompile_address(), &input_data, None, &evm_test_context()),
				expected_result
			);
		});
}

#[test]
fn test_encode_validate() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000)])
		.build()
		.execute_with(|| {
			let amount: U256 = 100u32.into();
			let blocked = true;

			let input_data = EvmDataWriter::new()
				.write_selector(Action::EncodeValidate)
				.write(amount)
				.write(blocked)
				.build();

			let expected_bytes: Bytes =
				TestEncoder::encode_call(AvailableStakeCalls::Validate(ValidatorPrefs {
					commission: Perbill::from_parts(100u32.into()),
					blocked: true,
				}))
				.as_slice()
				.into();

			// Expected result is one
			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: EvmDataWriter::new().write(expected_bytes).build(),
				cost: Default::default(),
				logs: Default::default(),
			}));

			// Assert that no props have been opened.
			assert_eq!(
				Precompiles::execute(precompile_address(), &input_data, None, &evm_test_context()),
				expected_result
			);
		});
}

#[test]
fn test_encode_withdraw_unbonded() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000)])
		.build()
		.execute_with(|| {
			let amount: U256 = 100u32.into();

			let input_data = EvmDataWriter::new()
				.write_selector(Action::EncodeWithdrawUnbonded)
				.write(amount)
				.build();

			// Ethereum style
			let expected_bytes: Bytes =
				TestEncoder::encode_call(AvailableStakeCalls::WithdrawUnbonded(100u32.into()))
					.as_slice()
					.into();

			let expected_result = Some(Ok(PrecompileOutput {
				exit_status: ExitSucceed::Returned,
				output: EvmDataWriter::new().write(expected_bytes).build(),
				cost: Default::default(),
				logs: Default::default(),
			}));

			assert_eq!(
				Precompiles::execute(precompile_address(), &input_data, None, &evm_test_context()),
				expected_result
			);
		});
}
