use crate::OnChain;

pub struct Contract {
    deps: Vec<&'static str>,
    input_output_data: Vec<InputOutputPair>,
    contract_id: &'static str,
    user_input: Option<Vec<u8>>,
}

impl Contract {
    pub fn new(contract_id: &'static str) -> Self {
        Contract {
            contract_id,
            deps: vec![],
            input_output_data: vec![],
            user_input: None,
        }
    }

    pub fn add_dep<T: OnChain>(&mut self) {
        let id = T::_id().expect("cell dep cell has no id");
        self.deps.push(id);
    }

    pub fn add_input_output<T: OnChain>(&mut self, input: T, output: T) {
        let id = T::_id().expect("on chain type should have an id");
        let value = InputOutputPair::new(id, input._to_bytes(), output._to_bytes());
        self.input_output_data.push(value);
    }

    pub fn add_user_input<T: OnChain>(&mut self, user_input: T) {
        self.user_input = Some(user_input._to_bytes());
    }

    pub fn build(mut self) {
        self.deps.sort();
        self.input_output_data.sort_by_key(|e| e.id);
    }
}

#[derive(Debug)]
struct InputOutputPair {
    input: Vec<u8>,
    output: Vec<u8>,
    id: &'static str,
}

impl InputOutputPair {
    pub fn new(id: &'static str, input: Vec<u8>, output: Vec<u8>) -> Self {
        todo!()
    }
}
