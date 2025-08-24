use std::collections::HashMap;
use dxc_types::namespace::Block;

static DEFAULT_NAMESPACE: &str = "dxc";
static STATE_PREFIX: &str = "is-";

fn bem(namespace: String, block: Block, block_suffix: String, element: String, modifier: String) -> String {
    let mut cls = format! {"{}-{}", namespace ,block };

    if block_suffix != String::new() {
        cls = format! {"{}-{}", cls, block_suffix };
    }
    if element != String::new() {
        cls = format! {"{}__{}", cls, element };
    }
    if modifier != String::new() {
        cls = format! {"{}--{}", cls, modifier };
    }
    return cls;
}

// pub const NAMESPACE_CONTEXT_KEY: &str = "namespaceContextKey";

pub fn use_get_derived_namespace(namespace_overrides: Option<String>) -> String {
    match namespace_overrides {
        Some(namespace) => namespace,
        _ => String::from(DEFAULT_NAMESPACE),
    }
}

#[derive(Clone, Debug)]
pub struct UseNamespace {
    block: Block,
    namespace_overrides: Option<String>,
}

impl UseNamespace {
    pub fn new(block: Block, namespace_overrides: Option<String>) -> Self {
        UseNamespace {
            block,
            namespace_overrides,
        }
    }

    pub fn namespace(&self) -> String {
        use_get_derived_namespace(self.namespace_overrides.clone())
    }

    pub fn b(&self) -> String {
        bem(self.namespace(), self.block, String::new(), String::new(), String::new())
    }

    pub fn b_(&self, block_suffix: String) -> String {
        bem(self.namespace(), self.block, block_suffix, String::new(), String::new())
    }

    pub fn e_(&self, element: String) -> String {
        bem(self.namespace(), self.block, String::new(), element, String::new())
    }

    pub fn m_(&self, modifier: String) -> String {
        if modifier == String::new() {
            String::new()
        } else {
            bem(self.namespace(), self.block, String::new(), String::new(), modifier)
        }
    }

    pub fn be_(&self, block_suffix: String, element: String) -> String {
        bem(self.namespace(), self.block, block_suffix, element, String::new())
    }

    pub fn em_(&self, element: String, modifier: String) -> String {
        bem(self.namespace(), self.block, String::new(), element, modifier)
    }

    pub fn bm_(&self, block_suffix: String, modifier: String) -> String {
        bem(self.namespace(), self.block, block_suffix, String::new(), modifier)
    }

    pub fn bem_(&self, block_suffix: String, element: String, modifier: String) -> String {
        bem(
            self.namespace(),
            self.block,
            block_suffix,
            element,
            modifier,
        )
    }

    pub fn is_(&self, name: String, state: Option<bool>) -> String {
        match state {
            Some(true) => format!("{}{}", STATE_PREFIX, name),
            Some(false) => String::new(),
            None => format!("{}{}", STATE_PREFIX, name),
        }
    }

    pub fn css_var(&self, object: &HashMap<String, String>) -> HashMap<String, String> {
        object
            .iter()
            .map(|(key, value)| {
                let css_var_name = format!("--{}-{}", self.namespace(), key);
                (css_var_name, value.to_string())
            })
            .collect()
    }

    pub fn css_var_block(&self, object: HashMap<String, String>) -> String {
        object
            .iter()
            .map(|(key, value)| {
                format!("--{}-{}-{}: {};", self.namespace(), self.block, key, value)
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn css_var_name(&self, name: String) -> String {
        format!("--{}-{}", self.namespace(), name)
    }

    pub fn css_var_block_name(&self, name: String) -> String {
        format!("--{}-{}-{}", self.namespace(), self.block, name)
    }
}
