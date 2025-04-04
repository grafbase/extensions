use super::Expression;

#[derive(Clone, Debug, Default)]
pub struct CaseBuilder<'a> {
    when: Vec<When<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Case<'a> {
    pub(crate) when: Vec<When<'a>>,
    pub(crate) r#else: Box<Expression<'a>>,
}

impl<'a> Case<'a> {
    pub fn builder() -> CaseBuilder<'a> {
        CaseBuilder::default()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct When<'a> {
    pub(crate) condition: Expression<'a>,
    pub(crate) result: Expression<'a>,
}

impl<'a> CaseBuilder<'a> {
    pub fn when(mut self, condition: impl Into<Expression<'a>>, result: impl Into<Expression<'a>>) -> Self {
        self.when.push(When {
            condition: condition.into(),
            result: result.into(),
        });

        self
    }

    pub fn r#else(self, expression: impl Into<Expression<'a>>) -> Case<'a> {
        Case {
            when: self.when,
            r#else: Box::new(expression.into()),
        }
    }
}
