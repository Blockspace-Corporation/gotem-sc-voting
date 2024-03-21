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

    #[derive(Encode, Decode, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct VoterOutput {
        pub voter_id: Id,
        pub case_id: Id,
        pub voter: String,
        pub amount_hold: Balance,
        pub vote_credit: Balance,
    }

    #[derive(Encode, Decode, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct VotesOutput {
        pub vote_id: Id,
        pub case_id: Id,
        pub evidence_id: Id,
        pub voter: String,
        pub yes_credit: u8,
        pub no_credit: u8,
        pub destribution_reward: u8,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        VoterNotFound,
        VoteNotFound,
    }

    impl VoterOutput {
        fn get_voter(voter_id: Id, voter: &Voter) -> VoterOutput {
            VoterOutput {
                voter_id: voter_id.clone(),
                case_id: voter.case_id.clone(),
                voter: voter.voter.clone(),
                amount_hold: voter.amount_hold.clone(),
                vote_credit: voter.vote_credit.clone(),
            }
        }
    }

    impl VotesOutput {
        fn get_vote(vote_id: Id, vote: &Votes) -> VotesOutput {
            VotesOutput {
                vote_id: vote_id.clone(),
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
            let length: Id = (self.voter.len() as Id).checked_add(1).unwrap();
            self.voter.insert(length, voter);
        }

        #[ink(message)]
        pub fn set_vote(&mut self, vote: Votes) {
            let length: Id = (self.vote.len() as Id).checked_add(1).unwrap();
            self.vote.insert(length, vote);
        }

        #[ink(message)]
        pub fn burn_voter(&mut self, voter_id: Id) -> Result<(), Error> {
            if !self.voter.contains_key(&voter_id) {
                return Err(Error::VoterNotFound)
            };
            self.voter.remove(&voter_id);
            Ok(())
        }

        #[ink(message)]
        pub fn burn_vote(&mut self, vote_id: Id) -> Result<(), Error> {
            if !self.vote.contains_key(&vote_id) {
                return Err(Error::VoteNotFound)
            };
            self.vote.remove(&vote_id);
            Ok(())
        }

        #[ink(message)]
        pub fn update_voter(&mut self, voter_id: Id, new_voter: Voter) -> Result<(), Error> {
            let voter: &mut Voter = self
                .voter
                .get_mut(&voter_id)
                .ok_or(Error::VoterNotFound)?;
            *voter = new_voter;
            Ok(())
        }

        #[ink(message)]
        pub fn update_vote(&mut self, vote_id: Id, new_vote: Votes) -> Result<(), Error> {
            let vote: &mut Votes = self
                .vote
                .get_mut(&vote_id)
                .ok_or(Error::VoteNotFound)?;
            *vote = new_vote;
            Ok(())
        }

        #[ink(message)]
        pub fn get_all_voter(&self) -> Vec<VoterOutput> {
            let voter: Vec<VoterOutput> = self
                .voter
                .iter()
                .map(|(voter_id, voter)| VoterOutput::get_voter(*voter_id, voter))
                .collect();
            voter
        }

        #[ink(message)]
        pub fn get_voter_by_id(&self, voter_id: Id) -> Option<VoterOutput> {
            if let Some(voter) = self.voter.get(&voter_id) {
                let voter: VoterOutput = VoterOutput::get_voter(voter_id, voter);
                Some(voter)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn get_all_votes(&self) -> Vec<VotesOutput> {
            let vote: Vec<VotesOutput> = self
                .vote
                .iter()
                .map(|(vote_id, vote)| VotesOutput::get_vote(*vote_id, vote))
                .collect();
            vote
        }

        #[ink(message)]
        pub fn get_vote_by_id(&self, vote_id: Id) -> Option<VotesOutput> {
            if let Some(vote) = self.vote.get(&vote_id) {
                let vote: VotesOutput = VotesOutput::get_vote(vote_id, vote);
                Some(vote)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn votes_by_evidence_id(&self, evidence_id: Id) -> Vec<VotesOutput> {
            let votes: Vec<VotesOutput> = self
                .vote
                .iter()
                .filter_map(|(vote_id, vote)| {
                    if evidence_id == vote.evidence_id {
                        Some(VotesOutput::get_vote(*vote_id, vote))
                    } else {
                        None
                    }
                })
                .collect();
            votes
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
