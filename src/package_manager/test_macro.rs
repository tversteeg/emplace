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
                let package = packages.next().expect("Expected package not catched");
                assert!(package.name() == $package, "Package \"{}\" should be matched, but it's not, \"{}\" found instead", $package, package.name());
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
                let package = packages.next().expect("expected package not catched");
                assert_eq!($package, package.name(), "package \"{}\" should be matched, but it's not", package.full_command());

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
                let flag = flags.next().expect(&format!("flag \"{}\" is missing from package \"{}\"", $flag, $package.full_command()));
                assert_eq!($flag, flag);
            )*
            assert!(flags.next().is_none());
        }
    };
}
