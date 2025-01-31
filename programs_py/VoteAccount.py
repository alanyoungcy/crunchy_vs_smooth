from seahorse.prelude import *

class VoteAccount(Account):
    owner: Pubkey
    crunchy: u64
    smooth: u64
    bump: u8