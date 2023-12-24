use thiserror::Error;
use vcard_parser::error::VcardError;
use xdg::BaseDirectoriesError;
/// Errors from the API
#[derive(Error, Debug)]
pub enum ErrorContactManager {
    #[error("from basedirectories")]
    BaseDirectory(#[from] BaseDirectoriesError),
    #[error("no vcard or found")]
    /// no vcard was not found for the search.
    Inexistant,
    #[error("vcard already exist")]
    /// The attempt to create the vcard failed as it already exist.
    AlreadyExist,
    #[error("error from Io")]
    /// An error from the filesystem occured.
    IoError(#[from] std::io::Error),
    #[error("error from Uuid, a uuid parse failed. Verify the validity of the uuid")]
    /// An error from the filesystem occured.
    Uuid(#[from] uuid::Error),
    #[error("error from vcard_parser library")]
    /// An error from the vcard_parser library occured.
    VcardError(VcardError),
}

impl From<VcardError> for ErrorContactManager {
    fn from(err: VcardError) -> Self {
        ErrorContactManager::VcardError(err)
    }
}
