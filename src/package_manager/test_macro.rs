#[allow(unused_macros)]
#[macro_export]
macro_rules! catch {
    ( $manager:expr, $line:literal => () ) => {
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

    ( $manager:expr, $line:literal => $( $package:literal ),+ $flags:tt ) => {
        // No idea how to reduce this duplicate code
        {
            let manager = $manager;

            // Create an iterator for the catches
            let mut packages = manager.catch($line).into_iter();
            $(
                let package = packages.next().unwrap();
                assert_eq!($package, package.name(), "Package \"{}\" should be matched, but it's not", package.full_command());

                catch!(package, $flags);
            )*
            assert!(packages.next().is_none());
        }
    };

    // Filled in the inner loop of catching packages
    ( $package:ident, [ $( $flag:literal ),* ] ) => {
        {
            // Create an iterator for the flags
            let mut flags = $package.flags().into_iter();
            $(
                let flag = flags.next().unwrap();
                assert_eq!($flag, flag, "Flag \"{}\" is missing from package \"{}\"", $flag, $package.full_command());
            )*
            assert!(flags.next().is_none());
        }
    };
}
