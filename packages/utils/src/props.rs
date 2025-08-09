// static EP_PROP_KEY: &str = "__epPropKey";

// struct Prop {
//     r#type: String,
//     value: String,
//     validator: Option<fn(value: String) -> bool>,
//     default: Option<String>,
//     required: bool,
// }

// // pub fn is_ep_prop(prop: Prop) -> Prop {
// //     prop.r#type = "__epPropKey".to_string();
// // }

// pub fn build_prop(prop: &Prop) {
//     let prop = prop.clone();

//     match prop.validator {
//         Some(validator) => {
//             prop.value = validator;
//         }
//         None => {
//             prop.value = prop.default.clone().unwrap_or(prop.value.clone());
//         }
//     }
// }
