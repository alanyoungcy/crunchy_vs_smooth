#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct VoteAccount {
    pub owner: Pubkey,
    pub crunchy: u64,
    pub smooth: u64,
    pub bump: u8,
}

impl<'info, 'entrypoint> VoteAccount {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedVoteAccount<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let crunchy = account.crunchy;
        let smooth = account.smooth;
        let bump = account.bump;

        Mutable::new(LoadedVoteAccount {
            __account__: account,
            __programs__: programs_map,
            owner,
            crunchy,
            smooth,
            bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedVoteAccount>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let crunchy = loaded.crunchy;

        loaded.__account__.crunchy = crunchy;

        let smooth = loaded.smooth;

        loaded.__account__.smooth = smooth;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;
    }
}

#[derive(Debug)]
pub struct LoadedVoteAccount<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, VoteAccount>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub crunchy: u64,
    pub smooth: u64,
    pub bump: u8,
}
