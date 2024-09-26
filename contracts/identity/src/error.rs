use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("The project is closed to collaboration. Please reach out to the project owner.")]
    Unauthorized {},

    #[error("Identity already exists for this address")]
    IdentityAlreadyExists {},

    #[error("Identity not found. Create new identity before modify.")]
    IdentityNotFound {},

    #[error("Loan not found")]
    LoanNotFound {},

    #[error("Template '{template_id}' is not approved")]
    TemplateNotApproved { template_id: String },

    #[error("Field '{field_name}' should be a number")]
    InvalidNumberField { field_name: String },

    #[error("Field '{field_name}' does not match format")]
    InvalidFormat { field_name: String },

    #[error("Field '{field_name}' is missing")]
    MissingField { field_name: String },

    #[error("Template not found: {template_id}")]
    TemplateNotFound { template_id: String },

    #[error("Template '{template_id}' has already been reviewed")]
    TemplateAlreadyReviewed { template_id: String },

    #[error("Template '{template_id}' already exists for this user")]
    TemplateAlreadyExists { template_id: String },

    #[error("Invalid loan request, Fields does not match with the fields in template")]
    InvalidLoanRequest{}
}
