use apistos::web::{self, Scope};

pub(crate) mod onboarding;

pub(crate) fn handlers() -> Scope {
    web::scope("")
        .service(onboarding::handlers())
}
