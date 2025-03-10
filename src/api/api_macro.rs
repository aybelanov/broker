
// Macro for applying to endpoints
#[macro_export]
macro_rules! apply_filters {
    ($service:expr, $($filter:expr),* ) => {
        {
            let mut scope = actix_web::web::scope("");
            $(
                scope = scope.wrap($filter);
            )*
            scope.service($service)
        }
    };
}