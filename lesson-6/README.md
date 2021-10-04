# lesson-6

编译和运行
* cargo build --features runtime-benchmarks --release

* ./target/release/node-template benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_template --extrinsic do_something --steps 20 --repeat 50

## 作业：

### 为template模块do_something添加benchmark用例

``` rust
use super::*;
#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
benchmarks! {
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}
}
impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
```
### benchmark运行结果转换为对应的权重定义

> 运行结果截图：

> cd node && cargo build --features runtime-benchmarks --release && cd ..

> ./target/release/node-template benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_template --extrinsic do_something --steps 20 --repeat 50

![benchmark.png](https://github.com/zongxunjie/SA/blob/main/lesson-6/benchmark.png)

```rust
/// 使用benchmark作为权重
    #[pallet::weight(27_070_000 + T::DbWeight::get().writes(1))]
	pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {...}
```

### 选择node-template或其他节点程序，生成ChainSpec文件（未编码和编码后）

> ./target/release/node-template build-spec --disable-default-bootnode --chain local > localTestSpec.json

> ./target/release/node-template build-spec --disable-default-bootnode --chain=localTestSpec.json --raw > localTestSpecRaw.json

[localTestSpec.json](https://github.com/zongxunjie/SA/blob/main/lesson-6/localTestSpec.json)

[localTestSpecRaw.json](https://github.com/zongxunjie/SA/blob/main/lesson-6/localTestSpecRaw.json)

### 由ChainSpec，部署网络

* 生成账户 subkey generate, grandpa使用ed,其余使用sr

* 修改/bin/node/cli/src/commands.rs，添加product入口
```rust
	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let spec =
			match id {
				"" => return Err("Please specify which chain you want to run, e.g. --dev or --chain=local".into()),
				"dev" => Box::new(chain_spec::development_config()),
				"local" => Box::new(chain_spec::local_testnet_config()),
				"fir" | "flaming-fir" => Box::new(chain_spec::flaming_fir_config()?),
				"staging" => Box::new(chain_spec::staging_testnet_config()),
				"product" => Box::new(chain_spect::product_config()),
				path => Box::new(chain_spec::ChainSpec::from_json_file(
					std::path::PathBuf::from(path),
				)?),
			};
		Ok(spec)
	}
```
* 修改/bin/node/cli/src/chain_spec.rs，添加product代码内置四个验证者
```rust
fn product_config_genesis() -> GenesisConfig {
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	// and
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![(
		// s
		hex!["s"].into(),
		// c
		hex!["c"].into(),
		// g
		hex!["g"].unchecked_into(),
		// b
		hex!["b"].unchecked_into(),
		// i
		hex!["i"].unchecked_into(),
		// s
		hex!["s"].unchecked_into(),
	),(
		// s
		hex!["s"].into(),
		// c
		hex!["c"].into(),
		// g
		hex!["g"].unchecked_into(),
		// b
		hex!["b"].unchecked_into(),
		// i
		hex!["i"].unchecked_into(),
		// s
		hex!["s"].unchecked_into(),
	),(
		// s
		hex!["s"].into(),
		// c
		hex!["c"].into(),
		// g
		hex!["g"].unchecked_into(),
		// b
		hex!["b"].unchecked_into(),
		// i
		hex!["i"].unchecked_into(),
		// s
		hex!["s"].unchecked_into(),
	),(
		// s
		hex!["s"].into(),
		// c
		hex!["c"].into(),
		// g
		hex!["g"].unchecked_into(),
		// b
		hex!["b"].unchecked_into(),
		// i
		hex!["i"].unchecked_into(),
		// s
		hex!["s"].unchecked_into(),
	)];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// r
		"r"
	].into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts))
}

/// Product config.
pub fn product_config() -> ChainSpec {
	let boot_nodes = vec![];
	let properties = serde_json::map::Map<String, serde_json::Value>::new();
	let token_symbol:serde_json::Value = "xxx".into();
	let token_decimals:serde_json::Value = 0.into();
	properties.insert(String::from("tokenSymbol"), token_symbol);
	properties.insert(String::from("tokenDecimals"), token_decimals);
	ChainSpec::from_genesis(
		"xtoken",
		"x_token",
		ChainType::Live,
		product_config_genesis,
		boot_nodes,
		None,
		None,
		Some(properties),
		Default::default(),
	)
}

#[test]
fn test_product_chain_spec() {
	product_config().build_storage().unwrap();
}
```
* 使用配置文件启动substrate网络
> ./substrate build-spec --disable-default-bootnode --chain product > productSpec.json

> ./substrate build-spec --disable-default-bootnode --chain=productSpec.json --raw > productSpecRaw.json

* 生成node key： subkey generate-node-key

* 启动节点1
> ./substrate --base-path /tmp/node01 --chain ./productSpecRaw.json --port 30333 --ws-port 9944 --rpc-port 9933 --validator --name ProductNode01 --node-key=1

* 插入keystore
> curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params": ["babe","b","b"]}'

> curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params": ["gran","g","g"]}'

* 启动节点2、3、4
> ./substrate --base-path /tmp/node02 --chain ./productSpecRaw.json --port 30334 --ws-port 9945 --rpc-port 9934 --validator --name ProductNode02 --node-key=2 --bootnodes /ip4/xxx.xxx.xxx.xxx/tcp/30333/p2p/1

* 插入keystore
> curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":2,"method":"author_insertKey","params": ["babe","b","b"]}'

> curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":2,"method":"author_insertKey","params": ["gran","g","g"]}'

* 重启节点，开始确认最终块。