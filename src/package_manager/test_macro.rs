#[allow(unused_macros)]
#[macro_export]
macro_rules! catch {
    ( $manager:expr, $line:literal => ()) => {
        catch!($manager, $line => )
    };

    ( $manager:expr, $line:literal => $( $package:literal ),* ) => {
        {
            let manager = $manager;
            // Create an iterator for the catches
            let mut packages = manager.catch($line).into_iter();
            $(
                let package = packages.next().unwrap();
                assert_eq!($package, package.full_command());
            )*
            assert!(packages.next().is_none());
        }
    };
}
