extern crate cargo_patch_test_dependency;

#[cfg(test)]
mod tests {
    use cargo_patch_test_dependency::Container;

    #[test]
    fn it_works() {
        let container = Container(());
        assert_eq!(container, Container(()));
    }
}
