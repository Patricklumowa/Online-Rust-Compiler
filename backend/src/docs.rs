use utoipa::OpenApi;
use crate::auth;
use crate::snippets;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::register_handler,
        auth::login_handler,
        snippets::create_snippet,
        snippets::list_snippets,
        snippets::get_snippet,
        snippets::update_snippet,
        snippets::patch_snippet,
        snippets::delete_snippet,
    ),
    components(
        schemas(
            auth::RegisterRequest,
            auth::LoginRequest,
            auth::AuthResponse,
            snippets::Snippet,
            snippets::CreateSnippetRequest,
            snippets::CreateSnippetResponse,
            snippets::UpdateSnippetRequest,
            snippets::PatchSnippetRequest,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "snippets", description = "Snippet management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
