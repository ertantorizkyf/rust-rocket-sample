#[cfg(test)]
pub mod env_impl_test_mod {
    // api: /api/general/env-impl
    use crate::handlers::general::env_impl;

    use std::env;

    #[test]
    fn env_impl_ok() {
        // env impl success, RUST_ENV is set
        env::remove_var("RUST_ENV");
        env::set_var("RUST_ENV", "test");

        let status = &env_impl().unwrap().status;
        let message = &env_impl().unwrap().message;
        assert_eq!(status.to_owned(), "success".to_string());
        assert!(message.to_owned().contains("test"));
    }

    #[test]
    fn env_impl_nok() {
        // env impl success, RUST_ENV is not set
        env::remove_var("RUST_ENV");

        let status = &env_impl().unwrap().status;
        let message = &env_impl().unwrap().message;
        assert_eq!(status.to_owned(), "failed".to_string());
        assert!(message.to_owned().contains("UNSET"));
    }
}

#[cfg(test)]
pub mod fibonacci_test_mod {
    // api: /api/general/fibonacci
    use crate::handlers::general::fibonacci;

    #[test]
    fn fib_ok_1_seq() {
        // success calculating fib number with 1 sequence
        let fib_response = &fibonacci(1).unwrap();
        assert_eq!(fib_response.data.to_owned(), "1".to_string());
    }

    #[test]
    fn fib_ok_5_seq() {
        // success calculating fib number with 5 sequence
        let fib_response = &fibonacci(5).unwrap();
        assert_eq!(fib_response.data.to_owned(), "5".to_string());
    }

    #[test]
    fn fib_ok_50_seq() {
        // success calculating fib number with 50 sequence
        let fib_response = &fibonacci(50).unwrap();
        assert_eq!(fib_response.data.to_owned(), "12586269025".to_string());
    }
}
