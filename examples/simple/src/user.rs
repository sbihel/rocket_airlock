use rocket::{ Request, request::{FromRequest, Outcome}};
use rocket_airlock::Airlock;
use crate::hatch;


#[derive(Debug)]
pub(crate) struct User {
    pub(crate) name: String
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        match cookies.get_private("logged_in") {
            Some(logged_in) => {
                let username = logged_in.value().to_string();
                // Here you could do something else with your hatch, like checking session lifetime or other stuff.
                let hatch = request.guard::<Airlock<hatch::SimpleHatch>>()
                    .await
                    .expect("Hatch 'SimpleHatch' was not installed into the airlock.")
                    .hatch;

                if hatch.is_session_expired(&username) {
                    // If session is expired, forward user to the next route, which in this case is /login.
                    return Outcome::Forward(());
                }

                Outcome::Success(User{ name: username })
            },
            _ => Outcome::Forward(())
        }
    }
}
