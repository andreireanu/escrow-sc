#![no_std]

use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

pub type OfferId = u64;

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Offer<M: ManagedTypeApi> {
    pub creator: ManagedAddress<M>,
    pub offered_payment: EsdtTokenPayment<M>,
    pub accepted_payment: EsdtTokenPayment<M>,
    pub accepted_address: ManagedAddress<M>,
}

#[multiversx_sc::contract]
pub trait EscrowSc {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(createOffer)]
    fn create_offer(
        &self,
        accepted_token: TokenIdentifier,
        accepted_nonce: u64,
        accepted_amount: BigUint,
        accepted_address: ManagedAddress,
    ) -> OfferId {
        let payment = self.call_value().single_esdt();
        let caller = self.blockchain().get_caller();
        let new_offer_id = self.get_new_offer_id();

        let offer = Offer {
            creator: caller,
            offered_payment: payment,
            accepted_payment: EsdtTokenPayment::new(
                accepted_token,
                accepted_nonce,
                accepted_amount,
            ),
            accepted_address,
        };

        self.offers(new_offer_id).set(offer);
        new_offer_id
    }

    fn get_new_offer_id(&self) -> OfferId {
        let last_offer_id_mapper = self.last_offer_id();
        let new_offer_id = last_offer_id_mapper.get() + 1;
        last_offer_id_mapper.set(new_offer_id);
        new_offer_id
    }

    #[storage_mapper("offers")]
    fn offers(&self, id: OfferId) -> SingleValueMapper<Offer<Self::Api>>;

    #[storage_mapper("lastOfferId")]
    fn last_offer_id(&self) -> SingleValueMapper<OfferId>;
}
