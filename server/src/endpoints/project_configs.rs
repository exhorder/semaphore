use ::actix::prelude::*;
use actix_web::{http::Method, Error, Json};
use futures::{future, Future};

use crate::actors::project::{
    GetProject, GetProjectState, GetProjectStates, GetProjectStatesResponse,
};
use crate::extractors::{CurrentServiceState, SignedJson};
use crate::service::ServiceApp;

#[allow(clippy::needless_pass_by_value)]
fn get_project_configs(
    state: CurrentServiceState,
    body: SignedJson<GetProjectStates>,
) -> ResponseFuture<Json<GetProjectStatesResponse>, Error> {
    let public_key = body.public_key;
    let futures = body.inner.projects.into_iter().map(move |project_id| {
        let public_key = public_key.clone();
        state
            .project_cache()
            .send(GetProject { id: project_id })
            .map_err(Error::from)
            .and_then(|project| project.send(GetProjectState).map_err(Error::from))
            .map(move |project_state| {
                let project_state = project_state.ok()?;
                // If public key is known (even if rate-limited, which is Some(false)), it has
                // access to the project config
                if project_state.config.trusted_relays.contains(&public_key) {
                    Some((*project_state).clone())
                } else {
                    log::debug!(
                        "Public key {} does not have access to project {}",
                        public_key,
                        project_id
                    );
                    None
                }
            })
            .map(move |project_state| (project_id, project_state))
    });

    Box::new(future::join_all(futures).map(|mut project_states| {
        Json(GetProjectStatesResponse {
            configs: project_states.drain(..).collect(),
        })
    }))
}

pub fn configure_app(app: ServiceApp) -> ServiceApp {
    app.resource("/api/0/relays/projectconfigs/", |r| {
        r.method(Method::POST).with(get_project_configs);
    })
}
