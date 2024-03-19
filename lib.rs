#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::vote::{
    Vote,
    VoteRef,
};

#[ink::contract]
pub mod vote {
    use ink_prelude:: {
        string::String,
        vec::Vec,
        collections::BTreeMap,
    };
    use scale::{
        Decode,
        Encode,
    };

    pub type Id = u32;

    #[ink(storage)]
    pub struct Vote {
        pub voter: BTreeMap<Id, Voter>,
        pub vote: BTreeMap<Id, Votes>,
    }

    #[derive(Encode, Decode, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Voter {
        pub case_id: Id,
        pub voter: String,
        pub amount_hold: Balance,
        pub vote_credit: Balance,
    }

    #[derive(Encode, Decode, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Votes {
        pub case_id: Id,
        pub evidence_id: Id,
        pub voter: String,
        pub yes_credit: u8,
        pub no_credit: u8,
        pub destribution_reward: u8,
    }

    impl Voter {
        fn get_voter(voter: &Voter) -> Voter {
            Voter {
                case_id: voter.case_id.clone(),
                voter: voter.voter.clone(),
                amount_hold: voter.amount_hold.clone(),
                vote_credit: voter.vote_credit.clone(),
            }
        }
    }

    impl Votes {
        fn get_vote(vote: &Votes) -> Votes {
            Votes {
                case_id: vote.case_id.clone(),
                evidence_id: vote.evidence_id.clone(),
                voter: vote.voter.clone(),
                yes_credit: vote.yes_credit.clone(),
                no_credit: vote.no_credit.clone(),
                destribution_reward: vote.destribution_reward.clone(),
            }
        }
    }

    impl Vote {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {
                voter: BTreeMap::new(),
                vote: BTreeMap::new(),
            }
        }

        #[ink(message)]
        pub fn set_voter(&mut self, voter: Voter) {
            let length = (self.voter.len() as u32).checked_add(1).unwrap();
            self.voter.insert(length, voter);
        }

        #[ink(message)]
        pub fn set_vote(&mut self, vote: Votes) {
            let length = (self.vote.len() as u32).checked_add(1).unwrap();
            self.vote.insert(length, vote);
        }

        #[ink(message)]
        pub fn get_all_voter(&self) -> Vec<Voter> {
            let voter = self
                .voter
                .iter()
                .map(|(_id, voter)| Voter::get_voter(voter))
                .collect();
            voter
        }

        #[ink(message)]
        pub fn get_voter_by_id(&self, voter_id: Id) -> Option<Voter> {
            if let Some(voter) = self.voter.get(&voter_id) {
                let voter = Voter::get_voter(voter);
                Some(voter)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn get_all_votes(&self) -> Vec<Votes> {
            let vote = self
                .vote
                .iter()
                .map(|(_id, vote)| Votes::get_vote(vote))
                .collect();
            vote
        }

        #[ink(message)]
        pub fn get_vote_by_id(&self, vote_id: Id) -> Option<Votes> {
            if let Some(vote) = self.vote.get(&vote_id) {
                let vote = Votes::get_vote(vote);
                Some(vote)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn set_code(&mut self, code_hash: Hash) {
            self.env().set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!("Failed to `set_code_hash` to {code_hash:?} due to {err:?}")
            });
            ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
        }
    }
}
