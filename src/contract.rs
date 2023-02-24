pub struct ContractResult {
    // Ids of the on-chain statuses that will be read while executing this contract
    pub deps: Vec<&'static str>,
    // Ids of the on-chain statuses that will be override while executing this contract
    pub input_output_data: Vec<(&'static str, Vec<u8>, Vec<u8>)>,
    // The id of this contract
    pub contract_id: &'static str,
    pub user_input: Option<Vec<u8>>,
}
