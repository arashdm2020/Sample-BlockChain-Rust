use std::error::Error;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator: String,
    pub created_at: DateTime<Utc>,
    pub voting_start: DateTime<Utc>,
    pub voting_end: DateTime<Utc>,
    pub status: ProposalStatus,
    pub votes: HashMap<String, Vote>,
    pub required_votes: u64,
    pub budget_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Failed,
    Executed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: String,
    pub proposal_id: String,
    pub choice: VoteChoice,
    pub timestamp: DateTime<Utc>,
    pub weight: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityBudget {
    pub total_amount: f64,
    pub allocated_amount: f64,
    pub proposals: Vec<String>,
}

pub struct Governance {
    proposals: Arc<RwLock<HashMap<String, Proposal>>>,
    community_budget: Arc<RwLock<CommunityBudget>>,
    voting_power: Arc<RwLock<HashMap<String, u64>>>,
}

impl Governance {
    pub fn new() -> Self {
        Governance {
            proposals: Arc::new(RwLock::new(HashMap::new())),
            community_budget: Arc::new(RwLock::new(CommunityBudget {
                total_amount: 0.0,
                allocated_amount: 0.0,
                proposals: vec![],
            })),
            voting_power: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_proposal(&self, proposal: Proposal) -> Result<(), Box<dyn Error>> {
        let mut proposals = self.proposals.write().await;
        proposals.insert(proposal.id.clone(), proposal);
        Ok(())
    }

    pub async fn cast_vote(&self, vote: Vote) -> Result<(), Box<dyn Error>> {
        let mut proposals = self.proposals.write().await;
        if let Some(proposal) = proposals.get_mut(&vote.proposal_id) {
            proposal.votes.insert(vote.voter.clone(), vote);
            
            // Check if proposal has passed
            if self.check_proposal_status(proposal).await? {
                proposal.status = ProposalStatus::Passed;
            }
        }
        Ok(())
    }

    pub async fn check_proposal_status(&self, proposal: &Proposal) -> Result<bool, Box<dyn Error>> {
        let mut total_votes = 0;
        let mut yes_votes = 0;
        
        for vote in proposal.votes.values() {
            total_votes += vote.weight;
            if matches!(vote.choice, VoteChoice::Yes) {
                yes_votes += vote.weight;
            }
        }
        
        Ok(total_votes >= proposal.required_votes && yes_votes > total_votes / 2)
    }

    pub async fn execute_proposal(&self, proposal_id: &str) -> Result<(), Box<dyn Error>> {
        let mut proposals = self.proposals.write().await;
        let mut budget = self.community_budget.write().await;
        
        if let Some(proposal) = proposals.get_mut(proposal_id) {
            if proposal.status == ProposalStatus::Passed {
                // Update community budget
                budget.allocated_amount += proposal.budget_amount;
                budget.proposals.push(proposal_id.to_string());
                
                proposal.status = ProposalStatus::Executed;
            }
        }
        
        Ok(())
    }

    pub async fn update_voting_power(&self, user_id: &str, power: u64) -> Result<(), Box<dyn Error>> {
        let mut voting_power = self.voting_power.write().await;
        voting_power.insert(user_id.to_string(), power);
        Ok(())
    }

    pub async fn get_voting_power(&self, user_id: &str) -> Result<u64, Box<dyn Error>> {
        let voting_power = self.voting_power.read().await;
        Ok(*voting_power.get(user_id).unwrap_or(&0))
    }
}

// Protocol Improvement Proposal System
pub struct ProtocolImprovement {
    governance: Arc<Governance>,
}

impl ProtocolImprovement {
    pub fn new(governance: Arc<Governance>) -> Self {
        ProtocolImprovement { governance }
    }

    pub async fn submit_proposal(&self, proposal: Proposal) -> Result<(), Box<dyn Error>> {
        self.governance.create_proposal(proposal).await
    }

    pub async fn review_proposal(&self, proposal_id: &str) -> Result<(), Box<dyn Error>> {
        // TODO: Implement proposal review process
        Ok(())
    }
}

// Community Development Budget Management
pub struct BudgetManagement {
    governance: Arc<Governance>,
}

impl BudgetManagement {
    pub fn new(governance: Arc<Governance>) -> Self {
        BudgetManagement { governance }
    }

    pub async fn allocate_budget(&self, proposal_id: &str) -> Result<(), Box<dyn Error>> {
        self.governance.execute_proposal(proposal_id).await
    }

    pub async fn get_budget_status(&self) -> Result<CommunityBudget, Box<dyn Error>> {
        let budget = self.governance.community_budget.read().await;
        Ok(budget.clone())
    }
}

// Voting Power Calculation
pub struct VotingPowerCalculator {
    governance: Arc<Governance>,
}

impl VotingPowerCalculator {
    pub fn new(governance: Arc<Governance>) -> Self {
        VotingPowerCalculator { governance }
    }

    pub async fn calculate_voting_power(&self, user_id: &str) -> Result<u64, Box<dyn Error>> {
        // TODO: Implement voting power calculation based on:
        // 1. Token holdings
        // 2. Staking amount
        // 3. Participation in governance
        // 4. Time in the ecosystem
        Ok(100) // Placeholder
    }

    pub async fn update_user_voting_power(&self, user_id: &str) -> Result<(), Box<dyn Error>> {
        let power = self.calculate_voting_power(user_id).await?;
        self.governance.update_voting_power(user_id, power).await
    }
} 