use anyhow::{bail, Result};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::{services::auth::userdata::UserData, state::AppState};

use super::errors::AppError;

pub async fn auth(
    State(state): State<AppState>,
    auth_header: TypedHeader<Authorization<Bearer>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let token = auth_header.0.token();

    let authenticator = &state.authenticator;

    let data = authenticator.authenticate(token).await?;

    match data {
        UserData::Auth0(_) => {
            req.extensions_mut().insert(data);
            Ok(next.run(req).await)
        }
    }
}
