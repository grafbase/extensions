use core::fmt;
use std::{borrow::Cow, fmt::Write};

use grafbase_sdk::host_io::postgres::{self as sdk, types::DatabaseType};

use crate::ast::{
    self, Alias, Average, Column, CommonTableExpression, Compare, Concat, ConditionTree, Delete, Encode, EncodeFormat,
    Expression, ExpressionKind, Function, FunctionType, Grouping, Insert, Join, JoinData, JsonAgg, JsonBuildArray,
    JsonBuildObject, JsonCompare, JsonExtract, JsonExtractArrayElem, JsonType, JsonUnquote, OnConflict, Order,
    Ordering, ParameterizedValue, Query, Replace, Row, Select, SqlOp, Table, TableType, ToJsonb, Update, Values,
};

const C_BACKTICK_OPEN: &str = "\"";
const C_BACKTICK_CLOSE: &str = "\"";

const C_QUOTE: &str = "'";

/// Renders an AST node into a Postgres query.
///
/// Takes an AST node that can be converted into a `Query` and renders it into a Postgres query
/// using the visitor pattern. The resulting query includes both the SQL string and any bound
/// parameters.
///
/// # Returns
///
/// A finalized Postgres query that can be executed against a database.
pub fn render<'a>(ast: impl Into<ast::Query<'a>>) -> sdk::Query {
    let mut renderer = Renderer::default();
    renderer.visit_query(ast.into());

    renderer.builder.finalize()
}

#[derive(Debug, Default)]
struct Renderer {
    builder: sdk::QueryBuilder,
}

impl Renderer {
    fn write<D: fmt::Display>(&mut self, s: D) {
        self.builder
            .write_fmt(format_args!("{s}"))
            .expect("we ran out of memory or something else why write failed");
    }

    fn substitute_parameter(&mut self, parameter: impl DatabaseType) {
        self.substitute_value(ParameterizedValue {
            value: parameter.into_bound_value(0),
            enum_type: None,
        });
    }

    fn substitute_value(&mut self, value: ParameterizedValue<'_>) {
        let cast = {
            let cast = value.enum_type.or(value.value.type_cast().map(Cow::Borrowed));
            cast.map(|cast| format!("::{}", cast))
        };

        self.builder.bind_value(value.value);
        self.write("$");
        self.write(self.builder.bound_values());

        if let Some(ref cast) = cast {
            self.write(cast);
        }
    }

    fn visit_limit_and_offset(&mut self, limit: Option<u32>, offset: Option<u32>) {
        match (limit, offset) {
            (Some(limit), Some(offset)) => {
                self.write(" LIMIT ");
                self.substitute_parameter(limit);

                self.write(" OFFSET ");
                self.substitute_parameter(offset);
            }
            (None, Some(offset)) => {
                self.write(" OFFSET ");
                self.substitute_parameter(offset);
            }
            (Some(limit), None) => {
                self.write(" LIMIT ");
                self.substitute_parameter(limit);
            }
            (None, None) => (),
        }
    }

    fn visit_insert(&mut self, insert: Insert<'_>) {
        self.write("INSERT ");

        if let Some(table) = insert.table.clone() {
            self.write("INTO ");
            self.visit_table(table, true);
        }

        match insert.values {
            Expression {
                kind: ExpressionKind::Row(row),
                ..
            } => {
                if row.values.is_empty() {
                    self.write(" DEFAULT VALUES");
                } else {
                    let columns = insert.columns.len();

                    self.write(" (");
                    for (i, c) in insert.columns.into_iter().enumerate() {
                        self.visit_column(c.name.into_owned().into());

                        if i < (columns - 1) {
                            self.write(",");
                        }
                    }

                    self.write(")");
                    self.write(" VALUES ");
                    self.visit_row(row);
                }
            }
            Expression {
                kind: ExpressionKind::Values(values),
                ..
            } => {
                let columns = insert.columns.len();

                self.write(" (");
                for (i, c) in insert.columns.into_iter().enumerate() {
                    self.visit_column(c.name.into_owned().into());

                    if i < (columns - 1) {
                        self.write(",");
                    }
                }

                self.write(")");
                self.write(" VALUES ");
                let values_len = values.len();

                for (i, row) in values.into_iter().enumerate() {
                    self.visit_row(row);

                    if i < (values_len - 1) {
                        self.write(", ");
                    }
                }
            }
            expr => self.surround_with("(", ")", |ref mut s| s.visit_expression(expr)),
        }

        match insert.on_conflict {
            Some(OnConflict::DoNothing) => self.write(" ON CONFLICT DO NOTHING"),
            Some(OnConflict::Update(update, constraints)) => {
                self.write(" ON CONFLICT");
                self.columns_to_bracket_list(constraints);
                self.write(" DO ");

                self.visit_upsert(update);
            }
            None => (),
        }

        if let Some(returning) = insert.returning {
            if !returning.is_empty() {
                let values = returning.into_iter().map(|r| r.into()).collect();
                self.write(" RETURNING ");
                self.visit_columns(values);
            }
        };
    }

    fn visit_delete(&mut self, delete: Delete<'_>) {
        self.write("DELETE FROM ");
        self.visit_table(delete.table, true);

        if let Some(conditions) = delete.conditions {
            self.write(" WHERE ");
            self.visit_conditions(conditions);
        }

        if let Some(returning) = delete.returning {
            self.write(" RETURNING ");

            let length = returning.len();

            for (i, expression) in returning.into_iter().enumerate() {
                self.visit_expression(expression);

                if i < (length - 1) {
                    self.write(", ");
                }
            }
        }
    }

    fn visit_aggregate_to_string(&mut self, value: Expression<'_>) {
        self.write("ARRAY_TO_STRING");
        self.write("(");
        self.write("ARRAY_AGG");
        self.write("(");
        self.visit_expression(value);
        self.write(")");
        self.write("','");
        self.write(")")
    }

    fn visit_json_extract(&mut self, json_extract: JsonExtract<'_>) {
        let json_path = json_extract.path;

        self.write("(");
        self.visit_expression(*json_extract.column);

        if json_extract.extract_as_string {
            self.write("#>>");
        } else {
            self.write("#>");
        }

        // We use the `ARRAY[]::text[]` notation to better handle escaped character
        // The text protocol used when sending prepared statement doesn't seem to work well with escaped characters
        // when using the '{a, b, c}' string array notation.
        self.surround_with("ARRAY[", "]::text[]", |s| {
            let len = json_path.len();

            for (index, path) in json_path.into_iter().enumerate() {
                s.substitute_parameter(path.to_string());

                if index < len - 1 {
                    s.write(", ");
                }
            }
        });

        self.write(")");

        if !json_extract.extract_as_string {
            self.write("::jsonb");
        }
    }

    fn visit_json_unquote(&mut self, json_unquote: JsonUnquote<'_>) {
        self.write("(");
        self.visit_expression(*json_unquote.expr);
        self.write("#>>ARRAY[]::text[]");
        self.write(")");
    }

    fn visit_array_contains(&mut self, left: Expression<'_>, right: Expression<'_>, not: bool) {
        if not {
            self.write("( NOT ");
        }

        self.visit_expression(left);
        self.write(" @> ");
        self.visit_expression(right);

        if not {
            self.write(" )");
        }
    }

    fn visit_array_contained(&mut self, left: Expression<'_>, right: Expression<'_>, not: bool) {
        if not {
            self.write("( NOT ");
        }

        self.visit_expression(left);
        self.write(" <@ ");
        self.visit_expression(right);

        if not {
            self.write(" )");
        }
    }

    fn visit_array_overlaps(&mut self, left: Expression<'_>, right: Expression<'_>) {
        self.visit_expression(left);
        self.write(" && ");
        self.visit_expression(right);
    }

    fn visit_json_extract_array_item(&mut self, extract: JsonExtractArrayElem<'_>) {
        self.write("(");
        self.visit_expression(*extract.expr);
        self.write("->>");
        self.write(extract.index);
        self.write(")");
    }

    fn visit_json_type_equals(&mut self, left: Expression<'_>, json_type: JsonType<'_>, not: bool) {
        self.write("JSONB_TYPEOF");
        self.write("(");
        self.visit_expression(left);
        self.write(")");

        if not {
            self.write(" != ");
        } else {
            self.write(" = ");
        }

        match json_type {
            JsonType::Array => self.visit_expression(Expression {
                kind: ExpressionKind::Parameterized(ParameterizedValue {
                    value: "array".to_string().into_bound_value(0),
                    enum_type: None,
                }),
                alias: None,
            }),
            JsonType::Boolean => self.visit_expression(Expression {
                kind: ExpressionKind::Parameterized(ParameterizedValue {
                    value: "boolean".to_string().into_bound_value(0),
                    enum_type: None,
                }),
                alias: None,
            }),
            JsonType::Number => self.visit_expression(Expression {
                kind: ExpressionKind::Parameterized(ParameterizedValue {
                    value: "number".to_string().into_bound_value(0),
                    enum_type: None,
                }),
                alias: None,
            }),
            JsonType::Object => self.visit_expression(Expression {
                kind: ExpressionKind::Parameterized(ParameterizedValue {
                    value: "object".to_string().into_bound_value(0),
                    enum_type: None,
                }),
                alias: None,
            }),
            JsonType::String => self.visit_expression(Expression {
                kind: ExpressionKind::Parameterized(ParameterizedValue {
                    value: "string".to_string().into_bound_value(0),
                    enum_type: None,
                }),
                alias: None,
            }),
            JsonType::Null => self.visit_expression(Expression {
                kind: ExpressionKind::Parameterized(ParameterizedValue {
                    value: "null".to_string().into_bound_value(0),
                    enum_type: None,
                }),
                alias: None,
            }),
            JsonType::ColumnRef(column) => {
                self.write("JSONB_TYPEOF");
                self.write("(");
                self.visit_column(*column);
                self.write("::jsonb)")
            }
        }
    }

    fn visit_like(&mut self, left: Expression<'_>, right: Expression<'_>) {
        let need_cast = matches!(&left.kind, ExpressionKind::Column(_));
        self.visit_expression(left);

        // NOTE: Pg is strongly typed, LIKE comparisons are only between strings.
        // to avoid problems with types without implicit casting we explicitly cast to text
        if need_cast {
            self.write("::text");
        }

        self.write(" LIKE ");
        self.visit_expression(right);
    }

    fn visit_not_like(&mut self, left: Expression<'_>, right: Expression<'_>) {
        let need_cast = matches!(&left.kind, ExpressionKind::Column(_));
        self.visit_expression(left);

        // NOTE: Pg is strongly typed, LIKE comparisons are only between strings.
        // to avoid problems with types without implicit casting we explicitly cast to text
        if need_cast {
            self.write("::text");
        }

        self.write(" NOT LIKE ");
        self.visit_expression(right);
    }

    fn visit_ordering(&mut self, ordering: Ordering<'_>) {
        let len = ordering.0.len();

        for (i, (value, ordering)) in ordering.0.into_iter().enumerate() {
            let direction = ordering.map(|dir| match dir {
                Order::Asc => " ASC",
                Order::Desc => " DESC",
                Order::AscNullsFirst => " ASC NULLS FIRST",
                Order::AscNullsLast => " ASC NULLS LAST",
                Order::DescNullsFirst => " DESC NULLS FIRST",
                Order::DescNullsLast => " DESC NULLS LAST",
            });

            self.visit_expression(value);
            self.write(direction.unwrap_or(""));

            if i < (len - 1) {
                self.write(", ");
            }
        }
    }

    fn visit_concat(&mut self, concat: Concat<'_>) {
        let len = concat.exprs.len();

        self.surround_with("(", ")", |s| {
            for (i, expr) in concat.exprs.into_iter().enumerate() {
                s.visit_expression(expr);

                if i < (len - 1) {
                    s.write(" || ");
                }
            }
        });
    }

    fn visit_to_jsonb(&mut self, to_jsonb: ToJsonb<'_>) {
        self.write("to_jsonb(");
        self.visit_table(to_jsonb.table, false);
        self.write(".*)");
    }

    fn visit_json_build_object(&mut self, json_build_object: JsonBuildObject<'_>) {
        let values_length = json_build_object.values.len();
        self.write("json_build_object(");

        for (i, (name, expression)) in json_build_object.values.into_iter().enumerate() {
            self.surround_with("'", "'", |renderer| {
                renderer.write(&name);
            });

            self.write(", ");
            self.visit_expression(expression);

            if i < (values_length - 1) {
                self.write(",");
            }
        }

        self.write(")");
    }

    fn visit_json_build_array(&mut self, json_build_array: JsonBuildArray<'_>) {
        self.write("json_build_array(");

        let values_length = json_build_array.expressions.len();
        for (i, expression) in json_build_array.expressions.into_iter().enumerate() {
            self.visit_expression(expression);

            if i < (values_length - 1) {
                self.write(",");
            }
        }

        self.write(")");
    }

    fn visit_json_agg(&mut self, json_agg: JsonAgg<'_>) {
        self.write("json_agg(");

        if json_agg.distinct {
            self.write("DISTINCT ");
        }

        self.visit_expression(json_agg.expression);

        if let Some(ordering) = json_agg.order_by {
            self.write(" ORDER BY ");
            self.visit_ordering(ordering);
        }

        self.write(")");
    }

    fn visit_encode(&mut self, encode: Encode<'_>) {
        self.write("encode");

        self.surround_with("(", ")", |s| {
            s.visit_expression(encode.expression);
            s.write(", ");

            match encode.format {
                EncodeFormat::Base64 => s.write("'base64'"),
                EncodeFormat::Escape => s.write("'escape'"),
                EncodeFormat::Hex => s.write("'hex'"),
            }
        });
    }

    fn visit_replace(&mut self, replace: Replace<'_>) {
        self.write("replace");

        self.surround_with("(", ")", |s| {
            s.visit_expression(*replace.expression);
            s.write(", ");

            s.visit_sql_string_pattern(replace.old_value);
            s.write(", ");

            s.surround_with("'", "'", |s| {
                s.write(replace.new_value);
            });
        });
    }

    fn visit_sql_string_pattern(&mut self, pattern: ast::SqlStringPattern<'_>) {
        match pattern {
            ast::SqlStringPattern::Literal(literal) => self.surround_with("'", "'", |s| {
                s.write(literal);
            }),
            ast::SqlStringPattern::EscapedContent(content) => self.surround_with("E'", "'", |s| {
                s.write(content);
            }),
        }
    }

    fn visit_decode(&mut self, encode: ast::Decode<'_>) {
        self.write("decode(");
        self.visit_expression(encode.expression);
        self.write(", ");

        match encode.format {
            EncodeFormat::Base64 => self.write("'base64'"),
            EncodeFormat::Escape => self.write("'escape'"),
            EncodeFormat::Hex => self.write("'hex'"),
        }

        self.write(")");
    }

    fn visit_join_data(&mut self, data: JoinData<'_>, empty_on: bool) {
        if data.lateral {
            self.write(" LATERAL ");
        }

        self.visit_table(data.table, true);
        if !data.conditions.is_no_condition() || empty_on {
            self.write(" ON ");
            self.visit_conditions(data.conditions)
        }
    }

    fn surround_with<F>(&mut self, begin: &str, end: &str, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.write(begin);
        f(self);
        self.write(end)
    }

    fn columns_to_bracket_list(&mut self, columns: Vec<Column<'_>>) {
        let len = columns.len();

        self.write(" (");
        for (i, c) in columns.into_iter().enumerate() {
            self.visit_column(c.name.into_owned().into());

            if i < (len - 1) {
                self.write(",");
            }
        }
        self.write(")");
    }

    /// The join statements in the query
    fn visit_joins(&mut self, joins: Vec<Join<'_>>) {
        for join in joins {
            match join {
                Join::Inner(data) => {
                    self.write(" INNER JOIN ");
                    self.visit_join_data(data, true);
                }
                Join::Left(data) => {
                    self.write(" LEFT JOIN ");
                    self.visit_join_data(data, true);
                }
                Join::Right(data) => {
                    self.write(" RIGHT JOIN ");
                    self.visit_join_data(data, true);
                }
                Join::Full(data) => {
                    self.write(" FULL JOIN ");
                    self.visit_join_data(data, true);
                }
                Join::Cross(data) => {
                    self.write(" CROSS JOIN ");
                    self.visit_join_data(data, false);
                }
            }
        }
    }

    fn visit_common_table_expression(&mut self, cte: CommonTableExpression<'_>) {
        self.visit_table(Table::from(cte.name.into_owned()), false);
        self.write(" AS ");

        let query = cte.query;
        self.surround_with("(", ")", |ref mut s| s.visit_query(query));
    }

    /// A walk through a `SELECT` statement
    fn visit_select(&mut self, select: Select<'_>) {
        let number_of_ctes = select.ctes.len();

        if number_of_ctes > 0 {
            self.write("WITH ");

            for (i, cte) in select.ctes.into_iter().enumerate() {
                self.visit_common_table_expression(cte);

                if i < (number_of_ctes - 1) {
                    self.write(", ");
                }
            }

            self.write(" ");
        }

        self.write("SELECT ");

        if select.distinct {
            self.write("DISTINCT ");
        }

        if !select.tables.is_empty() {
            if select.columns.is_empty() {
                for (i, table) in select.tables.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }

                    match &table.typ {
                        TableType::Query(_) | TableType::Values(_) => match table.alias {
                            Some(ref alias) => {
                                self.surround_with(C_BACKTICK_OPEN, C_BACKTICK_CLOSE, |ref mut s| s.write(&alias.name));
                                self.write(".*");
                            }
                            None => self.write("*"),
                        },
                        TableType::Table(_) => match table.alias.clone() {
                            Some(ref alias) => {
                                self.surround_with(C_BACKTICK_OPEN, C_BACKTICK_CLOSE, |ref mut s| s.write(&alias.name));
                                self.write(".*");
                            }
                            None => {
                                self.visit_table(table.clone(), false);
                                self.write(".*");
                            }
                        },
                        TableType::JoinedTable(jt) => match table.alias.clone() {
                            Some(ref alias) => {
                                self.surround_with(C_BACKTICK_OPEN, C_BACKTICK_CLOSE, |ref mut s| s.write(&alias.name));
                                self.write(".*");
                            }
                            None => {
                                let mut unjoined_table = table.clone();
                                // Convert the table typ to a `TableType::Table` for the SELECT statement print
                                // We only want the join to appear in the FROM clause
                                unjoined_table.typ = TableType::Table(jt.0.clone());

                                self.visit_table(unjoined_table, false);
                                self.write(".*");
                            }
                        },
                        TableType::Expression(_) => {
                            todo!("hmm");
                        }
                    }
                }
            } else {
                self.visit_columns(select.columns);
            }

            self.write(" FROM ");

            for (i, table) in select.tables.into_iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }

                self.visit_table(table, true);
            }

            if !select.joins.is_empty() {
                self.visit_joins(select.joins);
            }

            if let Some(conditions) = select.conditions {
                self.write(" WHERE ");
                self.visit_conditions(conditions);
            }
            if !select.grouping.is_empty() {
                self.write(" GROUP BY ");
                self.visit_grouping(select.grouping);
            }
            if let Some(conditions) = select.having {
                self.write(" HAVING ");
                self.visit_conditions(conditions);
            }
            if !select.ordering.is_empty() {
                self.write(" ORDER BY ");
                self.visit_ordering(select.ordering);
            }

            self.visit_limit_and_offset(select.limit, select.offset);
        } else if select.columns.is_empty() {
            self.write(" *");
        } else {
            self.visit_columns(select.columns);
        }
    }

    /// A walk through an `UPDATE` statement
    fn visit_update(&mut self, update: Update<'_>) {
        self.write("UPDATE ");
        self.visit_table(update.table, true);

        {
            self.write(" SET ");
            let pairs = update.columns.into_iter().zip(update.values);
            let len = pairs.len();

            for (i, (key, value)) in pairs.enumerate() {
                self.visit_column(key);
                self.write(" = ");
                self.visit_expression(value);

                if i < (len - 1) {
                    self.write(", ");
                }
            }
        }

        if let Some(conditions) = update.conditions {
            self.write(" WHERE ");
            self.visit_conditions(conditions);
        }

        if let Some(returning) = update.returning {
            if !returning.is_empty() {
                let values = returning.into_iter().map(|r| r.into()).collect();
                self.write(" RETURNING ");
                self.visit_columns(values);
            }
        }
    }

    fn visit_upsert(&mut self, update: Update<'_>) {
        self.write("UPDATE ");

        self.write("SET ");
        self.visit_update_set(update.clone());

        if let Some(conditions) = update.conditions {
            self.write(" WHERE ");
            self.visit_conditions(conditions);
        }
    }

    fn visit_update_set(&mut self, update: Update<'_>) {
        let pairs = update.columns.into_iter().zip(update.values);
        let len = pairs.len();

        for (i, (key, value)) in pairs.enumerate() {
            self.visit_column(key);
            self.write(" = ");
            self.visit_expression(value);

            if i < (len - 1) {
                self.write(", ");
            }
        }
    }

    /// A helper for delimiting an identifier, surrounding every part with `C_BACKTICK`
    /// and delimiting the values with a `.`
    fn delimited_identifiers<'a>(&mut self, parts: impl ExactSizeIterator<Item = &'a str>) {
        let len = parts.len();

        for (i, part) in parts.enumerate() {
            self.surround_with_backticks(part);

            if i < (len - 1) {
                self.write(".");
            }
        }
    }

    /// A helper for delimiting a part of an identifier, surrounding it with `C_BACKTICK`
    fn surround_with_backticks(&mut self, part: &str) {
        self.surround_with(C_BACKTICK_OPEN, C_BACKTICK_CLOSE, |ref mut s| s.write(part));
    }

    /// A walk through a complete `Query` statement
    fn visit_query(&mut self, query: Query<'_>) {
        match query {
            Query::Select(select) => self.visit_select(*select),
            Query::Insert(insert) => self.visit_insert(*insert),
            Query::Update(update) => self.visit_update(*update),
            Query::Delete(delete) => self.visit_delete(*delete),
        }
    }

    /// The selected columns
    fn visit_columns(&mut self, columns: Vec<Expression<'_>>) {
        let len = columns.len();

        for (i, column) in columns.into_iter().enumerate() {
            self.visit_expression(column);

            if i < (len - 1) {
                self.write(", ");
            }
        }
    }

    fn visit_operation(&mut self, op: SqlOp<'_>) {
        match op {
            SqlOp::Add(left, right) => self.surround_with("(", ")", |ref mut se| {
                se.visit_expression(left);
                se.write(" + ");
                se.visit_expression(right)
            }),
            SqlOp::Sub(left, right) => self.surround_with("(", ")", |ref mut se| {
                se.visit_expression(left);
                se.write(" - ");
                se.visit_expression(right)
            }),
            SqlOp::Mul(left, right) => self.surround_with("(", ")", |ref mut se| {
                se.visit_expression(left);
                se.write(" * ");
                se.visit_expression(right)
            }),
            SqlOp::Div(left, right) => self.surround_with("(", ")", |ref mut se| {
                se.visit_expression(left);
                se.write(" / ");
                se.visit_expression(right)
            }),
            SqlOp::Rem(left, right) => self.surround_with("(", ")", |ref mut se| {
                se.visit_expression(left);
                se.write(" % ");
                se.visit_expression(right)
            }),
            SqlOp::Append(left, right) => self.surround_with("(", ")", |ref mut se| {
                se.visit_expression(left);
                se.write(" || ");
                se.visit_expression(right)
            }),
            SqlOp::JsonDeleteAtPath(left, right) => self.surround_with("(", ")", |ref mut se| {
                se.visit_expression(left);
                se.write(" #- ");
                se.visit_expression(right);
            }),
        }
    }

    /// A visit to a value used in an expression
    fn visit_expression(&mut self, value: Expression<'_>) {
        match value.kind {
            ExpressionKind::Value(value) => self.visit_expression(*value),
            ExpressionKind::Raw(value) => self.write(value),
            ExpressionKind::RawString(value) => {
                self.write(C_QUOTE);
                self.write(value);
                self.write(C_QUOTE);
            }
            ExpressionKind::ConditionTree(tree) => self.visit_conditions(tree),
            ExpressionKind::Compare(compare) => self.visit_compare(compare),
            ExpressionKind::Parameterized(val) => self.substitute_value(val),
            ExpressionKind::ManyParameterized(vals) => {
                let length = vals.len();

                for (i, val) in vals.into_iter().enumerate() {
                    self.substitute_value(val);

                    if i < length - 1 {
                        self.write(", ");
                    }
                }
            }
            ExpressionKind::Column(column) => self.visit_column(*column),
            ExpressionKind::Row(row) => self.visit_row(row),
            ExpressionKind::Selection(selection) => {
                self.surround_with("(", ")", |ref mut s| s.visit_select(*selection))
            }
            ExpressionKind::Function(function) => self.visit_function(*function),
            ExpressionKind::Op(op) => self.visit_operation(*op),
            ExpressionKind::Values(values) => self.visit_values(values),
            ExpressionKind::Asterisk(table) => match table {
                Some(table) => {
                    self.visit_table(*table, false);
                    self.write(".*")
                }
                None => self.write("*"),
            },
            ExpressionKind::Default => self.write("DEFAULT"),
            ExpressionKind::Table(table) => self.visit_table(*table, false),
            ExpressionKind::Case(case) => self.visit_case(case),
        }

        if let Some(alias) = value.alias {
            self.visit_alias(alias);
        };
    }

    fn visit_multiple_tuple_comparison(&mut self, left: Row<'_>, right: Values<'_>, negate: bool) {
        self.visit_row(left);
        self.write(if negate { " NOT IN " } else { " IN " });
        self.visit_values(right)
    }

    fn visit_values(&mut self, values: Values<'_>) {
        self.surround_with("(", ")", |ref mut s| {
            let len = values.len();
            for (i, row) in values.into_iter().enumerate() {
                s.visit_row(row);

                if i < (len - 1) {
                    s.write(",");
                }
            }
        })
    }

    /// A database table identifier
    fn visit_table(&mut self, table: Table<'_>, include_alias: bool) {
        match table.typ {
            TableType::Table(table_name) => match table.database {
                Some(database) => self.delimited_identifiers([&*database, &*table_name].into_iter()),
                None => self.delimited_identifiers([&*table_name].into_iter()),
            },
            TableType::Values(values) => self.visit_values(values),
            TableType::Query(select) => self.surround_with("(", ")", |ref mut s| s.visit_select(*select)),
            TableType::JoinedTable(jt) => {
                match table.database {
                    Some(database) => self.delimited_identifiers([&*database, &*jt.0].into_iter()),
                    None => self.delimited_identifiers([&*jt.0].into_iter()),
                }
                self.visit_joins(jt.1)
            }
            TableType::Expression(expr) => {
                self.visit_expression(expr);
            }
        };

        if include_alias {
            if let Some(alias) = table.alias {
                self.visit_alias(alias);
            };
        }
    }

    /// A database column identifier
    fn visit_column(&mut self, column: Column<'_>) {
        match column.table {
            Some(table) => {
                self.visit_table(table, false);
                self.write(".");
                self.delimited_identifiers([&*column.name].into_iter());
            }
            _ => self.delimited_identifiers([&*column.name].into_iter()),
        };

        if let Some(alias) = column.alias {
            self.visit_alias(alias);
        }
    }

    /// A row of data used as an expression
    fn visit_row(&mut self, row: Row<'_>) {
        self.surround_with("(", ")", |ref mut s| {
            let len = row.values.len();
            for (i, value) in row.values.into_iter().enumerate() {
                s.visit_expression(value);

                if i < (len - 1) {
                    s.write(",");
                }
            }
        })
    }

    /// A walk through the query conditions
    fn visit_conditions(&mut self, tree: ConditionTree<'_>) {
        match tree {
            ConditionTree::And(expressions) => self.surround_with("(", ")", |ref mut s| {
                let len = expressions.len();

                for (i, expr) in expressions.into_iter().enumerate() {
                    s.visit_expression(expr);

                    if i < (len - 1) {
                        s.write(" AND ");
                    }
                }
            }),
            ConditionTree::Or(expressions) => self.surround_with("(", ")", |ref mut s| {
                let len = expressions.len();

                for (i, expr) in expressions.into_iter().enumerate() {
                    s.visit_expression(expr);

                    if i < (len - 1) {
                        s.write(" OR ");
                    }
                }
            }),
            ConditionTree::Not(expression) => self.surround_with("(", ")", |ref mut s| {
                s.write("NOT ");
                s.visit_expression(*expression)
            }),
            ConditionTree::Single(expression) => self.visit_expression(*expression),
            ConditionTree::NoCondition => self.write("1=1"),
            ConditionTree::NegativeCondition => self.write("1=0"),
            ConditionTree::Exists(table) => self.surround_with("(", ")", |ref mut s| {
                s.write("EXISTS ");

                s.surround_with("(", ")", |ref mut s| {
                    s.visit_table(*table, false);
                })
            }),
        }
    }

    fn visit_greater_than(&mut self, left: Expression<'_>, right: Expression<'_>) {
        self.visit_expression(left);
        self.write(" > ");
        self.visit_expression(right)
    }

    fn visit_greater_than_or_equals(&mut self, left: Expression<'_>, right: Expression<'_>) {
        self.visit_expression(left);
        self.write(" >= ");
        self.visit_expression(right)
    }

    fn visit_less_than(&mut self, left: Expression<'_>, right: Expression<'_>) {
        self.visit_expression(left);
        self.write(" < ");
        self.visit_expression(right)
    }

    fn visit_less_than_or_equals(&mut self, left: Expression<'_>, right: Expression<'_>) {
        self.visit_expression(left);
        self.write(" <= ");
        self.visit_expression(right)
    }

    /// A comparison expression
    fn visit_compare(&mut self, compare: Compare<'_>) {
        match compare {
            Compare::Equals(left, right) => self.visit_equals(*left, *right),
            Compare::NotEquals(left, right) => self.visit_not_equals(*left, *right),
            Compare::IsNotDistinctFrom(left, right) => self.visit_is_not_distinct_from(*left, *right),
            Compare::LessThan(left, right) => self.visit_less_than(*left, *right),
            Compare::LessThanOrEquals(left, right) => self.visit_less_than_or_equals(*left, *right),
            Compare::GreaterThan(left, right) => self.visit_greater_than(*left, *right),
            Compare::GreaterThanOrEquals(left, right) => self.visit_greater_than_or_equals(*left, *right),
            Compare::AnySelection(left, right) => {
                self.visit_expression(*left);
                self.write(" = ANY ");

                self.surround_with("(", ")", |this| {
                    this.visit_expression(*right);
                });
            }
            Compare::NotAllSelection(left, right) => {
                self.visit_expression(*left);
                self.write(" <> ALL ");

                self.surround_with("(", ")", |this| {
                    this.visit_expression(*right);
                });
            }
            Compare::In(left, right) => match (*left, *right) {
                // To prevent `x IN ()` from happening.
                (
                    _,
                    Expression {
                        kind: ExpressionKind::Row(ref row),
                        ..
                    },
                ) if row.is_empty() => self.write("1=0"),

                // To prevent `x IN ()` from happening.
                (
                    Expression {
                        kind: ExpressionKind::Row(_),
                        ..
                    },
                    Expression {
                        kind: ExpressionKind::Values(ref vals),
                        ..
                    },
                ) if vals.row_len() == 0 => self.write("1=0"),

                // Flattening out a row.
                (
                    Expression {
                        kind: ExpressionKind::Row(mut cols),
                        ..
                    },
                    Expression {
                        kind: ExpressionKind::Values(vals),
                        ..
                    },
                ) if cols.len() == 1 && vals.row_len() == 1 => {
                    let col = cols.pop().unwrap();
                    let vals = vals.flatten_row().unwrap();

                    self.visit_expression(col);
                    self.write(" IN ");
                    self.visit_row(vals)
                }

                // No need to do `IN` if right side is only one value,
                (
                    left,
                    Expression {
                        kind: ExpressionKind::Parameterized(pv),
                        ..
                    },
                ) => {
                    self.visit_expression(left);
                    self.write(" = ");
                    self.substitute_value(pv)
                }

                (
                    Expression {
                        kind: ExpressionKind::Row(row),
                        ..
                    },
                    Expression {
                        kind: ExpressionKind::Values(values),
                        ..
                    },
                ) => self.visit_multiple_tuple_comparison(row, values, false),

                // expr IN (..)
                (left, right) => {
                    self.visit_expression(left);
                    self.write(" IN ");
                    self.visit_expression(right)
                }
            },
            Compare::NotIn(left, right) => match (*left, *right) {
                // To prevent `x NOT IN ()` from happening.
                (
                    _,
                    Expression {
                        kind: ExpressionKind::Row(ref row),
                        ..
                    },
                ) if row.is_empty() => self.write("1=1"),

                // To prevent `x NOT IN ()` from happening.
                (
                    Expression {
                        kind: ExpressionKind::Row(_),
                        ..
                    },
                    Expression {
                        kind: ExpressionKind::Values(ref vals),
                        ..
                    },
                ) if vals.row_len() == 0 => self.write("1=1"),

                // Flattening out a row.
                (
                    Expression {
                        kind: ExpressionKind::Row(mut cols),
                        ..
                    },
                    Expression {
                        kind: ExpressionKind::Values(vals),
                        ..
                    },
                ) if cols.len() == 1 && vals.row_len() == 1 => {
                    let col = cols.pop().unwrap();
                    let vals = vals.flatten_row().unwrap();

                    self.visit_expression(col);
                    self.write(" NOT IN ");
                    self.visit_row(vals)
                }

                // No need to do `IN` if right side is only one value,
                (
                    left,
                    Expression {
                        kind: ExpressionKind::Parameterized(pv),
                        ..
                    },
                ) => {
                    self.visit_expression(left);
                    self.write(" <> ");
                    self.substitute_value(pv)
                }

                (
                    Expression {
                        kind: ExpressionKind::Row(row),
                        ..
                    },
                    Expression {
                        kind: ExpressionKind::Values(values),
                        ..
                    },
                ) => self.visit_multiple_tuple_comparison(row, values, true),

                // expr IN (..)
                (left, right) => {
                    self.visit_expression(left);
                    self.write(" NOT IN ");
                    self.visit_expression(right)
                }
            },
            Compare::Like(left, right) => self.visit_like(*left, *right),
            Compare::NotLike(left, right) => self.visit_not_like(*left, *right),
            Compare::Null(column) => {
                self.visit_expression(*column);
                self.write(" IS NULL")
            }
            Compare::NotNull(column) => {
                self.visit_expression(*column);
                self.write(" IS NOT NULL")
            }
            Compare::Between(val, left, right) => {
                self.visit_expression(*val);
                self.write(" BETWEEN ");
                self.visit_expression(*left);
                self.write(" AND ");
                self.visit_expression(*right)
            }
            Compare::NotBetween(val, left, right) => {
                self.visit_expression(*val);
                self.write(" NOT BETWEEN ");
                self.visit_expression(*left);
                self.write(" AND ");
                self.visit_expression(*right)
            }
            Compare::Raw(left, comp, right) => {
                self.visit_expression(*left);
                self.write(" ");
                self.write(comp);
                self.write(" ");
                self.visit_expression(*right)
            }
            Compare::Json(json_compare) => match json_compare {
                JsonCompare::ArrayContains(left, right) => self.visit_array_contains(*left, *right, false),
                JsonCompare::ArrayContained(left, right) => self.visit_array_contained(*left, *right, false),
                JsonCompare::ArrayOverlaps(left, right) => self.visit_array_overlaps(*left, *right),
                JsonCompare::ArrayNotContains(left, right) => self.visit_array_contains(*left, *right, true),
                JsonCompare::TypeEquals(left, json_type) => self.visit_json_type_equals(*left, json_type, false),
                JsonCompare::TypeNotEquals(left, json_type) => self.visit_json_type_equals(*left, json_type, true),
            },
            Compare::Any(left) => {
                self.write("ANY");
                self.surround_with("(", ")", |s| s.visit_expression(*left))
            }
            Compare::All(left) => {
                self.write("ALL");
                self.surround_with("(", ")", |s| s.visit_expression(*left))
            }
        }
    }

    fn visit_equals(&mut self, left: Expression<'_>, right: Expression<'_>) {
        self.visit_expression(left);
        self.write(" = ");
        self.visit_expression(right);
    }

    fn visit_not_equals(&mut self, left: Expression<'_>, right: Expression<'_>) {
        self.visit_expression(left);
        self.write(" <> ");
        self.visit_expression(right);
    }

    fn visit_is_not_distinct_from(&mut self, left: Expression<'_>, right: Expression<'_>) {
        self.visit_expression(left);
        self.write(" IS NOT DISTINCT FROM ");
        self.visit_expression(right);
    }

    /// A visit in the `GROUP BY` section of the query
    fn visit_grouping(&mut self, grouping: Grouping<'_>) {
        let len = grouping.0.len();

        for (i, value) in grouping.0.into_iter().enumerate() {
            self.visit_expression(value);

            if i < (len - 1) {
                self.write(", ");
            }
        }
    }

    fn visit_average(&mut self, avg: Average<'_>) {
        self.write("AVG");
        self.surround_with("(", ")", |ref mut s| s.visit_column(avg.column));
    }

    fn visit_function(&mut self, fun: Function<'_>) {
        match fun.r#type {
            FunctionType::Count(fun_count) => {
                if fun_count.exprs.is_empty() {
                    self.write("COUNT(*)");
                } else {
                    self.write("COUNT");
                    self.surround_with("(", ")", |ref mut s| s.visit_columns(fun_count.exprs));
                }
            }
            FunctionType::Cast(cast) => {
                self.write("CAST");
                self.surround_with("(", ")", |ref mut s| {
                    s.visit_expression(cast.expr);
                    s.write(" AS ");
                    s.write(cast.target_type);
                });
            }
            FunctionType::AggregateToString(agg) => {
                self.visit_aggregate_to_string(agg.value.as_ref().clone());
            }
            FunctionType::RowToJson(row_to_json) => {
                self.write("ROW_TO_JSON");
                self.surround_with("(", ")", |ref mut s| s.visit_table(row_to_json.expr, false))
            }
            FunctionType::Average(avg) => {
                self.visit_average(avg);
            }
            FunctionType::Sum(sum) => {
                self.write("SUM");
                self.surround_with("(", ")", |ref mut s| s.visit_expression(*sum.expr));
            }
            FunctionType::Lower(lower) => {
                self.write("LOWER");
                self.surround_with("(", ")", |ref mut s| s.visit_expression(*lower.expression));
            }
            FunctionType::Upper(upper) => {
                self.write("UPPER");
                self.surround_with("(", ")", |ref mut s| s.visit_expression(*upper.expression));
            }
            FunctionType::Minimum(min) => {
                self.write("MIN");
                self.surround_with("(", ")", |ref mut s| s.visit_column(min.column));
            }
            FunctionType::Maximum(max) => {
                self.write("MAX");
                self.surround_with("(", ")", |ref mut s| s.visit_column(max.column));
            }
            FunctionType::Coalesce(coalesce) => {
                self.write("COALESCE");
                self.surround_with("(", ")", |s| s.visit_columns(coalesce.exprs));
            }
            FunctionType::JsonExtract(json_extract) => {
                self.visit_json_extract(json_extract);
            }
            FunctionType::JsonExtractArrayElem(extract) => {
                self.visit_json_extract_array_item(extract);
            }
            FunctionType::JsonUnquote(unquote) => {
                self.visit_json_unquote(unquote);
            }
            FunctionType::ArrayPosition(pos) => {
                self.write("ARRAY_POSITION");
                self.surround_with("(", ")", |s| {
                    s.visit_expression(pos.array);
                    s.write(",");
                    s.visit_expression(pos.column);
                });
            }
            FunctionType::ConvertFrom(convert_from) => {
                self.write("CONVERT_FROM");
                self.surround_with("(", ")", |s| {
                    s.visit_expression(convert_from.expression);
                    s.write(",");
                    s.surround_with("'", "'", |s| {
                        s.write(convert_from.charset);
                    });
                });
            }
            FunctionType::ToJsonb(to_jsonb) => self.visit_to_jsonb(to_jsonb),
            FunctionType::JsonAgg(json_agg) => self.visit_json_agg(json_agg),
            FunctionType::Encode(encode) => self.visit_encode(encode),
            FunctionType::Decode(encode) => self.visit_decode(encode),
            FunctionType::JsonBuildObject(encode) => self.visit_json_build_object(encode),
            FunctionType::JsonBuildArray(encode) => self.visit_json_build_array(encode),
            FunctionType::Unnest(unnest) => self.visit_unnest(unnest),
            FunctionType::RowNumber(row_number) => self.visit_row_number(row_number),
            FunctionType::Replace(replace) => self.visit_replace(replace),
            FunctionType::Concat(concat) => {
                self.visit_concat(concat);
            }
        };

        if let Some(alias) = fun.alias {
            self.visit_alias(alias);
        }
    }

    fn visit_alias(&mut self, alias: Alias<'_>) {
        self.write(" AS ");

        self.surround_with("\"", "\"", |t| {
            t.write(alias.name);
        });

        if !alias.columns.is_empty() {
            self.surround_with("(", ")", |t| {
                let len = alias.columns.len();
                for (i, column) in alias.columns.into_iter().enumerate() {
                    t.visit_column(column);

                    if i < len - 1 {
                        t.write(",");
                    }
                }
            });
        }
    }

    fn visit_case(&mut self, case: ast::Case<'_>) {
        self.write("CASE");

        for when in case.when {
            self.write(" WHEN ");
            self.visit_expression(when.condition);
            self.write(" THEN ");
            self.visit_expression(when.result);
        }

        self.write(" ELSE ");
        self.visit_expression(*case.r#else);
        self.write(" END");
    }

    fn visit_unnest(&mut self, unnest: ast::Unnest<'_>) {
        self.write("unnest");

        self.surround_with("(", ")", |s| {
            s.visit_expression(*unnest.expression);
        });

        if unnest.with_ordinality {
            self.write(" WITH ORDINALITY");
        }
    }

    fn visit_row_number(&mut self, row_number: ast::RowNumber<'_>) {
        self.write("ROW_NUMBER() OVER ");

        self.surround_with("(", ")", |this| {
            let partitioning = row_number.over.partitioning;
            let ordering = row_number.over.ordering;

            if !partitioning.is_empty() {
                let len = partitioning.len();

                this.write("PARTITION BY ");

                for (i, column) in partitioning.into_iter().enumerate() {
                    this.visit_column(column);

                    if i < len - 1 {
                        this.write(", ");
                    }
                }

                this.write(" ");
            }

            this.write("ORDER BY ");
            this.visit_ordering(ordering);
        });
    }
}
