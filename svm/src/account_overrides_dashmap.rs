use {
    dashmap::DashMap, solana_account::AccountSharedData, solana_pubkey::Pubkey,
    solana_sdk_ids::sysvar,
};

/// Encapsulates overridden accounts, typically used for transaction
/// simulations. Account overrides are currently not used when loading the
/// durable nonce account or when constructing the instructions sysvar account.
#[derive(Default)]
pub struct AccountOverrideDashMap {
    accounts: DashMap<Pubkey, AccountSharedData>,
}

impl AccountOverrideDashMap {
    /// Insert or remove an account with a given pubkey to/from the list of overrides.
    fn set_account(&mut self, pubkey: &Pubkey, account: Option<AccountSharedData>) {
        match account {
            Some(account) => self.accounts.insert(*pubkey, account),
            None => {
                if let Some((_, v)) = self.accounts.remove(pubkey) {
                    Some(v)
                } else {
                    None
                }
            }
        };
    }

    /// Sets in the slot history
    ///
    /// Note: no checks are performed on the correctness of the contained data
    pub fn set_slot_history(&mut self, slot_history: Option<AccountSharedData>) {
        self.set_account(&sysvar::slot_history::id(), slot_history);
    }

    /// Gets the account if it's found in the list of overrides
    pub(crate) fn get(
        &self,
        pubkey: &Pubkey,
    ) -> Option<dashmap::mapref::one::Ref<'_, Pubkey, AccountSharedData>> {
        self.accounts.get(pubkey)
    }
}

#[cfg(test)]
mod test {
    use {
        crate::account_overrides_dashmap::AccountOverrideDashMap,
        solana_account::AccountSharedData, solana_pubkey::Pubkey, solana_sdk_ids::sysvar,
    };

    #[test]
    fn test_set_account() {
        let mut accounts = AccountOverrideDashMap::default();
        let data = AccountSharedData::default();
        let key = Pubkey::new_unique();
        accounts.set_account(&key, Some(data.clone()));
        assert_eq!(*accounts.get(&key).unwrap().value(), data);

        accounts.set_account(&key, None);
        assert!(accounts.get(&key).is_none());
    }

    #[test]
    fn test_slot_history() {
        let mut accounts = AccountOverrideDashMap::default();
        let data = AccountSharedData::default();

        assert_eq!(accounts.get(&sysvar::slot_history::id()).is_none(), true);
        accounts.set_slot_history(Some(data.clone()));

        assert_eq!(
            *accounts.get(&sysvar::slot_history::id()).unwrap().value(),
            data
        );
    }
}
