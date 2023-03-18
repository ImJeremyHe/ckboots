
pub fn main() -> Result<()> {
    


let bytes = types::load_input_data(0)?;
let frog = <types::Frog as types::OnChain>::_from_bytes(&bytes)?;


{ frog . physical -= 1 ; frog . traval_cnt += 1 ; }


let bytes = types::load_output_data(0)?;
let frog_output = <types::Frog as types::OnChain>::_from_bytes(&bytes)?;
if !frog._eq(frog_output) {
    return crate::error::Error::NotEqual;
}


    Ok(())
}
