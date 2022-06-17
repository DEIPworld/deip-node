use crate::FTImplT;
use sp_runtime::traits::Zero;

pub struct FToken<Impl: FTImplT>
where
    Self: FTokenT<Impl>,
{
    id: Impl::FTokenId,
    account: Impl::Account,
    balance: Impl::FTokenAmount,
}

pub trait FTokenT<Impl: FTImplT>: Sized {
    fn pick_ft(id: Impl::FTokenId, account: &Impl::Account) -> Option<Self>;

    fn transfer_amount(self, to: &Impl::Account, amount: Impl::FTokenAmount) -> Result<(), ()>;

    fn transfer_all(self, to: &Impl::Account) -> Result<(), ()> {
        let all = *self.balance();
        self.transfer_amount(to, all)
    }

    fn account(&self) -> &Impl::Account;

    fn balance(&self) -> &Impl::FTokenAmount;
}

impl<Impl: FTImplT> FTokenT<Impl> for FToken<Impl> {
    fn pick_ft(id: Impl::FTokenId, account: &Impl::Account) -> Option<Self> {
        let balance = Impl::balance(id, account);
        if balance.is_zero() {
            return None
        }
        Some(Self { id, account: account.clone(), balance })
    }

    fn transfer_amount(self, to: &Impl::Account, amount: Impl::FTokenAmount) -> Result<(), ()> {
        let Self { id, account, balance } = self;

        if amount > balance {
            return Err(())
        }

        Impl::transfer(id, &account, to, amount).map_err(|_| ())
    }

    fn account(&self) -> &Impl::Account {
        &self.account
    }

    fn balance(&self) -> &Impl::FTokenAmount {
        &self.balance
    }
}
