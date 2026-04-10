/// Contract for obtaining a fresh Tapis JWT.
///
/// Implement this trait on any type that knows how to produce or refresh a
/// Tapis access token.  Pass an `Arc<dyn TokenProvider>` to a service-client
/// constructor to enable automatic token refresh inside
/// [`RefreshMiddleware`](../client/struct.RefreshMiddleware.html).
///
/// # No-circular-dependency guarantee
///
/// `tapis-core` has **zero** HTTP or service-client dependencies.
/// `tapis-authenticator` implements `TokenProvider`; all other service crates
/// depend on `tapis-core` only. This breaks the would-be cycle:
///
/// ```text
/// tapis-jobs ──dep──> tapis-core <──impl── tapis-authenticator
///                          ▲
///                    (no dep on tapis-authenticator)
/// ```
///
/// # Infinite-loop protection
///
/// `RefreshMiddleware` checks the outgoing URL before calling `get_token()`.
/// If the URL contains `/oauth2/tokens` or `/v3/tokens` it skips refresh,
/// mirroring the tapipy guard on `create_token` / `refresh_token` operations.
///
/// # Example
///
/// ```rust,no_run
/// use std::sync::Arc;
/// use tapis_core::TokenProvider;
///
/// struct StaticToken(String);
///
/// #[async_trait::async_trait]
/// impl TokenProvider for StaticToken {
///     async fn get_token(&self) -> Option<String> {
///         Some(self.0.clone())
///     }
/// }
///
/// // Pass to any service client that accepts a TokenProvider.
/// let provider: Arc<dyn TokenProvider> = Arc::new(StaticToken("my-jwt".into()));
/// ```
#[async_trait::async_trait]
pub trait TokenProvider: Send + Sync {
    /// Return a fresh JWT string, or `None` to leave the current token
    /// on the request unchanged.
    ///
    /// Implementations should **not** panic. If the token cannot be obtained
    /// (e.g., network error), return `None` so the request proceeds with the
    /// stale token rather than crashing.
    async fn get_token(&self) -> Option<String>;
}
