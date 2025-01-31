#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::dot::VoteAccount::{LoadedVoteAccount, VoteAccount};
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

pub fn init_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut voter: Empty<Mutable<LoadedVoteAccount<'info, '_>>>,
    mut vote_account_bump: u8,
) -> () {
    let mut init_voter = voter.account.clone();

    assign!(init_voter.borrow_mut().owner, owner.key());

    assign!(init_voter.borrow_mut().bump, vote_account_bump);
}

pub fn vote_crunchy_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut vote: Mutable<LoadedVoteAccount<'info, '_>>,
) -> () {
    if !(owner.key() == vote.borrow().owner) {
        panic!("This is not your Vote account!");
    }

    assign!(vote.borrow_mut().crunchy, vote.borrow().crunchy + 1);
}

pub fn vote_smooth_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut vote: Mutable<LoadedVoteAccount<'info, '_>>,
) -> () {
    if !(owner.key() == vote.borrow().owner) {
        panic!("This is not your Vote account!");
    }

    assign!(vote.borrow_mut().smooth, vote.borrow().smooth + 1);
}
