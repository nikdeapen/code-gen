use crate::{CodeBuffer, Expression, WithName, WithParams};

/// A function call.
pub struct Call {
    base: Option<Box<dyn Expression>>,
    fn_name: String,
    params: Vec<Box<dyn Expression>>,
}

impl<S: Into<String>> From<S> for Call {
    fn from(fn_name: S) -> Self {
        Self {
            base: None,
            fn_name: fn_name.into(),
            params: vec![],
        }
    }
}

impl Call {
    //! Base Expression

    /// Gets the base expression.
    pub fn base(&self) -> Option<&Box<dyn Expression>> {
        self.base.as_ref()
    }

    /// Sets the base expression.
    pub fn set_base<E>(&mut self, base: E)
    where
        E: 'static + Expression,
    {
        self.base = Some(Box::new(base));
    }

    /// Sets the base expression.
    pub fn with_base<E>(mut self, base: E) -> Self
    where
        E: 'static + Expression,
    {
        self.set_base(base);
        self
    }

    /// Writes the base expression.
    pub fn write_base(&self, b: &mut CodeBuffer) {
        if let Some(base) = &self.base {
            base.write(b);
            b.write(".");
        }
    }
}

impl WithName for Call {
    fn name(&self) -> &str {
        self.fn_name.as_str()
    }
}

impl WithParams for Call {
    fn params(&self) -> &[Box<dyn Expression>] {
        self.params.as_slice()
    }

    fn add_boxed_param(&mut self, param: Box<dyn Expression>) {
        self.params.push(param);
    }
}

impl Expression for Call {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_base(b);
        self.write_name(b);
        self.write_params_with_parens(b);
    }
}
