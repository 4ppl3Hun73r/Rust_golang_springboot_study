mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

mod outermost2 {
    pub fn middle_function() {
        // inside::secret_function(); 
        inside::inner_function(); 
    }
    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            ::outermost2::middle_secret_function(); 
        }

        fn secret_function() {
        }
    }
}

fn try_me2() {
    outermost2::middle_function();
    // outermost2::middle_secret_function();
    outermost2::inside::inner_function();
    // outermost2::inside::secret_function();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
