//! Tapis SDK - Unified client library for Tapis services

/// Tapis Actors service client
pub mod actors {
    pub use tapis_actors::*;
}

/// Tapis Apps service client
pub mod apps {
    pub use tapis_apps::*;
}

/// Tapis Authenticator service client
pub mod authenticator {
    pub use tapis_authenticator::*;
}

/// Tapis Files service client
pub mod files {
    pub use tapis_files::*;
}

/// Tapis Globus Proxy service client
pub mod globus_proxy {
    pub use tapis_globus_proxy::*;
}

/// Tapis Jobs service client
pub mod jobs {
    pub use tapis_jobs::*;
}

/// Tapis Meta service client
pub mod meta {
    pub use tapis_meta::*;
}

/// Tapis Notifications service client
pub mod notifications {
    pub use tapis_notifications::*;
}

/// Tapis Pgrest service client
pub mod pgrest {
    pub use tapis_pgrest::*;
}

/// Tapis Pods service client
pub mod pods {
    pub use tapis_pods::*;
}

/// Tapis Security Kernel service client
pub mod sk {
    pub use tapis_sk::*;
}

/// Tapis Streams service client
pub mod streams {
    pub use tapis_streams::*;
}

/// Tapis Systems service client
pub mod systems {
    pub use tapis_systems::*;
}

/// Tapis Tenants service client
pub mod tenants {
    pub use tapis_tenants::*;
}

/// Tapis Tokens service client
pub mod tokens {
    pub use tapis_tokens::*;
}

/// Tapis Workflows service client
pub mod workflows {
    pub use tapis_workflows::*;
}
