use async_trait::async_trait;
use std::sync::Arc;
use crate::file_format::FileFormatFactory;
use datafusion_session::Session;

#[async_trait]
pub trait SessionFileHandler: Send + Sync + Session {
    
    /// Retrieves a [FileFormatFactory] based on file extension which has been registered
    /// via SessionContext::register_file_format. Extensions are not case sensitive.
    fn get_file_format_factory(
        &self,
        ext: &str,
    ) -> Option<Arc<dyn FileFormatFactory>>;

}