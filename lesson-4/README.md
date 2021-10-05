# lesson-4

## offchain_worker(OCW)

使用场景案例：
* 计算量大的工作。
* 没用绝对结果的操作：随机数、http请求（有的成功、有的失败，会影响共识）
* 需要缓存数据的计算（利用ocw单点存储）

特点：
* 不影响出块
* 能读到链上存储、不能写链上存储
* 专属存储位置，只供这节点所有链下工作机进程读写。
* 同一时间可有多个链下工作机进程在跑着

## pallet hook: offchain_worker 
hooks:
* on_initialize: 初始化区块时执行里面的逻辑
* on_finalize: 区块最终化时执行里面的逻辑
* on_runtime_upgrade: 当更新runtime逻辑时执行里面的逻辑
* offchain_worker: 区块链上逻辑运行完后，就开始调用这个函数，是链下工作机的入口。完成一次区块生成后，就会调用ocw入口fn offchain_worker(){}

三种交易方法把计算结果写回链上：
* 1. 签名交易
* 2. 不签名交易
* 3. 不签名交易但有签名数据

## 作业

* 以ocw-example为基础，把它拷到assignment目录里来修改，最后提交这个代码库。

* 利用offchain worker取出DOT当前对USD的价格，并写到一个Vec的存储里，你们自己选一种方法提交回链上，并在代码注释为什么用这种方法提交回链上最好。
只保留当前最新的10个价格，其他价格可丢弃（就是Vec的长度长到10后，这时再插入一个值时，要先丢弃最早的那个值）。

* 这个http请求可得到当前DOT价格：https://api.coincap.io/v2/assets/polkadot
```json
{
    "data": {
        "id": "polkadot",
        "rank": "9",
        "symbol": "DOT",
        "name": "Polkadot",
        "supply": "1036994797.5844200000000000",
        "maxSupply": null,
        "marketCapUsd": "32146103137.3271305225090428",
        "volumeUsd24Hr": "444810522.3132980053147750",
        "priceUsd": "30.9992906543103177",
        "changePercent24Hr": "-3.5364724514635216",
        "vwap24Hr": "31.8196219535103320",
        "explorer": "https://polkascan.io/polkadot"
    },
    "timestamp": 1633332496242
}
```

### 关键代码

* 通过http请求获取价格
```rust
	// 定义数据结构获取价格数据
	// ref: https://serde.rs/container-attrs.html#crate
	#[derive(Deserialize, Encode, Decode, Default)]
	struct PolkadotPriceInfo {
		data: PolkadotPriceData,
	}
	// ref: https://serde.rs/container-attrs.html#crate
	#[derive(Deserialize, Encode, Decode, Default)]
	struct PolkadotPriceData {
		// Specify our own deserializing function to convert JSON string to vector of bytes
		#[serde(deserialize_with = "de_string_to_bytes")]
		priceUsd: Vec<u8>,
	}
	impl fmt::Debug for PolkadotPriceData {
		// `fmt` converts the vector of bytes inside the struct back to string for
		//   more friendly display.
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(
				f,
				"{{ priceUsd: {} }}",
				str::from_utf8(&self.priceUsd).map_err(|_| fmt::Error)?
				)
		}
	}
    // 定义获取polkadot价格http url路径
	const HTTP_REMOTE_REQUEST_POLKADOT: &str = "https://api.coincap.io/v2/assets/polkadot";
    /// Fetch from remote and deserialize the JSON to a struct
    fn fetch_n_parse_polkadot() -> Result<PolkadotPriceData, Error<T>> {
        let resp_bytes = Self::fetch_from_remote_polkadot().map_err(|e| {
            log::error!("fetch_from_remote_polkadot error: {:?}", e);
            <Error<T>>::HttpFetchingError
        })?;

        let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;
        // Print out our fetched JSON string
        log::info!("{}", resp_str);

        // Deserializing JSON to struct, thanks to `serde` and `serde_derive`
        let polkadot_price_info: PolkadotPriceInfo =
        serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::HttpFetchingError)?;
        Ok(polkadot_price_info.data)
    }

    /// This function uses the `offchain::http` API to query the remote github information,
    ///   and returns the JSON response as vector of bytes.
    fn fetch_from_remote_polkadot() -> Result<Vec<u8>, Error<T>> {
        log::info!("sending request to: {}", HTTP_REMOTE_REQUEST_POLKADOT);

        // Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
        let request = rt_offchain::http::Request::get(HTTP_REMOTE_REQUEST_POLKADOT);

        // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
        let timeout = sp_io::offchain::timestamp()
        .add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));

        // For github API request, we also need to specify `user-agent` in http request header.
        //   See: https://developer.github.com/v3/#user-agent-required
        let pending = request
        .add_header("User-Agent", HTTP_HEADER_USER_AGENT)
            .deadline(timeout) // Setting the timeout time
            .send() // Sending the request out by the host
            .map_err(|_| <Error<T>>::HttpFetchingError)?;

        // By default, the http request is async from the runtime perspective. So we are asking the
        //   runtime to wait here.
        // The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
        //   ref: https://substrate.dev/rustdocs/v2.0.0/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
        let response = pending
        .try_wait(timeout)
        .map_err(|_| <Error<T>>::HttpFetchingError)?
        .map_err(|_| <Error<T>>::HttpFetchingError)?;

        if response.code != 200 {
            log::error!("Unexpected http request status code: {}", response.code);
            return Err(<Error<T>>::HttpFetchingError);
        }

        // Next we fully read the response body and collect it to a vector of bytes.
        Ok(response.body().collect::<Vec<u8>>())
    }


```

* 此次offchain交易采用不签名但签名数据的方式进行上链，签名数据用来保证数据内容完整性。
```rust
fn fetch_price_info() -> Result<(), Error<T>> {
    // 利用 offchain worker 取出 DOT 当前对 USD 的价格，并把写到一个 Vec 的存储里，
    // 你们自己选一种方法提交回链上，并在代码注释为什么用这种方法提交回链上最好。只保留当前最近的 10 个价格，
    // 其他价格可丢弃 （就是 Vec 的长度长到 10 后，这时再插入一个值时，要先丢弃最早的那个值）。

    // 取得的价格 parse 完后，放在以下存儲：
    // pub type Prices<T> = StorageValue<_, VecDeque<(u64, Permill)>, ValueQuery>

    // 这个 http 请求可得到当前 DOT 价格：
    // [https://api.coincap.io/v2/assets/polkadot](https://api.coincap.io/v2/assets/polkadot)。

    // Since off-chain storage can be accessed by off-chain workers from multiple runs, it is important to lock
    //   it before doing heavy computations or write operations.
    //
    // There are four ways of defining a lock:
    //   1) `new` - lock with default time and block exipration
    //   2) `with_deadline` - lock with default block but custom time expiration
    //   3) `with_block_deadline` - lock with default time but custom block expiration
    //   4) `with_block_and_time_deadline` - lock with custom time and block expiration
    // Here we choose the most custom one for demonstration purpose.
    let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(
        b"offchain-demo::lock", LOCK_BLOCK_EXPIRATION,
        rt_offchain::Duration::from_millis(LOCK_TIMEOUT_EXPIRATION)
        );

    // We try to acquire the lock here. If failed, we know the `fetch_n_parse` part inside is being
    //   executed by previous run of ocw, so the function just returns.
    if let Ok(_guard) = lock.try_lock() {
        match Self::fetch_n_parse_polkadot() {
            Ok(polkadot_price_data) => {
                // 不签名交易代码
                // 将价格转化string，然后按"."分割，price取整数部分，price_permill取小数前6位。
                // let s = str::from_utf8(&polkadot_price_data.priceUsd).unwrap();
                // let v: Vec<&str> = s.split(".").collect();
                // let price: u64 = v[0].parse::<u64>().unwrap();
                // let price_permill: Permill = Permill::from_parts(v[1][..6].parse::<u32>().unwrap());
                // let call = Call::submit_price_unsigned(price, price_permill);

                // // `submit_unsigned_transaction` returns a type of `Result<(), ()>`
                // //   ref: https://substrate.dev/rustdocs/v2.0.0/frame_system/offchain/struct.SubmitTransaction.html#method.submit_unsigned_transaction
                // SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
                // .map_err(|_| {
                // 	log::error!("Failed in offchain_unsigned_tx");
                // 	<Error<T>>::OffchainUnsignedTxError
                // });
                // 签名交易代码
                // Retrieve the signer to sign the payload
                let signer = Signer::<T, T::AuthorityId>::any_account();
                // 将价格转化string，然后按"."分割，price取整数部分，price_permill取小数前6位。
                let s = str::from_utf8(&polkadot_price_data.priceUsd).unwrap();
                let v: Vec<&str> = s.split(".").collect();
                let price: u64 = v[0].parse::<u64>().unwrap();
                let price_permill: Permill = Permill::from_parts(v[1][..6].parse::<u32>().unwrap());

                // `send_unsigned_transaction` is returning a type of `Option<(Account<T>, Result<(), ()>)>`.
                //   Similar to `send_signed_transaction`, they account for:
                //   - `None`: no account is available for sending transaction
                //   - `Some((account, Ok(())))`: transaction is successfully sent
                //   - `Some((account, Err(())))`: error occured when sending the transaction
                // 选择使用不签名但签名数据的方式，保证数据完整性
                if let Some((_, res)) = signer.send_unsigned_transaction(
                    |acct| PayloadPolkadot { price, price_permill, public: acct.public.clone() },
                    Call::submit_price_unsigned_with_signed_payload
                    ) {
                    return res.map_err(|_| {
                        log::error!("Failed in offchain_unsigned_tx_signed_payload");
                        <Error<T>>::OffchainUnsignedTxSignedPayloadError
                    });
                }
                // The case of `None`: no account is available for sending
                log::error!("No local account available");
            }
            Err(err) => { return Err(err); }
        }
    }

    Ok(())
}

// 交易签名内容
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct PayloadPolkadot<Public> {
    price: u64,
    price_permill: Permill,
    public: Public,
}

impl<T: SigningTypes> SignedPayload<T> for PayloadPolkadot<T::Public> {
    fn public(&self) -> T::Public {
        self.public.clone()
    }
}

/// Validate unsigned call to this module.
///
/// By default unsigned transactions are disallowed, but implementing the validator
/// here we make sure that some particular calls (the ones produced by offchain worker)
/// are being whitelisted and marked as valid.
fn validate_unsigned(_source: TransactionSource, call: &Self::Call)
-> TransactionValidity
{
    let valid_tx = |provide| ValidTransaction::with_tag_prefix("ocw-demo")
    .priority(UNSIGNED_TXS_PRIORITY)
    .and_provides([&provide])
    .longevity(3)
    .propagate(true)
    .build();

    match call {
        Call::submit_number_unsigned(_number) => valid_tx(b"submit_number_unsigned".to_vec()),
        Call::submit_number_unsigned_with_signed_payload(ref payload, ref signature) => {
            if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
                return InvalidTransaction::BadProof.into();
            }
            valid_tx(b"submit_number_unsigned_with_signed_payload".to_vec())
        },
        // 允许提交不签名交易
        Call::submit_price_unsigned(_price, _price_Permill) => valid_tx(b"submit_price_unsigned".to_vec()),
        // 允许提交不签名但签名数据内容交易
        Call::submit_price_unsigned_with_signed_payload(ref payloadPolkadot, ref signature) => {
            if !SignedPayload::<T>::verify::<T::AuthorityId>(payloadPolkadot, signature.clone()) {
                return InvalidTransaction::BadProof.into();
            }
            valid_tx(b"submit_price_unsigned_with_signed_payload".to_vec())
        },
        _ => InvalidTransaction::Call.into(),
    }
}

// 不签名交易
#[pallet::weight(10000)]
pub fn submit_price_unsigned(origin: OriginFor<T>, price: u64, price_permill: Permill) -> DispatchResult {
    let _ = ensure_none(origin)?;
    log::info!("submit_price_unsigned: {}", price);
    Self::append_or_replace_price(price, price_permill);

    Self::deposit_event(Event::NewPrice(None, (price, price_permill)));
    Ok(())
}

// 不签名但签名数据内容
#[pallet::weight(10000)]
pub fn submit_price_unsigned_with_signed_payload(origin: OriginFor<T>, payloadPolkadot: PayloadPolkadot<T::Public>,
    _signature: T::Signature) -> DispatchResult
{
    let _ = ensure_none(origin)?;
    // we don't need to verify the signature here because it has been verified in
    //   `validate_unsigned` function when sending out the unsigned tx.
    let PayloadPolkadot { price, price_permill, public } = payloadPolkadot;
    log::info!("submit_price_unsigned_with_signed_payload: ({}, {:?})", price, public);
    // 先进先出方式，只保留10个结果
    Self::append_or_replace_price(price, price_permill);

    Self::deposit_event(Event::NewPrice(None, (price, price_permill)));
    Ok(())
}

```

* 调用offchain hooks
```rust
/// Offchain Worker entry point.
///
/// By implementing `fn offchain_worker` you declare a new offchain worker.
/// This function will be called when the node is fully synced and a new best block is
/// succesfuly imported.
/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
/// so the code should be able to handle that.
/// You can use `Local Storage` API to coordinate runs of the worker.
fn offchain_worker(block_number: T::BlockNumber) {
    log::info!("Hello World from offchain workers!");

    // Here we are showcasing various techniques used when running off-chain workers (ocw)
    // 1. Sending signed transaction from ocw
    // 2. Sending unsigned transaction from ocw
    // 3. Sending unsigned transactions with signed payloads from ocw
    // 4. Fetching JSON via http requests in ocw
    // 采用10来增加请求间隔时间
    const TX_TYPES: u32 = 10;
    let modu = block_number.try_into().map_or(TX_TYPES, |bn: usize| (bn as u32) % TX_TYPES);
    let result = match modu {
        0 => Self::offchain_signed_tx(block_number),
        1 => Self::offchain_unsigned_tx(block_number),
        2 => Self::offchain_unsigned_tx_signed_payload(block_number),
        3 => Self::fetch_github_info(),
        4 => Self::fetch_price_info(),
        _ => Err(Error::<T>::UnknownOffchainMux),
    };

    if let Err(e) = result {
        log::error!("offchain_worker error: {:?}", e);
    }
}
```

### 运行截图

* 第一次获取并上链运行截图

![1.png](https://github.com/zongxunjie/SA/blob/main/lesson-4/1.png)

* 从浏览器查看

![2.png](https://github.com/zongxunjie/SA/blob/main/lesson-4/2.png)

* 有些请求没有成功

![3.png](https://github.com/zongxunjie/SA/blob/main/lesson-4/3.png)

![4.png](https://github.com/zongxunjie/SA/blob/main/lesson-4/4.png)

* 第一次记录达到10个上限

![5.png](https://github.com/zongxunjie/SA/blob/main/lesson-4/5.png)

* 第十一次写入挤掉第一次

![6.png](https://github.com/zongxunjie/SA/blob/main/lesson-4/6.png)

* 浏览器查看第十二次写入挤掉第二次

![7.png](https://github.com/zongxunjie/SA/blob/main/lesson-4/7.png)
