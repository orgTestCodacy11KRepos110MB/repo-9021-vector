//! Utilities shared between both VRL functions.
use std::collections::BTreeMap;

use vrl::{
    diagnostic::{Label, Span},
    prelude::*,
};

use crate::{Case, Condition, IndexHandle, TableRegistry};

#[derive(Debug)]
pub enum Error {
    TablesNotLoaded,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TablesNotLoaded => write!(f, "enrichment tables not loaded"),
        }
    }
}

impl std::error::Error for Error {}

impl DiagnosticError for Error {
    fn code(&self) -> usize {
        111
    }

    fn labels(&self) -> Vec<Label> {
        match self {
            Error::TablesNotLoaded => {
                vec![Label::primary(
                    "enrichment table error: tables not loaded".to_string(),
                    Span::default(),
                )]
            }
        }
    }
}

/// Evaluates the condition object to search the enrichment tables with.
pub(crate) fn evaluate_condition<'a>(
    ctx: &mut Context,
    key: &'a str,
    value: &expression::Expr,
) -> Result<Condition<'a>> {
    let value = value.resolve(ctx)?;

    Ok(match value {
        Value::Object(map) if map.contains_key("from") && map.contains_key("to") => {
            Condition::BetweenDates {
                field: key,
                from: *map
                    .get("from")
                    .expect("should contain from")
                    .as_timestamp()
                    .ok_or("from in condition must be a timestamp")?,
                to: *map
                    .get("to")
                    .expect("should contain to")
                    .as_timestamp()
                    .ok_or("to in condition must be a timestamp")?,
            }
        }
        _ => Condition::Equals { field: key, value },
    })
}

/// Add an index for the given condition to the given enrichment table.
pub(crate) fn add_index(
    registry: &mut TableRegistry,
    tablename: &str,
    case: Case,
    condition: &BTreeMap<String, expression::Expr>,
) -> std::result::Result<IndexHandle, ExpressionError> {
    let fields = condition
        .iter()
        .filter_map(|(field, value)| match value {
            expression::Expr::Container(expression::Container {
                variant: expression::Variant::Object(map),
            }) if map.contains_key("from") && map.contains_key("to") => None,
            _ => Some(field.as_ref()),
        })
        .collect::<Vec<_>>();
    let index = registry.add_index(tablename, case, &fields)?;

    Ok(index)
}
