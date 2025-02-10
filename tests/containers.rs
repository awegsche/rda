#[cfg(test)]
mod containertests {

    use rda::*;
    #[test]
    fn sanity() {
        assert_eq!(1,1);

    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}