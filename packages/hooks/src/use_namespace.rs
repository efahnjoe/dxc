use std::collections::HashMap;

static DEFAULT_NAMESPACE: &'static str = "dxc";
static STATE_PREFIX: &'static str = "is-";

fn bem(namespace: &str, block: &str, block_suffix: &str, element: &str, modifier: &str) -> String {
    let mut cls = format! {"{}-{}", namespace ,block };

    if block_suffix != "" {
        cls = format! {"{}-{}", cls, block_suffix };
    }
    if element != "" {
        cls = format! {"{}__{}", cls, element };
    }
    if modifier != "" {
        cls = format! {"{}--{}", cls, modifier };
    }
    return cls;
}

// pub const NAMESPACE_CONTEXT_KEY: &str = "namespaceContextKey";

pub fn use_get_derived_namespace(namespace_overrides: Option<&str>) -> &str {
    match namespace_overrides {
        Some(namespace) => namespace,
        None => DEFAULT_NAMESPACE,
    }
}

pub struct UseNamespace<'a> {
    block: &'a str,
    namespace_overrides: Option<&'a str>,
}

impl<'a> UseNamespace<'a> {
    pub fn new(block: &'a str, namespace_overrides: Option<&'a str>) -> Self {
        UseNamespace {
            block,
            namespace_overrides,
        }
    }

    pub fn namespace(&self) -> &str {
        use_get_derived_namespace(self.namespace_overrides.as_deref())
    }

    pub fn b(&self) -> String {
        bem(self.namespace(), self.block, "", "", "")
    }

    pub fn b_(&self, block_suffix: &str) -> String {
        bem(self.namespace(), self.block, block_suffix, "", "")
    }

    pub fn e_(&self, element: &str) -> String {
        bem(self.namespace(), self.block, "", element, "")
    }

    pub fn m_(&self, modifier: &str) -> String {
        bem(self.namespace(), self.block, "", "", modifier)
    }

    pub fn be_(&self, block_suffix: &str, element: &str) -> String {
        bem(self.namespace(), self.block, block_suffix, element, "")
    }

    pub fn em_(&self, element: &str, modifier: &str) -> String {
        bem(self.namespace(), self.block, "", element, modifier)
    }

    pub fn bm_(&self, block_suffix: &str, modifier: &str) -> String {
        bem(self.namespace(), self.block, block_suffix, "", modifier)
    }

    pub fn bem_(&self, block_suffix: &str, element: &str, modifier: &str) -> String {
        bem(
            self.namespace(),
            self.block,
            block_suffix,
            element,
            modifier,
        )
    }

    pub fn is_(&self, name: &str, state: Option<bool>) -> String {
        match state {
            Some(true) => format!("{}{}", STATE_PREFIX, name),
            Some(false) => String::new(),
            None => name.to_string(),
        }
    }

    pub fn css_var(&self, object: &HashMap<&str, &str>) -> HashMap<String, String> {
        object
            .iter()
            .map(|(&key, &value)| {
                let css_var_name = format!("--{}-{}", self.namespace(), key);
                (css_var_name, value.to_string())
            })
            .collect()
    }

    pub fn css_var_block(&self, object: HashMap<&str, &str>) -> String {
        object
            .iter()
            .map(|(key, value)| {
                format!("--{}-{}-{}: {};", self.namespace(), self.block, key, value)
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn css_var_name(&self, name: &str) -> String {
        format!("--{}-{}", self.namespace(), name)
    }

    pub fn css_var_block_name(&self, name: &str) -> String {
        format!("--{}-{}-{}", self.namespace(), self.block, name)
    }
}
