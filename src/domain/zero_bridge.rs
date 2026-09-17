use super::anti_spaghetti::AntiSpaghettiDirective;
use super::contract_evaluator::{ContractEvaluationReceipt, InvariantContractEvaluator};

pub struct ZeroBridge;

impl ZeroBridge {
    pub fn compile_intent_to_contract(
        raw_intent: &str,
        directive: &AntiSpaghettiDirective,
    ) -> Result<String, Box<ContractEvaluationReceipt>> {
        let receipt = InvariantContractEvaluator::evaluate(raw_intent, directive);

        if receipt.dispatch_allowed {
            if let Some(contract) = receipt.contract_spec {
                return Ok(contract);
            }
        }

        Err(Box::new(receipt))
    }
}
