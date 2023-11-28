use crate::{
    ast::*,
    errors::{Error, ErrorDetails},
};

// impl Statement {
//     pub fn try_from_properties(
//         statement: Statement,
//         id: Identifier,
//         properties: Vec<Property>,
//     ) -> Result<Self, Vec<Error>> {
//         let errors: Vec<Error> = vec![];
//         match statement {
//             Statement::ServiceDef => {
//                 let mut version = format!("0.1.0");
//                 let mut operations = vec![];
//                 let mut resources = vec![];
//
//                 properties.iter().for_each(|property| {
//                     let id = property.id.name;
//                     match id {
//                         "version" => match property.value {
//                             Expression::Literal(_) => todo!(),
//                             Expression::Identifier { name } => {
//                                 errors.push(Error::TypeError(ErrorDetails {
//                                     message: format!(
//                                         "version field should be a string literal - received identifier: {name}"
//                                     ),
//                                 }))
//                             }
//                             Expression::ArrayExpression { elements } => todo!(),
//                             Expression::ObjectExpression { properties } => todo!(),
//                         },
//                     }
//                 })
//             }
//             Statement::OperationDef { id, properties } => todo!(),
//             Statement::StructDef { id, properties } => todo!(),
//             Statement::SimpleTypeDef { id, _type } => todo!(),
//         }
//     }
// }
