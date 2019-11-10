extern crate kessels;

#[cfg(test)]
mod tests{

    use super::*;
    
    #[test]
    fn trabajo_test(){
        assert_eq!(500500,kessels::work(1));
    }
}
