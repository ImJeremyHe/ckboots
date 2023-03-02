use ckboots::create_app;
use ckboots_derives::{contract, OnChain};

#[derive(OnChain)]
#[onchain(id = "frog")]
pub struct Frog {
    pub physical: u8,
    pub traval_cnt: u8,
}

#[contract(Travel, id = "travel")]
fn travel(frog: &mut Frog) {
    frog.physical -= 1;
    frog.traval_cnt += 1;
}

// #[derive(CkbApp)]
// #[app(contracts(Travel), types(Frog))]
// pub struct TravelFrog {}
create_app!(TravelFrog {
    types: [Frog],
    contracts: [Travel],
});

#[cfg(test)]
mod tests {
    use super::{Frog, Travel};
    use ckboots::OnChain;

    #[test]
    fn test_new_travel_entry() {
        let frog = Frog {
            physical: 1,
            traval_cnt: 0,
        };
        let bytes = frog._to_bytes();
        let entry = Travel::new(vec![&bytes]);
        let result = entry.run();
        let (id, _, output) = result.input_output_data.iter().next().unwrap();
        assert_eq!(*id, "frog");
        let (new_frog, _) = ckboots::consume_and_decode::<Frog>(output).unwrap();
        assert_eq!(new_frog.physical, 0);
        assert_eq!(new_frog.traval_cnt, 1);
        assert_eq!(Travel::_get_args_ids(), vec!["frog"]);
    }
}
