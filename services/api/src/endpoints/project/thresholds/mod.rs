use bencher_json::{
    project::threshold::{JsonNewThreshold, JsonThreshold},
    JsonDirection, JsonPagination, ResourceId,
};
use bencher_rbac::project::Permission;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dropshot::{endpoint, HttpError, Path, Query, RequestContext, TypedBody};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    context::ApiContext,
    endpoints::{
        endpoint::{pub_response_ok, response_accepted, response_ok, ResponseAccepted, ResponseOk},
        Endpoint, Method,
    },
    error::api_error,
    model::project::{
        threshold::{InsertThreshold, QueryThreshold},
        QueryProject,
    },
    model::user::auth::AuthUser,
    schema,
    util::{
        cors::{get_cors, CorsResponse},
        error::into_json,
        same_project::SameProject,
    },
    ApiError,
};

pub mod alerts;
pub mod statistics;

use super::Resource;

const THRESHOLD_RESOURCE: Resource = Resource::Threshold;

#[derive(Deserialize, JsonSchema)]
pub struct ProjThresholdsParams {
    pub project: ResourceId,
}

pub type ProjThresholdsQuery = JsonPagination<ProjThresholdsSort, ()>;

#[derive(Clone, Copy, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProjThresholdsSort {
    #[default]
    Created,
}

#[allow(clippy::unused_async)]
#[endpoint {
    method = OPTIONS,
    path =  "/v0/projects/{project}/thresholds",
    tags = ["projects", "thresholds"]
}]
pub async fn proj_thresholds_options(
    _rqctx: RequestContext<ApiContext>,
    _path_params: Path<ProjThresholdsParams>,
    _query_params: Query<ProjThresholdsQuery>,
) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<ApiContext>())
}

#[endpoint {
    method = GET,
    path =  "/v0/projects/{project}/thresholds",
    tags = ["projects", "thresholds"]
}]
pub async fn proj_thresholds_get(
    rqctx: RequestContext<ApiContext>,
    path_params: Path<ProjThresholdsParams>,
    query_params: Query<ProjThresholdsQuery>,
) -> Result<ResponseOk<Vec<JsonThreshold>>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await.ok();
    let endpoint = Endpoint::new(THRESHOLD_RESOURCE, Method::GetLs);

    let json = get_ls_inner(
        rqctx.context(),
        auth_user.as_ref(),
        path_params.into_inner(),
        query_params.into_inner(),
        endpoint,
    )
    .await
    .map_err(|e| endpoint.err(e))?;

    if auth_user.is_some() {
        response_ok!(endpoint, json)
    } else {
        pub_response_ok!(endpoint, json)
    }
}

async fn get_ls_inner(
    context: &ApiContext,
    auth_user: Option<&AuthUser>,
    path_params: ProjThresholdsParams,
    query_params: ProjThresholdsQuery,
    endpoint: Endpoint,
) -> Result<Vec<JsonThreshold>, ApiError> {
    let conn = &mut *context.conn().await;

    let query_project =
        QueryProject::is_allowed_public(conn, &context.rbac, &path_params.project, auth_user)?;

    let mut query = schema::threshold::table
        .filter(schema::threshold::project_id.eq(query_project.id))
        .into_boxed();

    query = match query_params.order() {
        ProjThresholdsSort::Created => match query_params.direction {
            Some(JsonDirection::Asc) => query.order(schema::threshold::created.asc()),
            Some(JsonDirection::Desc) | None => query.order(schema::threshold::created.desc()),
        },
    };

    Ok(query
        .offset(query_params.offset())
        .limit(query_params.limit())
        .load::<QueryThreshold>(conn)
        .map_err(api_error!())?
        .into_iter()
        .filter_map(into_json!(endpoint, conn))
        .collect())
}

#[endpoint {
    method = POST,
    path =  "/v0/projects/{project}/thresholds",
    tags = ["projects", "thresholds"]
}]
pub async fn proj_threshold_post(
    rqctx: RequestContext<ApiContext>,
    path_params: Path<ProjThresholdsParams>,
    body: TypedBody<JsonNewThreshold>,
) -> Result<ResponseAccepted<JsonThreshold>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(THRESHOLD_RESOURCE, Method::Post);

    let json = post_inner(
        rqctx.context(),
        path_params.into_inner(),
        &body.into_inner(),
        &auth_user,
    )
    .await
    .map_err(|e| endpoint.err(e))?;

    response_accepted!(endpoint, json)
}

async fn post_inner(
    context: &ApiContext,
    path_params: ProjThresholdsParams,
    json_threshold: &JsonNewThreshold,
    auth_user: &AuthUser,
) -> Result<JsonThreshold, ApiError> {
    let conn = &mut *context.conn().await;

    // Verify that the branch and testbed are part of the same project
    let SameProject {
        project_id,
        branch_id,
        testbed_id,
    } = SameProject::validate(
        conn,
        &path_params.project,
        &json_threshold.branch,
        &json_threshold.testbed,
    )?;

    // Verify that the user is allowed
    QueryProject::is_allowed_id(
        conn,
        &context.rbac,
        project_id,
        auth_user,
        Permission::Create,
    )?;

    let insert_threshold =
        InsertThreshold::from_json(conn, project_id, branch_id, testbed_id, json_threshold)?;
    diesel::insert_into(schema::threshold::table)
        .values(&insert_threshold)
        .execute(conn)
        .map_err(api_error!())?;

    schema::threshold::table
        .filter(schema::threshold::uuid.eq(&insert_threshold.uuid))
        .first::<QueryThreshold>(conn)
        .map_err(api_error!())?
        .into_json(conn)
}

#[derive(Deserialize, JsonSchema)]
pub struct ProjThresholdParams {
    pub project: ResourceId,
    pub threshold: Uuid,
}

#[allow(clippy::unused_async)]
#[endpoint {
    method = OPTIONS,
    path =  "/v0/projects/{project}/thresholds/{threshold}",
    tags = ["projects", "thresholds"]
}]
pub async fn proj_threshold_options(
    _rqctx: RequestContext<ApiContext>,
    _path_params: Path<ProjThresholdParams>,
) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<ApiContext>())
}

#[endpoint {
    method = GET,
    path =  "/v0/projects/{project}/thresholds/{threshold}",
    tags = ["projects", "thresholds"]
}]
pub async fn proj_threshold_get(
    rqctx: RequestContext<ApiContext>,
    path_params: Path<ProjThresholdParams>,
) -> Result<ResponseOk<JsonThreshold>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await.ok();
    let endpoint = Endpoint::new(THRESHOLD_RESOURCE, Method::GetOne);

    let json = get_one_inner(
        rqctx.context(),
        path_params.into_inner(),
        auth_user.as_ref(),
    )
    .await
    .map_err(|e| endpoint.err(e))?;

    if auth_user.is_some() {
        response_ok!(endpoint, json)
    } else {
        pub_response_ok!(endpoint, json)
    }
}

async fn get_one_inner(
    context: &ApiContext,
    path_params: ProjThresholdParams,
    auth_user: Option<&AuthUser>,
) -> Result<JsonThreshold, ApiError> {
    let conn = &mut *context.conn().await;

    let query_project =
        QueryProject::is_allowed_public(conn, &context.rbac, &path_params.project, auth_user)?;

    schema::threshold::table
        .filter(schema::threshold::project_id.eq(query_project.id))
        .filter(schema::threshold::uuid.eq(path_params.threshold.to_string()))
        .select((
            schema::threshold::id,
            schema::threshold::uuid,
            schema::threshold::project_id,
            schema::threshold::metric_kind_id,
            schema::threshold::branch_id,
            schema::threshold::testbed_id,
            schema::threshold::statistic_id,
            schema::threshold::created,
            schema::threshold::modified,
        ))
        .first::<QueryThreshold>(conn)
        .map_err(api_error!())?
        .into_json(conn)
}
