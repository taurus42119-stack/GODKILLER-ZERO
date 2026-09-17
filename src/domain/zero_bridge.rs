use super::anti_spaghetti::AntiSpaghettiDirective;
use super::quantum_simulator::{QuantumSimulationReceipt, QuantumSuperpositionSimulator};

pub struct ZeroBridge;

impl ZeroBridge {
    pub fn compile_intent_to_contract(
        raw_intent: &str,
        directive: &AntiSpaghettiDirective,
    ) -> Result<String, Box<QuantumSimulationReceipt>> {
        let simulation_receipt =
            QuantumSuperpositionSimulator::simulate_superposition(raw_intent, directive);

        if simulation_receipt.collapse_allowed {
            if let Some(contract) = simulation_receipt.hoare_contract_spec {
                return Ok(contract);
            }
        }

        Err(Box::new(simulation_receipt))
    }
}
