// Copyright 2019 The Epic Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! JSON-RPC Stub generation for the Foreign API

use crate::keychain::Keychain;
use crate::libwallet::{
	self, BlockFees, CbData, ErrorKind, InitTxArgs, IssueInvoiceTxArgs, NodeClient,
	NodeVersionInfo, Slate, SlateVersion, VersionInfo, VersionedCoinbase, VersionedSlate,
	WalletLCProvider,
};
use crate::{Foreign, ForeignCheckMiddlewareFn};
use easy_jsonrpc_mw;

/// Public definition used to generate Foreign jsonrpc api.
/// * When running `epic-wallet listen` with defaults, the V2 api is available at
/// `localhost:3415/v2/foreign`
/// * The endpoint only supports POST operations, with the json-rpc request as the body
#[easy_jsonrpc_mw::rpc]
pub trait ForeignRpc {
	/**
	Networked version of [Foreign::check_version](struct.Foreign.html#method.check_version).

	# Json rpc example

	```
	# epic_wallet_api::doctest_helper_json_rpc_foreign_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "check_version",
		"id": 1,
		"params": []
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": {
				"foreign_api_version": 2,
				"supported_slate_versions": [
					"V3",
					"V2"
				]
			}
		}
	}
	# "#
	# ,false, 0, false, false);
	```
	*/
	fn check_version(&self) -> Result<VersionInfo, ErrorKind>;

	/**
	Networked Legacy (non-secure token) version of [Foreign::build_coinbase](struct.Foreign.html#method.build_coinbase).

	# Json rpc example

	```
	# epic_wallet_api::doctest_helper_json_rpc_foreign_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "build_coinbase",
		"id": 1,
		"params": [
			{
				"fees": 0,
				"height": 0,
				"key_id": null
			}
		]
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": {
				"kernel": {
					"excess": "0817b0f50ed29f9641d0741185205f1bfa15b80c727a7a99597fe2ceb073cfc73d",
					"excess_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841b97f48d17b7a42e23a4264ba232741797fef770808ca29450e35fdc3faaa6eb04",
					"features": "Coinbase",
					"fee": "0",
					"lock_height": "0"
				},
				"key_id": "0300000000000000000000000400000000",
				"output": {
					"commit": "09bfff863041e74c3068e8b43b2120aed60d5271c07bbf9d6172a8cefa8be6600a",
					"features": "Coinbase",
					"proof": "10437a3646185865241c3459c70cd3e1231a78098d29ba5fd9517aa5bb0890be089157af72d1827113d32a60c8708f83d86be0c154eb23e94b1e3c9130b970090626b2c7da298ce3a52bece998ebc1b8cc0bbc4a974a35194336f32b89cfc787a4dd793cc9626cf01bf08febe0f3e0e06ab6970e3fc9704be457856a693ee2d4508ccabd3ee259da025ff96d3905c62b9318072be0ee38ecd8b1d6097da655489084821d56bb693f7a98da23cdedc708d4b0e1033d0dfe14eef590a9b4776a07a2b9eff0f34ca2e51832a7b9930eaf215c30d60e4219af25c6da985e879a5552c429010be0dcf7405dce523d7c23e800227e7b39be3e8cd6186e4212c2471a038cbc543228e27b16c5e32214ca807fe2c82252cfddf77cc8082ee630bb951b02d7d56e8b857f9d8d2a514cb92695ed3d3a4056b6529b6f8e213d89935f0229755243a02f2c94ef828d1bc74f8cc960b7744d09c6b087269b2f1fcd4cf7c5d57ea45402a45bf2eeab519402e33dd51c9b5a1afb109d0994fc4d252d3820822b7c24800f37f908a8312e01c20ed3c6786c8205bc69d88ecd5484a6474cd9ab223529067eab8e4969621a8e84307b1981c26270ccb137575cc1cace7e9f465633dec761a11d1f137f7a5268ee71e5a43ea6c67400ce422dbeefc88d7568abc7ad332ef14fc6bc625f0fe5fbecbbcda32175bc7a044248b877aa2b26d14530670750961fa3acb47a3a362776aef679f2468d5c6d543cfe9d0f711c7506de6c71502bd228102eff29e53d3b8358eaaedfd1c0bbe484b544c39f14a5134b726c3c3518a0a455ddada5f9967e7ca769fe5dec827a13e01b2897f39dbd507d4a908016efaef6d3ea3888f6ef8d32087f7ddcc18124be7d1809b751585085d259827b8921d8d032a20db5dd5548d2a21727e472e1a1dcd06697953c2e015a74a7fd1aa7bc03d166"
				}
			}
		}
	}
	# "#
	# ,false, 4, false, false);
	```
	*/

	fn build_coinbase(&self, block_fees: &BlockFees) -> Result<VersionedCoinbase, ErrorKind>;
	fn build_foundation(&self, block_fees: &BlockFees) -> Result<VersionedCoinbase, ErrorKind>;

	/**
	Networked version of [Foreign::verify_slate_messages](struct.Foreign.html#method.verify_slate_messages).

	# Json rpc example

	```
	# epic_wallet_api::doctest_helper_json_rpc_foreign_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "verify_slate_messages",
		"id": 1,
		"params": [ {
				"amount": "6000000000",
				"fee": "8000000",
				"height": "4",
				"id": "0436430c-2b02-624c-2032-570501212b00",
				"lock_height": "4",
				"ttl_cutoff_height": null,
				"payment_proof": null,
				"num_participants": 2,
				"participant_data": [
				{
					"id": "0",
					"message": "my message",
					"message_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841b1d4c1358be398f801eb90d933774b5218fa7e769b11c4c640402253353656f75",
					"part_sig": null,
					"public_blind_excess": "034b4df2f0558b73ea72a1ca5c4ab20217c66bbe0829056fca7abe76888e9349ee",
					"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
				}
				],
				"tx": {
					"body": {
						"inputs": [
						{
							"commit": "08e1da9e6dc4d6e808a718b2f110a991dd775d65ce5ae408a4e1f002a4961aa9e7",
							"features": "Coinbase"
						}
						],
						"kernels": [
						{
							"excess": "000000000000000000000000000000000000000000000000000000000000000000",
							"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
							"features": "HeightLocked",
							"fee": "8000000",
							"lock_height": "4"
						}
						],
						"outputs": [
						{
							"commit": "094be57c91787fc2033d5d97fae099f1a6ddb37ea48370f1a138f09524c767fdd3",
							"features": "Plain",
							"proof": "2a42e9e902b70ce44e1fccb14de87ee0a97100bddf12c6bead1b9c5f4eb60300f29c13094fa12ffeee238fb4532b18f6b61cf51b23c1c7e1ad2e41560dc27edc0a2b9e647a0b3e4e806fced5b65e61d0f1f5197d3e2285c632d359e27b6b9206b2caffea4f67e0c7a2812e7a22c134b98cf89bd43d9f28b8bec25cce037a0ac5b1ae8f667e54e1250813a5263004486b4465ad4e641ab2b535736ea26535a11013564f08f483b7dab1c2bcc3ee38eadf2f7850eff7e3459a4bbabf9f0cf6c50d0c0a4120565cd4a2ce3e354c11721cd695760a24c70e0d5a0dfc3c5dcd51dfad6de2c237a682f36dc0b271f21bb3655e5333016aaa42c2efa1446e5f3c0a79ec417c4d30f77556951cb0f05dbfafb82d9f95951a9ea241fda2a6388f73ace036b98acce079f0e4feebccc96290a86dcc89118a901210b245f2d114cf94396e4dbb461e82aa26a0581389707957968c7cdc466213bb1cd417db207ef40c05842ab67a01a9b96eb1430ebc26e795bb491258d326d5174ad549401059e41782121e506744af8af9d8e493644a87d613600888541cbbe538c625883f3eb4aa3102c5cfcc25de8e97af8927619ce6a731b3b8462d51d993066b935b0648d2344ad72e4fd70f347fbd81041042e5ea31cc7b2e3156a920b80ecba487b950ca32ca95fae85b759c936246ecf441a9fdd95e8fee932d6782cdec686064018c857efc47fb4b2a122600d5fdd79af2486f44df7e629184e1c573bc0a9b3feb40b190ef2861a1ab45e2ac2201b9cd42e495deea247269820ed32389a2810ad6c0f9a296d2a2d9c54089fed50b7f5ecfcd33ab9954360e1d7f5598c32128cfcf2a1d8bf14616818da8a5343bfa88f0eedf392e9d4ab1ace1b60324129cd4852c2e27813a9cf71a6ae6229a4fcecc1a756b3e664c5f50af333082616815a3bec8fc0b75b8e4e767d719"
						}
						]
					},
					"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
				},
				"version_info": {
					"orig_version": 2,
					"version": 2,
					"block_header_version": 6
				}
			}
		]
	}
	# "#
	# ,
	# r#"
	{
		"jsonrpc": "2.0",
		"id": 1,
		"result": {
			"Ok": null
		}
	}
	# "#
	# ,false, 1 ,false, false);
	```
	*/
	fn verify_slate_messages(&self, slate: VersionedSlate) -> Result<(), ErrorKind>;

	/**
	Networked version of [Foreign::receive_tx](struct.Foreign.html#method.receive_tx).

	# Json rpc example

	```
	# epic_wallet_api::doctest_helper_json_rpc_foreign_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "receive_tx",
		"id": 1,
		"params": [
			{
			"version_info": {
				"version": 2,
				"orig_version": 2,
				"block_header_version": 6
			},
			"num_participants": 2,
			"id": "0436430c-2b02-624c-2032-570501212b00",
			"tx": {
				"offset": "d202964900000000d302964900000000d402964900000000d502964900000000",
				"body": {
					"inputs": [
						{
							"features": "Coinbase",
							"commit": "087df32304c5d4ae8b2af0bc31e700019d722910ef87dd4eec3197b80b207e3045"
						},
						{
							"features": "Coinbase",
							"commit": "08e1da9e6dc4d6e808a718b2f110a991dd775d65ce5ae408a4e1f002a4961aa9e7"
						}
					],
					"outputs": [
						{
							"features": "Plain",
							"commit": "0812276cc788e6870612296d926cba9f0e7b9810670710b5a6e6f1ba006d395774",
							"proof": "dcff6175390c602bfa92c2ffd1a9b2d84dcc9ea941f6f317bdd0f875244ef23e696fd17c71df79760ce5ce1a96aab1d15dd057358dc835e972febeb86d50ccec0dad7cfe0246d742eb753cf7b88c045d15bc7123f8cf7155647ccf663fca92a83c9a65d0ed756ea7ebffd2cac90c380a102ed9caaa355d175ed0bf58d3ac2f5e909d6c447dfc6b605e04925c2b17c33ebd1908c965a5541ea5d2ed45a0958e6402f89d7a56df1992e036d836e74017e73ccad5cb3a82b8e139e309792a31b15f3ffd72ed033253428c156c2b9799458a25c1da65b719780a22de7fe7f437ae2fccd22cf7ea357ab5aa66a5ef7d71fb0dc64aa0b5761f68278062bb39bb296c787e4cabc5e2a2933a416ce1c9a9696160386449c437e9120f7bb26e5b0e74d1f2e7d5bcd7aafb2a92b87d1548f1f911fb06af7bd6cc13cee29f7c9cb79021aed18186272af0e9d189ec107c81a8a3aeb4782b0d950e4881aa51b776bb6844b25bce97035b48a9bdb2aea3608687bcdd479d4fa998b5a839ff88558e4a29dff0ed13b55900abb5d439b70793d902ae9ad34587b18c919f6b875c91d14deeb1c373f5e76570d59a6549758f655f1128a54f162dfe8868e1587028e26ad91e528c5ae7ee9335fa58fb59022b5de29d80f0764a9917390d46db899acc6a5b416e25ecc9dccb7153646addcc81cadb5f0078febc7e05d7735aba494f39ef05697bbcc9b47b2ccc79595d75fc13c80678b5e237edce58d731f34c05b1ddcaa649acf2d865bbbc3ceda10508bcdd29d0496744644bf1c3516f6687dfeef5649c7dff90627d642739a59d91a8d1d0c4dc55d74a949e1074427664b467992c9e0f7d3af9d6ea79513e8946ddc0d356bac49878e64e6a95b0a30214214faf2ce317fa622ff3266b32a816e10a18e6d789a5da1f23e67b4f970a68a7bcd9e18825ee274b0483896a40"
						}
					],
					"kernels": [
						{
							"features": "Plain",
							"fee": "7000000",
							"lock_height": "0",
							"excess": "000000000000000000000000000000000000000000000000000000000000000000",
							"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
						}
					]
				}
			},
			"amount": "60000000000",
			"fee": "7000000",
			"height": "5",
			"lock_height": "0",
			"ttl_cutoff_height": null,
			"payment_proof": null,
			"participant_data": [
				{
					"id": "0",
					"public_blind_excess": "033ac2158fa0077f087de60c19d8e431753baa5b63b6e1477f05a2a6e7190d4592",
					"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f",
					"part_sig": null,
					"message": null,
					"message_sig": null
				}
			]
		},
		null,
		"Thanks, Yeastplume"
		]
	}
	# "#
	# ,
	# r#"
	{
	"id": 1,
	"jsonrpc": "2.0",
		"result": {
			"Ok": {
				"amount": "60000000000",
				"fee": "7000000",
				"height": "5",
				"id": "0436430c-2b02-624c-2032-570501212b00",
				"lock_height": "0",
				"ttl_cutoff_height": null,
				"payment_proof": null,
				"num_participants": 2,
				"participant_data": [
				{
					"id": "0",
					"message": null,
					"message_sig": null,
					"part_sig": null,
					"public_blind_excess": "033ac2158fa0077f087de60c19d8e431753baa5b63b6e1477f05a2a6e7190d4592",
					"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
				},
				{
					"id": "1",
					"message": "Thanks, Yeastplume",
		  "message_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841b30a1f1b21eade1b4bd211e1f137fbdbca1b78dc43da21b1695f6a0edf2437ff9",
					"part_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841b2b35bd28dfd2269e0670e0cf9270bd6df2d03fbd64523ee4ae622396055b96fc",
					"public_blind_excess": "038fe0443243dab173c068ef5fa891b242d2b5eb890ea09475e6e381170442ee16",
					"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
				}
				],
				"tx": {
				"body": {
					"inputs": [
					{
						"commit": "087df32304c5d4ae8b2af0bc31e700019d722910ef87dd4eec3197b80b207e3045",
						"features": "Coinbase"
					},
					{
						"commit": "08e1da9e6dc4d6e808a718b2f110a991dd775d65ce5ae408a4e1f002a4961aa9e7",
						"features": "Coinbase"
					}
					],
					"kernels": [
					{
						"excess": "000000000000000000000000000000000000000000000000000000000000000000",
						"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
						"features": "Plain",
						"fee": "7000000",
						"lock_height": "0"
					}
					],
					"outputs": [
					{
						"commit": "084ee97defa8c37124d4c69baa753e2532535faa81f79ea5e0489db25297d5beb8",
						"features": "Plain",
						"proof": "007df7dddd1efca757b2070740cc604628390eb59e151f96ff2eaa5361f5435fd1aa6ea3febc97fcfe1b3248d040c82de36180392976ba2d1147c2fb021c87ad044f1f9763934d9d3f4431417762eed03c53ce17aedb7824565c1f48fccec9c4abc0d28bd32b02ce9bee40bf6a60cf7c9c203cc24e4b779f901e12c987573698cf7f04e3aace26e71262138605424800adf3295d09f7f45dddf1855c785e98d45eae3cd111d18552e733895458df15e71a13838d789a4cb369f4ddb8aa9c503b080fd88a147245df0522d4136d36a183bd941e6cf94dffc78438b12194d4df7114d1e27a7a2f014920a321223ecbebb2b9642a22f8ed4e74883125f3e757b2f118853ffab1b68f15c1a2d021e583ff3fd1ea28720a81325b3cc2327ba9fb2fd9b2644adb7f3c7b2e319b2536a34f67e6f09346f24da6bcae1b241f8590493476dfe35b183e54f105eb219b601e0e53965409701dc1fd9562c42ad977505ea7bf264f01770569a4a358a70fb0b2c65969fac3b23954f0ca0adace0703243f1dab626509a8656e7a981709c3ac1d51694bafa55aad45c101937cbf3e45d6708c07be71419769a10a4f64f2b7d53a54eac73cdbd3279f91c5f8991a4b17621c36195a9391364fa221e8a8dee21ebc3a6eb9cd2940a3676e7ef3cdd46319bdc11f748785e49ff41bec2c3243255d83c6895bc0c893e6a772d7440a68321246b177709d3bd82d0dc2f5bca40c878e859b6f82319a386e0b7fcbc8010a25178b08418389ba7c6a77f99ac7f4ae5c686ab6574fcd0116f8573bccda3edfdff36c9c92ce2fb8bfb0ce2fe5c6b2498c6eb16fc2d40de9ddcba199a7e93d648abf39d6b248e196de7127e6b812e3080497f2a82afa69a471ab511e753e5b17a1c39c6728a065898af6674608d92a625e96e2f0258fe2eb06a27d0586d889d61f97faaa3facf58cda"
					},
					{
						"commit": "0812276cc788e6870612296d926cba9f0e7b9810670710b5a6e6f1ba006d395774",
						"features": "Plain",
						"proof": "dcff6175390c602bfa92c2ffd1a9b2d84dcc9ea941f6f317bdd0f875244ef23e696fd17c71df79760ce5ce1a96aab1d15dd057358dc835e972febeb86d50ccec0dad7cfe0246d742eb753cf7b88c045d15bc7123f8cf7155647ccf663fca92a83c9a65d0ed756ea7ebffd2cac90c380a102ed9caaa355d175ed0bf58d3ac2f5e909d6c447dfc6b605e04925c2b17c33ebd1908c965a5541ea5d2ed45a0958e6402f89d7a56df1992e036d836e74017e73ccad5cb3a82b8e139e309792a31b15f3ffd72ed033253428c156c2b9799458a25c1da65b719780a22de7fe7f437ae2fccd22cf7ea357ab5aa66a5ef7d71fb0dc64aa0b5761f68278062bb39bb296c787e4cabc5e2a2933a416ce1c9a9696160386449c437e9120f7bb26e5b0e74d1f2e7d5bcd7aafb2a92b87d1548f1f911fb06af7bd6cc13cee29f7c9cb79021aed18186272af0e9d189ec107c81a8a3aeb4782b0d950e4881aa51b776bb6844b25bce97035b48a9bdb2aea3608687bcdd479d4fa998b5a839ff88558e4a29dff0ed13b55900abb5d439b70793d902ae9ad34587b18c919f6b875c91d14deeb1c373f5e76570d59a6549758f655f1128a54f162dfe8868e1587028e26ad91e528c5ae7ee9335fa58fb59022b5de29d80f0764a9917390d46db899acc6a5b416e25ecc9dccb7153646addcc81cadb5f0078febc7e05d7735aba494f39ef05697bbcc9b47b2ccc79595d75fc13c80678b5e237edce58d731f34c05b1ddcaa649acf2d865bbbc3ceda10508bcdd29d0496744644bf1c3516f6687dfeef5649c7dff90627d642739a59d91a8d1d0c4dc55d74a949e1074427664b467992c9e0f7d3af9d6ea79513e8946ddc0d356bac49878e64e6a95b0a30214214faf2ce317fa622ff3266b32a816e10a18e6d789a5da1f23e67b4f970a68a7bcd9e18825ee274b0483896a40"
					}
					]
				},
				"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
				},
				"version_info": {
					"orig_version": 2,
					"version": 2,
					"block_header_version": 6
				}
			}
		}
	}
	# "#
	# ,false, 5, true, false);
	```
	*/
	fn receive_tx(
		&self,
		slate: VersionedSlate,
		dest_acct_name: Option<String>,
		message: Option<String>,
	) -> Result<VersionedSlate, ErrorKind>;

	/**

	Networked version of [Foreign::finalize_invoice_tx](struct.Foreign.html#method.finalize_invoice_tx).

	# Json rpc example

	```
	# epic_wallet_api::doctest_helper_json_rpc_foreign_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "finalize_invoice_tx",
		"id": 1,
		"params": [{
			"version_info": {
				"version": 2,
				"orig_version": 2,
				"block_header_version": 6
			},
			"num_participants": 2,
			"id": "0436430c-2b02-624c-2032-570501212b00",
			"tx": {
				"offset": "d202964900000000d302964900000000d402964900000000d502964900000000",
				"body": {
					"inputs": [
						{
							"features": "Coinbase",
							"commit": "09d8836ffd38ffca42567ef965fdcf1f35b05aeb357664d70cd482438ca0ca0c9e"
						},
						{
							"features": "Coinbase",
							"commit": "089be87c488db1e7c783b19272a83b23bce56a5263163554b345c6f7ffedac517e"
						}
					],
					"outputs": [
						{
							"features": "Plain",
							"commit": "091454e23b4dbc71f546a41035d69f4c87d0f6efb5ceb119cc0d2eef80ba1928d7",
							"proof": "c841838499e57d3cca5dffca671596c746b3b49150ca62b94928018337dc695ec646c3e38dbd291d7e1113f72c41e0597e0956261cf9384841d6d37953c8108a0285ba2f33d3b86a235c908a7a6ff8b6a2f7d3ee4ab05dd7a1e09064a72d993fec2d4ed03c89ca390598a9340bf8512cb8e57c3d62794386101d4563a3302041dfc364b9f96f2c06eecac1b33d608c59a84bd69f3d075b60896de76babfb538e558c844c6b09feff47dbfad3854275f68c2ce2e0d9a14fc0564bb069e3558df17447557bb501c3b0b97396b0f9b1cbc5f4f50feb33fd6c6f1cbf10b1d150f7c70144e0b28fa749dc4d0d45ac4382611a8e8b2a74f40d61411fd7fd0713ed075f2ce7d30a9b16f758177d94df2e6de611d7e75b87aa8303d544937ef4afec55d17e074817342fb498fb0c6ffca85f431cb520cd434d217ca303f5d194a39d0a93c0e3c37f5e503695502dc2a5cbe31911bc557402b36a9c2749e73ca8803f2962be75001042a03c9fce5dea2189fe05e61d4a7717397ff481566e77804bd51da8fc2f98bce3bab3c2c9b1ffa19b8840a752a8e9e0825cea9927b97c9ddb425ce5a9a689fb428e0890352b5c0fe93111cfaa1e5c6e06daebdbf242630d6b53c676241c33a17faed95901282f6e67b210e187e075f77f2582c9e23f3a386147c9f6a9432418ade828620a4fa0578ac04bb59b36b4ca679fc57d749d8dd47a1a61066ba6e0ff0a434b8f8668d9c9490735dd82c1cb4bcfafb1c213385ece71ef3c66e70aaeddf2bf35674e4f4c78868dfd8475f88fac65fade11de8b372337c6bdcf2d684b89094c732b8acf2bbe4ceedcb807da51171ffae4b51a80f221fb1b4f803ad1e13dff1b4a476297478a23d6382a98c6121018872bfbd85137cb30d230bea9c8ef42848e49f3dc92333a69dd50f525b24dab0b8c286f721e57462e4a538cc6019d"
						},
						{
							"features": "Plain",
							"commit": "09414416856d650cd42abad97943f8ea32ff19e7d5d10201ff790d1ca941f578ed",
							"proof": "f75fbd6a43556c5a0a18a1b304ce825684560c1f5591b572ea24f704d8b00c2df2b8bcfee98ede60ccacb45e146f3bd8b31e7f84019d68493c6a80be557d42a00db146eff731720c608e27c064dd236ab00a1e4e5d062ad371c1cbf5c3bb9a50edb2e6eaff9727c13b56c0834d311680092beb6aeeb76abe2cf352a4b79698da3a06d39a78c65c9113583a3ed7aef098e08138db7b0b6bbfce9bdcbe583c65590b27634fee1a1cccb59f054132cbc00a712ff0b4d9ce6d58a4a2bf62dce49defce48a5b72236db3632728e2effce93297f675743d41d5fdb5985fd11d7f4da201832274b9d53cbbbe221b39cbb71adf16d8ec00a1ed42f5593721396202bdf6ab802ea927f1e63456b61b679bffcf0bf7151b625f5179facfd2257ffe9e0c21dac4409100860e031a49587f4ad86cef66fcece0ef177bf0bda6f12cec3be09a659dd3feb954c36f655364845bf451b207cf500e44c9d1658fbd5248d2257ad783c6e02a32565ee056731d6a7b06669e6bf373f437c33dbb3b70dd913f83343f029dc288e311d92c0c8b42b227597679ebfad3cf371923530c87da23ff994a9278df2f74ce71687d5531d9594282ce0861954df896be88a01405291f0d51169fe2317e4b0769f41d9472c5ce239f53abd16b551024bc7f705d3bcd13f4118f105685a80b1aaded0bea9884f18600a9d6066c48edc596c62a69d357de14789eb60fe61de82aed04b7d5727ac25b976d1ebf60bdcaf6359329a69cac6db212cc049b5159a6e7b83453139cab4ae57b585f0ba1c8f90906db01a610af7a7a3a66e92223d91121643ad3ebbaf61ee8a2394062b9ea4b493a410ea9937ecb09ee31696412d024e455772788cb8a3b5c82699d1b723f2013f1e3a2c4c2f558d6adc8289147d171b88d9fc41e64c30ef1b064b08f7447b6b24b425103e8588203e7328b845b719"
						}
					],
					"kernels": [
						{
							"features": "Plain",
							"fee": "700000",
							"lock_height": "0",
							"excess": "000000000000000000000000000000000000000000000000000000000000000000",
							"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
						}
					]
				}
			},
			"amount": "600000000",
			"fee": "700000",
			"height": "5",
			"lock_height": "0",
			"ttl_cutoff_height": null,
			"payment_proof": null,
			"participant_data": [
				{
					"id": "1",
					"public_blind_excess": "029e99ff15745fc6f3988957d34340ecdbeea7de7d9d231a827413ec9912c61aad",
					"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f",
					"part_sig": null,
					"message": null,
					"message_sig": null
				},
				{
					"id": "0",
					"public_blind_excess": "023be94e6254d688b05c0c593089ee551b62034c2a37950cff4ec4b2e2c3f280c4",
					"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f",
					"part_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841bea191bb6f1a43b1596ea5954d79816b49e267db71df6e2328b15fc34170fb37f",
					"message": null,
					"message_sig": null
				}
			]
		}]
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": {
				"amount": "600000000",
				"fee": "700000",
				"height": "5",
				"id": "0436430c-2b02-624c-2032-570501212b00",
				"lock_height": "0",
				"ttl_cutoff_height": null,
				"payment_proof": null,
				"num_participants": 2,
				"participant_data": [
					{
						"id": "1",
						"message": null,
						"message_sig": null,
						"part_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841bbb71aee3aa45c5f9bbc9f9f2e92b5486b018f3b978a0bd5ffed7e674b16c747d",
						"public_blind_excess": "029e99ff15745fc6f3988957d34340ecdbeea7de7d9d231a827413ec9912c61aad",
						"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
					},
					{
						"id": "0",
						"message": null,
						"message_sig": null,
						"part_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841bea191bb6f1a43b1596ea5954d79816b49e267db71df6e2328b15fc34170fb37f",
						"public_blind_excess": "023be94e6254d688b05c0c593089ee551b62034c2a37950cff4ec4b2e2c3f280c4",
						"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
					}
				],
				"tx": {
					"body": {
						"inputs": [
							{
								"commit": "09d8836ffd38ffca42567ef965fdcf1f35b05aeb357664d70cd482438ca0ca0c9e",
								"features": "Coinbase"
							},
							{
								"commit": "089be87c488db1e7c783b19272a83b23bce56a5263163554b345c6f7ffedac517e",
								"features": "Coinbase"
							}
						],
						"kernels": [
							{
								"excess": "08d09187cb93cf5d6b97b28e8ca529912bf35ec8773d3e9af9b3c174a270dc7f05",
								"excess_sig": "66074d25a751c4743342c90ad8ead9454daa00d9b9aed29bca321036d16c4b4da58bc9999cea000f52b45347c1c46a3a4f3f70719696a09289ede2a9c87b27fd",
								"features": "Plain",
								"fee": "700000",
								"lock_height": "0"
							}
						],
						"outputs": [
							{
								"commit": "091454e23b4dbc71f546a41035d69f4c87d0f6efb5ceb119cc0d2eef80ba1928d7",
								"features": "Plain",
								"proof": "c841838499e57d3cca5dffca671596c746b3b49150ca62b94928018337dc695ec646c3e38dbd291d7e1113f72c41e0597e0956261cf9384841d6d37953c8108a0285ba2f33d3b86a235c908a7a6ff8b6a2f7d3ee4ab05dd7a1e09064a72d993fec2d4ed03c89ca390598a9340bf8512cb8e57c3d62794386101d4563a3302041dfc364b9f96f2c06eecac1b33d608c59a84bd69f3d075b60896de76babfb538e558c844c6b09feff47dbfad3854275f68c2ce2e0d9a14fc0564bb069e3558df17447557bb501c3b0b97396b0f9b1cbc5f4f50feb33fd6c6f1cbf10b1d150f7c70144e0b28fa749dc4d0d45ac4382611a8e8b2a74f40d61411fd7fd0713ed075f2ce7d30a9b16f758177d94df2e6de611d7e75b87aa8303d544937ef4afec55d17e074817342fb498fb0c6ffca85f431cb520cd434d217ca303f5d194a39d0a93c0e3c37f5e503695502dc2a5cbe31911bc557402b36a9c2749e73ca8803f2962be75001042a03c9fce5dea2189fe05e61d4a7717397ff481566e77804bd51da8fc2f98bce3bab3c2c9b1ffa19b8840a752a8e9e0825cea9927b97c9ddb425ce5a9a689fb428e0890352b5c0fe93111cfaa1e5c6e06daebdbf242630d6b53c676241c33a17faed95901282f6e67b210e187e075f77f2582c9e23f3a386147c9f6a9432418ade828620a4fa0578ac04bb59b36b4ca679fc57d749d8dd47a1a61066ba6e0ff0a434b8f8668d9c9490735dd82c1cb4bcfafb1c213385ece71ef3c66e70aaeddf2bf35674e4f4c78868dfd8475f88fac65fade11de8b372337c6bdcf2d684b89094c732b8acf2bbe4ceedcb807da51171ffae4b51a80f221fb1b4f803ad1e13dff1b4a476297478a23d6382a98c6121018872bfbd85137cb30d230bea9c8ef42848e49f3dc92333a69dd50f525b24dab0b8c286f721e57462e4a538cc6019d"
							},
							{
								"commit": "09414416856d650cd42abad97943f8ea32ff19e7d5d10201ff790d1ca941f578ed",
								"features": "Plain",
								"proof": "f75fbd6a43556c5a0a18a1b304ce825684560c1f5591b572ea24f704d8b00c2df2b8bcfee98ede60ccacb45e146f3bd8b31e7f84019d68493c6a80be557d42a00db146eff731720c608e27c064dd236ab00a1e4e5d062ad371c1cbf5c3bb9a50edb2e6eaff9727c13b56c0834d311680092beb6aeeb76abe2cf352a4b79698da3a06d39a78c65c9113583a3ed7aef098e08138db7b0b6bbfce9bdcbe583c65590b27634fee1a1cccb59f054132cbc00a712ff0b4d9ce6d58a4a2bf62dce49defce48a5b72236db3632728e2effce93297f675743d41d5fdb5985fd11d7f4da201832274b9d53cbbbe221b39cbb71adf16d8ec00a1ed42f5593721396202bdf6ab802ea927f1e63456b61b679bffcf0bf7151b625f5179facfd2257ffe9e0c21dac4409100860e031a49587f4ad86cef66fcece0ef177bf0bda6f12cec3be09a659dd3feb954c36f655364845bf451b207cf500e44c9d1658fbd5248d2257ad783c6e02a32565ee056731d6a7b06669e6bf373f437c33dbb3b70dd913f83343f029dc288e311d92c0c8b42b227597679ebfad3cf371923530c87da23ff994a9278df2f74ce71687d5531d9594282ce0861954df896be88a01405291f0d51169fe2317e4b0769f41d9472c5ce239f53abd16b551024bc7f705d3bcd13f4118f105685a80b1aaded0bea9884f18600a9d6066c48edc596c62a69d357de14789eb60fe61de82aed04b7d5727ac25b976d1ebf60bdcaf6359329a69cac6db212cc049b5159a6e7b83453139cab4ae57b585f0ba1c8f90906db01a610af7a7a3a66e92223d91121643ad3ebbaf61ee8a2394062b9ea4b493a410ea9937ecb09ee31696412d024e455772788cb8a3b5c82699d1b723f2013f1e3a2c4c2f558d6adc8289147d171b88d9fc41e64c30ef1b064b08f7447b6b24b425103e8588203e7328b845b719"
							}
						]
					},
					"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
				},
				"version_info": {
					"orig_version": 2,
					"version": 2,
					"block_header_version": 6
				}
			}
		}
	}
	# "#
	# ,false, 5, false, true);
	```
	*/
	fn finalize_invoice_tx(&self, slate: VersionedSlate) -> Result<VersionedSlate, ErrorKind>;
}

impl<'a, L, C, K> ForeignRpc for Foreign<'a, L, C, K>
where
	L: WalletLCProvider<'a, C, K>,
	C: NodeClient + 'a,
	K: Keychain + 'a,
{
	fn check_version(&self) -> Result<VersionInfo, ErrorKind> {
		Foreign::check_version(self).map_err(|e| e.kind())
	}

	fn build_coinbase(&self, block_fees: &BlockFees) -> Result<VersionedCoinbase, ErrorKind> {
		let cb: CbData = Foreign::build_coinbase(self, block_fees).map_err(|e| e.kind())?;
		Ok(VersionedCoinbase::into_version(cb, SlateVersion::V3))
	}

	fn build_foundation(&self, block_fees: &BlockFees) -> Result<VersionedCoinbase, ErrorKind> {
		let cb: CbData = Foreign::build_foundation(self, block_fees).map_err(|e| e.kind())?;
		Ok(VersionedCoinbase::into_version(cb, SlateVersion::V3))
	}

	fn verify_slate_messages(&self, slate: VersionedSlate) -> Result<(), ErrorKind> {
		Foreign::verify_slate_messages(self, &Slate::from(slate)).map_err(|e| e.kind())
	}

	fn receive_tx(
		&self,
		in_slate: VersionedSlate,
		dest_acct_name: Option<String>,
		message: Option<String>,
	) -> Result<VersionedSlate, ErrorKind> {
		let version = in_slate.version();
		let slate_from = Slate::from(in_slate);
		let out_slate = Foreign::receive_tx(
			self,
			&slate_from,
			dest_acct_name.as_ref().map(String::as_str),
			message,
		)
		.map_err(|e| e.kind())?;
		Ok(VersionedSlate::into_version(out_slate, version))
	}

	fn finalize_invoice_tx(&self, in_slate: VersionedSlate) -> Result<VersionedSlate, ErrorKind> {
		let version = in_slate.version();
		let out_slate =
			Foreign::finalize_invoice_tx(self, &Slate::from(in_slate)).map_err(|e| e.kind())?;
		Ok(VersionedSlate::into_version(out_slate, version))
	}
}

fn test_check_middleware(
	_name: ForeignCheckMiddlewareFn,
	_node_version_info: Option<NodeVersionInfo>,
	_slate: Option<&Slate>,
) -> Result<(), libwallet::Error> {
	// TODO: Implement checks
	// return Err(ErrorKind::GenericError("Test Rejection".into()))?
	Ok(())
}

/// helper to set up a real environment to run integrated doctests
pub fn run_doctest_foreign(
	request: serde_json::Value,
	test_dir: &str,
	use_token: bool,
	blocks_to_mine: u64,
	init_tx: bool,
	init_invoice_tx: bool,
) -> Result<Option<serde_json::Value>, String> {
	use easy_jsonrpc_mw::Handler;
	use epic_wallet_impls::test_framework::{self, LocalWalletClient, WalletProxy};
	use epic_wallet_impls::{DefaultLCProvider, DefaultWalletImpl};
	use epic_wallet_libwallet::{api_impl, WalletInst};
	use epic_wallet_util::epic_keychain::ExtKeychain;

	use crate::core::global::ChainTypes;
	use crate::core::{core::feijoada, global};
	use epic_wallet_util::epic_util as util;

	use std::sync::Arc;
	use util::Mutex;

	use std::fs;
	use std::thread;

	util::init_test_logger();
	let _ = fs::remove_dir_all(test_dir);
	global::set_mining_mode(ChainTypes::AutomatedTesting);
	global::set_foundation_path("../tests/assets/foundation.json".to_string());
	let mut policies: feijoada::Policy = feijoada::get_bottles_default();
	policies.insert(feijoada::PoWType::Cuckatoo, 100);
	global::set_policy_config(feijoada::PolicyConfig {
		policies: vec![policies.clone()],
		..Default::default()
	});
	let mut wallet_proxy: WalletProxy<
		DefaultLCProvider<LocalWalletClient, ExtKeychain>,
		LocalWalletClient,
		ExtKeychain,
	> = WalletProxy::new(test_dir);
	let chain = wallet_proxy.chain.clone();

	let rec_phrase_1 = util::ZeroingString::from(
		"fat twenty mean degree forget shell check candy immense awful \
		 flame next during february bulb bike sun wink theory day kiwi embrace peace lunch",
	);
	let empty_string = util::ZeroingString::from("");
	let client1 = LocalWalletClient::new("wallet1", wallet_proxy.tx.clone());
	let mut wallet1 =
		Box::new(DefaultWalletImpl::<LocalWalletClient>::new(client1.clone()).unwrap())
			as Box<
				dyn WalletInst<
					'static,
					DefaultLCProvider<LocalWalletClient, ExtKeychain>,
					LocalWalletClient,
					ExtKeychain,
				>,
			>;
	let lc = wallet1.lc_provider().unwrap();
	let _ = lc.set_top_level_directory(&format!("{}/wallet1", test_dir));
	lc.create_wallet(None, Some(rec_phrase_1), 32, empty_string.clone(), false)
		.unwrap();
	let mask1 = lc
		.open_wallet(None, empty_string.clone(), use_token, true)
		.unwrap();
	let wallet1 = Arc::new(Mutex::new(wallet1));

	if mask1.is_some() {
		println!("WALLET 1 MASK: {:?}", mask1.clone().unwrap());
	}

	wallet_proxy.add_wallet(
		"wallet1",
		client1.get_send_instance(),
		wallet1.clone(),
		mask1.clone(),
	);

	let rec_phrase_2 = util::ZeroingString::from(
		"hour kingdom ripple lunch razor inquiry coyote clay stamp mean \
		 sell finish magic kid tiny wage stand panther inside settle feed song hole exile",
	);
	let client2 = LocalWalletClient::new("wallet2", wallet_proxy.tx.clone());
	let mut wallet2 =
		Box::new(DefaultWalletImpl::<LocalWalletClient>::new(client2.clone()).unwrap())
			as Box<
				dyn WalletInst<
					'static,
					DefaultLCProvider<LocalWalletClient, ExtKeychain>,
					LocalWalletClient,
					ExtKeychain,
				>,
			>;
	let lc = wallet2.lc_provider().unwrap();
	let _ = lc.set_top_level_directory(&format!("{}/wallet2", test_dir));
	lc.create_wallet(None, Some(rec_phrase_2), 32, empty_string.clone(), false)
		.unwrap();
	let mask2 = lc
		.open_wallet(None, empty_string.clone(), use_token, true)
		.unwrap();
	let wallet2 = Arc::new(Mutex::new(wallet2));

	wallet_proxy.add_wallet(
		"wallet2",
		client2.get_send_instance(),
		wallet2.clone(),
		mask2.clone(),
	);

	// Set the wallet proxy listener running
	thread::spawn(move || {
		if let Err(e) = wallet_proxy.run() {
			error!("Wallet Proxy error: {}", e);
		}
	});

	// Mine a few blocks to wallet 1 so there's something to send
	for _ in 0..blocks_to_mine {
		let _ = test_framework::award_blocks_to_wallet(
			&chain,
			wallet1.clone(),
			(&mask1).as_ref(),
			1 as usize,
			false,
		);
		//update local outputs after each block, so transaction IDs stay consistent
		let (wallet_refreshed, _) = api_impl::owner::retrieve_summary_info(
			wallet1.clone(),
			(&mask1).as_ref(),
			&None,
			true,
			1,
		)
		.unwrap();
		assert!(wallet_refreshed);
	}

	if init_invoice_tx {
		let amount = 600_000_000;
		let mut slate = {
			let mut w_lock = wallet2.lock();
			let w = w_lock.lc_provider().unwrap().wallet_inst().unwrap();
			let args = IssueInvoiceTxArgs {
				amount,
				..Default::default()
			};
			api_impl::owner::issue_invoice_tx(&mut **w, (&mask2).as_ref(), args, true).unwrap()
		};
		slate = {
			let mut w_lock = wallet1.lock();
			let w = w_lock.lc_provider().unwrap().wallet_inst().unwrap();
			let args = InitTxArgs {
				src_acct_name: None,
				amount: slate.amount,
				minimum_confirmations: 2,
				max_outputs: 500,
				num_change_outputs: 1,
				selection_strategy_is_use_all: true,
				..Default::default()
			};
			api_impl::owner::process_invoice_tx(&mut **w, (&mask1).as_ref(), &slate, args, true)
				.unwrap()
		};
		println!("INIT INVOICE SLATE");
		// Spit out slate for input to finalize_invoice_tx
		println!("{}", serde_json::to_string_pretty(&slate).unwrap());
	}

	if init_tx {
		let amount = 600_000_000;
		let mut w_lock = wallet1.lock();
		let w = w_lock.lc_provider().unwrap().wallet_inst().unwrap();
		let args = InitTxArgs {
			src_acct_name: None,
			amount,
			minimum_confirmations: 2,
			max_outputs: 500,
			num_change_outputs: 1,
			selection_strategy_is_use_all: true,
			..Default::default()
		};
		let slate = api_impl::owner::init_send_tx(&mut **w, (&mask1).as_ref(), args, true).unwrap();
		println!("INIT SLATE");
		// Spit out slate for input to finalize_tx
		println!("{}", serde_json::to_string_pretty(&slate).unwrap());
	}

	let mut api_foreign = match init_invoice_tx {
		false => Foreign::new(wallet1, mask1, Some(test_check_middleware)),
		true => Foreign::new(wallet2, mask2, Some(test_check_middleware)),
	};
	api_foreign.doctest_mode = true;
	let foreign_api = &api_foreign as &dyn ForeignRpc;
	let res = foreign_api.handle_request(request).as_option();
	let _ = fs::remove_dir_all(test_dir);
	Ok(res)
}

#[doc(hidden)]
#[macro_export]
macro_rules! doctest_helper_json_rpc_foreign_assert_response {
	($request:expr, $expected_response:expr, $use_token:expr, $blocks_to_mine:expr, $init_tx:expr, $init_invoice_tx:expr) => {
		// create temporary wallet, run jsonrpc request on owner api of wallet, delete wallet, return
		// json response.
		// In order to prevent leaking tempdirs, This function should not panic.
		use epic_wallet_api::run_doctest_foreign;
		use serde_json;
		use serde_json::Value;
		use tempfile::tempdir;

		let dir = tempdir().map_err(|e| format!("{:#?}", e)).unwrap();
		let dir = dir
			.path()
			.to_str()
			.ok_or("Failed to convert tmpdir path to string.".to_owned())
			.unwrap();

		let request_val: Value = serde_json::from_str($request).unwrap();
		let expected_response: Value = serde_json::from_str($expected_response).unwrap();

		let response = run_doctest_foreign(
			request_val,
			dir,
			$use_token,
			$blocks_to_mine,
			$init_tx,
			$init_invoice_tx,
		)
		.unwrap()
		.unwrap();

		if response != expected_response {
			panic!(
				"(left != right) \nleft: {}\nright: {}",
				serde_json::to_string_pretty(&response).unwrap(),
				serde_json::to_string_pretty(&expected_response).unwrap()
			);
		}
	};
}
