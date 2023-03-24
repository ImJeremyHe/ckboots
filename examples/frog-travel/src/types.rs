use ckboots::create_app;
use ckboots_derives::{contract, OnChain};

#[derive(OnChain)]
#[onchain(id = "frog")]
pub struct Frog {
    #[onchain(default = 100)]
    pub physical: u8,
    pub traval_cnt: u8,
}

#[contract(Travel, id = "travel")]
fn travel(frog: &mut Frog) {
    frog.physical -= 1;
    frog.traval_cnt += 1;
}

create_app!(TravelFrog {
    types: [Frog],
    contracts: [Travel],
});

#[cfg(test)]
mod tests {
    use super::{Frog, Travel};
    use ckboots::OnChain;
    use ckboots_derives::OnChain;

    #[derive(OnChain)]
    pub struct House1 {
        pub frog: Frog,
    }

    #[derive(OnChain)]
    pub struct House2 {
        #[onchain(default = "Frog::onchain_new(90, 4)")]
        pub frog: Frog,
    }

    #[test]
    fn test_default() {
        let d = Frog::_default();
        assert_eq!(d.physical, 100);
        let h = House1::_default();
        assert_eq!(h.frog.physical, 100);
        let h = House2::_default();
        assert_eq!(h.frog.physical, 90);
        assert_eq!(h.frog.traval_cnt, 4);
    }

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
