use thiserror::Error;
use vcard_parser::{error::VcardError, vcard::Vcard};
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
    #[error(transparent)]
    /// An error from the filesystem occured.
    Uuid(#[from] uuid::Error),
    #[error(transparent)]
    /// An error from the vcard_parser library occured.
    VcardError(VcardError),
    #[error("error from vcard_parser library")]
    /// An error from the vcard_parser library occured.
    Try,
    #[error("Import function can only import a file, not a directory.")]
    ImportError,
}

impl From<VcardError> for ErrorContactManager {
    fn from(err: VcardError) -> Self {
        ErrorContactManager::VcardError(err)
    }
}

pub(crate) fn error_vcard_descriptive(vcard: &Vcard) -> String {
    format!("The following vcard does not possess a valid property:\n{}\nModify manually or delete the contact and create it again\n",
                    vcard.to_string())
}
