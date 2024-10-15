use cw_storage_plus::Map;

use crate::models::{Loan, Template};

// Storage for templates
pub const TEMPLATE_STORE: Map<String, Template> = Map::new("templates");

pub const LOAN_STORE: Map<String, Loan> = Map::new("loans");