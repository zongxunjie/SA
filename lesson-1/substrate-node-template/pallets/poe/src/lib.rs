#![cfg_attr(not(feature = "std"), no_std)]

// 声明外部可用
/// A modele for proof of existence
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    // 引用
    use frame_support::{dispatch::DispatchResultWithPostInfo, traits::Get, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    // 配置宏，继承系统配置
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        // 定义可配置常量
        #[pallet::constant]
        type MaxClaimValue: Get<u8>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // 存储宏
    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    // 存储类型StorageMap
    pub type Proofs<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;

    // 事件通知宏
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId,Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimTransfer(T::AccountId, Vec<u8>, T::AccountId),
    }

    // 错误类型宏
    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
        ProofNotExist,
        OverLen,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // 函数调用
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // 创建存证
        // 权重0
        #[pallet::weight(0)]
        // 两个参数发送方、key值，返回Result类型
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            // 验证签名
            let sender = ensure_signed(origin)?;

            // 验证是否超出范围
            ensure!(
                claim.len() < T::MaxClaimValue::get().into(),
                Error::<T>::OverLen
            );
            // 验证是否存在key值
            ensure!(
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofAlreadyExist
            );

            // 映射类型插入值，key和发送者，当前区块高度构成的元组
            Proofs::<T>::insert(
                &claim,
                (sender.clone(), frame_system::Pallet::<T>::block_number()),
            );

            // 触发事件
            Self::deposit_event(Event::ClaimCreated(sender, claim));
            // 返回适合的数据类型
            Ok(().into())
        }

        // 撤销
        #[pallet::weight(0)]
        pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            // 验证是否超出范围
            ensure!(
                claim.len() < T::MaxClaimValue::get().into(),
                Error::<T>::OverLen
            );

            // 验证存证是否存在，存在则获取所有者
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            // 校验发送发和数据所有者是同一人
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            // 映射类型删除元素
            Proofs::<T>::remove(&claim);

            // 触发事件通知
            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(().into())
        }

        // 转移所有权
        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
            account: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // 校验签名
            let sender = ensure_signed(origin)?;

            // 验证是否超出范围
            ensure!(
                claim.len() < T::MaxClaimValue::get().into(),
                Error::<T>::OverLen
            );

            // 校验待转移所有权的存证数据是否存在链上
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ProofNotExist);

            // 验证存证是否存在，存在则获取所有者
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            // 校验发送发和数据所有者是否同一人
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            // 映射类型，覆盖key对应的元素
            Proofs::<T>::insert(
                &claim,
                (account.clone(), frame_system::Pallet::<T>::block_number()),
            );

            // 触发事件通知
            Self::deposit_event(Event::ClaimTransfer(sender, claim, account));
            Ok(().into())
        }
    }
}
