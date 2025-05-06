use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn globally_disabled_mutations() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                CONSTRAINT "b_to_a" FOREIGN KEY (id) REFERENCES "a"."users"(id)
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"
        enable_mutations = false
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
      """
      Order User results by Loser fields
      """
      loser: LoserOrderByInput
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the related Loser object
      """
      loser: LoserFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
      """
      Order Loser results by User fields
      """
      user: UserOrderByInput
    }

    """
    Lookup input type for Loser objects for subgraph joins.
    """
    input LoserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the related User object
      """
      user: UserFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserEdge {
      """
      The item at the end of the edge
      """
      node: User! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for User
    """
    type UserConnection
      @pgConnection(type: "User")
    {
      """
      A list of edges
      """
      edges: [UserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type LoserEdge {
      """
      The item at the end of the edge
      """
      node: Loser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Loser
    """
    type LoserConnection
      @pgConnection(type: "Loser")
    {
      """
      A list of edges
      """
      edges: [LoserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      loser: Loser @pgRelation(name: "b_to_a")
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      user: User! @pgRelation(name: "b_to_a", fields: ["id"], references: ["id"])
    }

    type Query {
      """
      Query a unique User
      """
      user(
        """
        Input for unique User lookup
        """
        lookup: UserLookupInput!,
      ): User @pgSelectOne
      """
      Query and paginate multiple users
      """
      users(
        """
        Filter for User
        """
        filter: UserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [UserOrderByInput!],
      ): UserConnection! @pgSelectMany
      """
      Lookup multiple users for subgraph joins
      """
      userLookup(
        """
        Filter users with an array of keys
        """
        lookup: UserManyLookupInput @inaccessible,
      ): [User]! @pgLookup @lookup @inaccessible
      """
      Query a unique Loser
      """
      loser(
        """
        Input for unique Loser lookup
        """
        lookup: LoserLookupInput!,
      ): Loser @pgSelectOne
      """
      Query and paginate multiple losers
      """
      losers(
        """
        Filter for Loser
        """
        filter: LoserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [LoserOrderByInput!],
      ): LoserConnection! @pgSelectMany
      """
      Lookup multiple losers for subgraph joins
      """
      loserLookup(
        """
        Filter losers with an array of keys
        """
        lookup: LoserManyLookupInput @inaccessible,
      ): [Loser]! @pgLookup @lookup @inaccessible
    }
    "#);
}

#[tokio::test]
async fn globally_disabled_queries() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                CONSTRAINT "b_to_a" FOREIGN KEY (id) REFERENCES "a"."users"(id)
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"
        enable_queries = false
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the related Loser object
      """
      loser: LoserFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the related User object
      """
      user: UserFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Return type containing fields of the mutated or created User object
    """
    type UserReturning
      @pgReturning(type: "User")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one User
    """
    type UserCreatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many users
    """
    type UserCreateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one User
    """
    type UserUpdatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many users
    """
    type UserUpdateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one User
    """
    type UserDeletePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many users
    """
    type UserDeleteManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type containing fields of the mutated or created Loser object
    """
    type LoserReturning
      @pgReturning(type: "Loser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Loser
    """
    type LoserCreatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many losers
    """
    type LoserCreateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Loser
    """
    type LoserUpdatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many losers
    """
    type LoserUpdateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Loser
    """
    type LoserDeletePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many losers
    """
    type LoserDeleteManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Mutation {
      """
      Create a single User
      """
      userCreate(
        """
        Input for creating a single User
        """
        input: UserCreateInput!,
      ): UserCreatePayload! @pgInsertOne
      """
      Create multiple users
      """
      userCreateMany(
        """
        Input for creating multiple User instances
        """
        input: [UserCreateInput!]!,
      ): UserCreateManyPayload! @pgInsertMany
      """
      Update a unique User
      """
      userUpdate(
        """
        Lookup input for unique User update
        """
        lookup: UserLookupInput!,
        """
        Input for updating a User
        """
        input: UserUpdateInput!,
      ): UserUpdatePayload! @pgUpdateOne
      """
      Update multiple users
      """
      userUpdateMany(
        """
        Filter for updating multiple User instances
        """
        filter: UserFilterInput,
        """
        Input for updating multiple User instances
        """
        input: UserUpdateInput!,
      ): UserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique User
      """
      userDelete(
        """
        Lookup input for unique User deletion
        """
        lookup: UserLookupInput!,
      ): UserDeletePayload! @pgDeleteOne
      """
      Delete multiple users
      """
      userDeleteMany(
        """
        Filter for User deletion
        """
        filter: UserFilterInput,
      ): UserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Loser
      """
      loserCreate(
        """
        Input for creating a single Loser
        """
        input: LoserCreateInput!,
      ): LoserCreatePayload! @pgInsertOne
      """
      Create multiple losers
      """
      loserCreateMany(
        """
        Input for creating multiple Loser instances
        """
        input: [LoserCreateInput!]!,
      ): LoserCreateManyPayload! @pgInsertMany
      """
      Update a unique Loser
      """
      loserUpdate(
        """
        Lookup input for unique Loser update
        """
        lookup: LoserLookupInput!,
        """
        Input for updating a Loser
        """
        input: LoserUpdateInput!,
      ): LoserUpdatePayload! @pgUpdateOne
      """
      Update multiple losers
      """
      loserUpdateMany(
        """
        Filter for updating multiple Loser instances
        """
        filter: LoserFilterInput,
        """
        Input for updating multiple Loser instances
        """
        input: LoserUpdateInput!,
      ): LoserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Loser
      """
      loserDelete(
        """
        Lookup input for unique Loser deletion
        """
        lookup: LoserLookupInput!,
      ): LoserDeletePayload! @pgDeleteOne
      """
      Delete multiple losers
      """
      loserDeleteMany(
        """
        Filter for Loser deletion
        """
        filter: LoserFilterInput,
      ): LoserDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn disable_mutations_per_schema() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.b]
        enable_mutations = false
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Loser objects for subgraph joins.
    """
    input LoserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created User object
    """
    type UserReturning
      @pgReturning(type: "User")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one User
    """
    type UserCreatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many users
    """
    type UserCreateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one User
    """
    type UserUpdatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many users
    """
    type UserUpdateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one User
    """
    type UserDeletePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many users
    """
    type UserDeleteManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserEdge {
      """
      The item at the end of the edge
      """
      node: User! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for User
    """
    type UserConnection
      @pgConnection(type: "User")
    {
      """
      A list of edges
      """
      edges: [UserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type LoserEdge {
      """
      The item at the end of the edge
      """
      node: Loser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Loser
    """
    type LoserConnection
      @pgConnection(type: "Loser")
    {
      """
      A list of edges
      """
      edges: [LoserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique User
      """
      user(
        """
        Input for unique User lookup
        """
        lookup: UserLookupInput!,
      ): User @pgSelectOne
      """
      Query and paginate multiple users
      """
      users(
        """
        Filter for User
        """
        filter: UserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [UserOrderByInput!],
      ): UserConnection! @pgSelectMany
      """
      Lookup multiple users for subgraph joins
      """
      userLookup(
        """
        Filter users with an array of keys
        """
        lookup: UserManyLookupInput @inaccessible,
      ): [User]! @pgLookup @lookup @inaccessible
      """
      Query a unique Loser
      """
      loser(
        """
        Input for unique Loser lookup
        """
        lookup: LoserLookupInput!,
      ): Loser @pgSelectOne
      """
      Query and paginate multiple losers
      """
      losers(
        """
        Filter for Loser
        """
        filter: LoserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [LoserOrderByInput!],
      ): LoserConnection! @pgSelectMany
      """
      Lookup multiple losers for subgraph joins
      """
      loserLookup(
        """
        Filter losers with an array of keys
        """
        lookup: LoserManyLookupInput @inaccessible,
      ): [Loser]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single User
      """
      userCreate(
        """
        Input for creating a single User
        """
        input: UserCreateInput!,
      ): UserCreatePayload! @pgInsertOne
      """
      Create multiple users
      """
      userCreateMany(
        """
        Input for creating multiple User instances
        """
        input: [UserCreateInput!]!,
      ): UserCreateManyPayload! @pgInsertMany
      """
      Update a unique User
      """
      userUpdate(
        """
        Lookup input for unique User update
        """
        lookup: UserLookupInput!,
        """
        Input for updating a User
        """
        input: UserUpdateInput!,
      ): UserUpdatePayload! @pgUpdateOne
      """
      Update multiple users
      """
      userUpdateMany(
        """
        Filter for updating multiple User instances
        """
        filter: UserFilterInput,
        """
        Input for updating multiple User instances
        """
        input: UserUpdateInput!,
      ): UserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique User
      """
      userDelete(
        """
        Lookup input for unique User deletion
        """
        lookup: UserLookupInput!,
      ): UserDeletePayload! @pgDeleteOne
      """
      Delete multiple users
      """
      userDeleteMany(
        """
        Filter for User deletion
        """
        filter: UserFilterInput,
      ): UserDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn disable_queries_per_schema() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.b]
        enable_queries = false
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created User object
    """
    type UserReturning
      @pgReturning(type: "User")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one User
    """
    type UserCreatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many users
    """
    type UserCreateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one User
    """
    type UserUpdatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many users
    """
    type UserUpdateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one User
    """
    type UserDeletePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many users
    """
    type UserDeleteManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserEdge {
      """
      The item at the end of the edge
      """
      node: User! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for User
    """
    type UserConnection
      @pgConnection(type: "User")
    {
      """
      A list of edges
      """
      edges: [UserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type containing fields of the mutated or created Loser object
    """
    type LoserReturning
      @pgReturning(type: "Loser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Loser
    """
    type LoserCreatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many losers
    """
    type LoserCreateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Loser
    """
    type LoserUpdatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many losers
    """
    type LoserUpdateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Loser
    """
    type LoserDeletePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many losers
    """
    type LoserDeleteManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique User
      """
      user(
        """
        Input for unique User lookup
        """
        lookup: UserLookupInput!,
      ): User @pgSelectOne
      """
      Query and paginate multiple users
      """
      users(
        """
        Filter for User
        """
        filter: UserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [UserOrderByInput!],
      ): UserConnection! @pgSelectMany
      """
      Lookup multiple users for subgraph joins
      """
      userLookup(
        """
        Filter users with an array of keys
        """
        lookup: UserManyLookupInput @inaccessible,
      ): [User]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single User
      """
      userCreate(
        """
        Input for creating a single User
        """
        input: UserCreateInput!,
      ): UserCreatePayload! @pgInsertOne
      """
      Create multiple users
      """
      userCreateMany(
        """
        Input for creating multiple User instances
        """
        input: [UserCreateInput!]!,
      ): UserCreateManyPayload! @pgInsertMany
      """
      Update a unique User
      """
      userUpdate(
        """
        Lookup input for unique User update
        """
        lookup: UserLookupInput!,
        """
        Input for updating a User
        """
        input: UserUpdateInput!,
      ): UserUpdatePayload! @pgUpdateOne
      """
      Update multiple users
      """
      userUpdateMany(
        """
        Filter for updating multiple User instances
        """
        filter: UserFilterInput,
        """
        Input for updating multiple User instances
        """
        input: UserUpdateInput!,
      ): UserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique User
      """
      userDelete(
        """
        Lookup input for unique User deletion
        """
        lookup: UserLookupInput!,
      ): UserDeletePayload! @pgDeleteOne
      """
      Delete multiple users
      """
      userDeleteMany(
        """
        Filter for User deletion
        """
        filter: UserFilterInput,
      ): UserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Loser
      """
      loserCreate(
        """
        Input for creating a single Loser
        """
        input: LoserCreateInput!,
      ): LoserCreatePayload! @pgInsertOne
      """
      Create multiple losers
      """
      loserCreateMany(
        """
        Input for creating multiple Loser instances
        """
        input: [LoserCreateInput!]!,
      ): LoserCreateManyPayload! @pgInsertMany
      """
      Update a unique Loser
      """
      loserUpdate(
        """
        Lookup input for unique Loser update
        """
        lookup: LoserLookupInput!,
        """
        Input for updating a Loser
        """
        input: LoserUpdateInput!,
      ): LoserUpdatePayload! @pgUpdateOne
      """
      Update multiple losers
      """
      loserUpdateMany(
        """
        Filter for updating multiple Loser instances
        """
        filter: LoserFilterInput,
        """
        Input for updating multiple Loser instances
        """
        input: LoserUpdateInput!,
      ): LoserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Loser
      """
      loserDelete(
        """
        Lookup input for unique Loser deletion
        """
        lookup: LoserLookupInput!,
      ): LoserDeletePayload! @pgDeleteOne
      """
      Delete multiple losers
      """
      loserDeleteMany(
        """
        Filter for Loser deletion
        """
        filter: LoserFilterInput,
      ): LoserDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn disable_mutations_per_table() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."woof" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.b.tables.losers]
        enable_mutations = false
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Loser objects for subgraph joins.
    """
    input LoserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Specifies the ordering for Woof results.
    """
    input WoofOrderByInput @oneOf {
      """
      Order woofs by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Woof objects for subgraph joins.
    """
    input WoofManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Woof
    """
    input WoofLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Woof collections
    """
    input WoofCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: WoofFilterInput
    }

    """
    Filter input type for Woof objects.
    """
    input WoofFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [WoofFilterInput]
      """
      None of the filters must match
      """
      NONE: [WoofFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [WoofFilterInput]
    }

    """
    Input for creating a new Woof
    """
    input WoofCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Woof
    """
    input WoofUpdateInput {
      _: Boolean
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created User object
    """
    type UserReturning
      @pgReturning(type: "User")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one User
    """
    type UserCreatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many users
    """
    type UserCreateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one User
    """
    type UserUpdatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many users
    """
    type UserUpdateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one User
    """
    type UserDeletePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many users
    """
    type UserDeleteManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserEdge {
      """
      The item at the end of the edge
      """
      node: User! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for User
    """
    type UserConnection
      @pgConnection(type: "User")
    {
      """
      A list of edges
      """
      edges: [UserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type LoserEdge {
      """
      The item at the end of the edge
      """
      node: Loser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Loser
    """
    type LoserConnection
      @pgConnection(type: "Loser")
    {
      """
      A list of edges
      """
      edges: [LoserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type containing fields of the mutated or created Woof object
    """
    type WoofReturning
      @pgReturning(type: "Woof")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Woof
    """
    type WoofCreatePayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: WoofReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many woofs
    """
    type WoofCreateManyPayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [WoofReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Woof
    """
    type WoofUpdatePayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: WoofReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many woofs
    """
    type WoofUpdateManyPayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [WoofReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Woof
    """
    type WoofDeletePayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: WoofReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many woofs
    """
    type WoofDeleteManyPayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [WoofReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type WoofEdge {
      """
      The item at the end of the edge
      """
      node: Woof! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Woof
    """
    type WoofConnection
      @pgConnection(type: "Woof")
    {
      """
      A list of edges
      """
      edges: [WoofEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Woof
      @key(fields: "id")
      @pgTable(name: "woof", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique User
      """
      user(
        """
        Input for unique User lookup
        """
        lookup: UserLookupInput!,
      ): User @pgSelectOne
      """
      Query and paginate multiple users
      """
      users(
        """
        Filter for User
        """
        filter: UserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [UserOrderByInput!],
      ): UserConnection! @pgSelectMany
      """
      Lookup multiple users for subgraph joins
      """
      userLookup(
        """
        Filter users with an array of keys
        """
        lookup: UserManyLookupInput @inaccessible,
      ): [User]! @pgLookup @lookup @inaccessible
      """
      Query a unique Loser
      """
      loser(
        """
        Input for unique Loser lookup
        """
        lookup: LoserLookupInput!,
      ): Loser @pgSelectOne
      """
      Query and paginate multiple losers
      """
      losers(
        """
        Filter for Loser
        """
        filter: LoserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [LoserOrderByInput!],
      ): LoserConnection! @pgSelectMany
      """
      Lookup multiple losers for subgraph joins
      """
      loserLookup(
        """
        Filter losers with an array of keys
        """
        lookup: LoserManyLookupInput @inaccessible,
      ): [Loser]! @pgLookup @lookup @inaccessible
      """
      Query a unique Woof
      """
      woof(
        """
        Input for unique Woof lookup
        """
        lookup: WoofLookupInput!,
      ): Woof @pgSelectOne
      """
      Query and paginate multiple woofs
      """
      woofs(
        """
        Filter for Woof
        """
        filter: WoofFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [WoofOrderByInput!],
      ): WoofConnection! @pgSelectMany
      """
      Lookup multiple woofs for subgraph joins
      """
      woofLookup(
        """
        Filter woofs with an array of keys
        """
        lookup: WoofManyLookupInput @inaccessible,
      ): [Woof]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single User
      """
      userCreate(
        """
        Input for creating a single User
        """
        input: UserCreateInput!,
      ): UserCreatePayload! @pgInsertOne
      """
      Create multiple users
      """
      userCreateMany(
        """
        Input for creating multiple User instances
        """
        input: [UserCreateInput!]!,
      ): UserCreateManyPayload! @pgInsertMany
      """
      Update a unique User
      """
      userUpdate(
        """
        Lookup input for unique User update
        """
        lookup: UserLookupInput!,
        """
        Input for updating a User
        """
        input: UserUpdateInput!,
      ): UserUpdatePayload! @pgUpdateOne
      """
      Update multiple users
      """
      userUpdateMany(
        """
        Filter for updating multiple User instances
        """
        filter: UserFilterInput,
        """
        Input for updating multiple User instances
        """
        input: UserUpdateInput!,
      ): UserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique User
      """
      userDelete(
        """
        Lookup input for unique User deletion
        """
        lookup: UserLookupInput!,
      ): UserDeletePayload! @pgDeleteOne
      """
      Delete multiple users
      """
      userDeleteMany(
        """
        Filter for User deletion
        """
        filter: UserFilterInput,
      ): UserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Woof
      """
      woofCreate(
        """
        Input for creating a single Woof
        """
        input: WoofCreateInput!,
      ): WoofCreatePayload! @pgInsertOne
      """
      Create multiple woofs
      """
      woofCreateMany(
        """
        Input for creating multiple Woof instances
        """
        input: [WoofCreateInput!]!,
      ): WoofCreateManyPayload! @pgInsertMany
      """
      Update a unique Woof
      """
      woofUpdate(
        """
        Lookup input for unique Woof update
        """
        lookup: WoofLookupInput!,
        """
        Input for updating a Woof
        """
        input: WoofUpdateInput!,
      ): WoofUpdatePayload! @pgUpdateOne
      """
      Update multiple woofs
      """
      woofUpdateMany(
        """
        Filter for updating multiple Woof instances
        """
        filter: WoofFilterInput,
        """
        Input for updating multiple Woof instances
        """
        input: WoofUpdateInput!,
      ): WoofUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Woof
      """
      woofDelete(
        """
        Lookup input for unique Woof deletion
        """
        lookup: WoofLookupInput!,
      ): WoofDeletePayload! @pgDeleteOne
      """
      Delete multiple woofs
      """
      woofDeleteMany(
        """
        Filter for Woof deletion
        """
        filter: WoofFilterInput,
      ): WoofDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn disable_queries_per_table() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."woof" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.b.tables.losers]
        enable_queries = false
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Specifies the ordering for Woof results.
    """
    input WoofOrderByInput @oneOf {
      """
      Order woofs by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Woof objects for subgraph joins.
    """
    input WoofManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Woof
    """
    input WoofLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Woof collections
    """
    input WoofCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: WoofFilterInput
    }

    """
    Filter input type for Woof objects.
    """
    input WoofFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [WoofFilterInput]
      """
      None of the filters must match
      """
      NONE: [WoofFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [WoofFilterInput]
    }

    """
    Input for creating a new Woof
    """
    input WoofCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Woof
    """
    input WoofUpdateInput {
      _: Boolean
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created User object
    """
    type UserReturning
      @pgReturning(type: "User")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one User
    """
    type UserCreatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many users
    """
    type UserCreateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one User
    """
    type UserUpdatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many users
    """
    type UserUpdateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one User
    """
    type UserDeletePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many users
    """
    type UserDeleteManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserEdge {
      """
      The item at the end of the edge
      """
      node: User! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for User
    """
    type UserConnection
      @pgConnection(type: "User")
    {
      """
      A list of edges
      """
      edges: [UserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type containing fields of the mutated or created Loser object
    """
    type LoserReturning
      @pgReturning(type: "Loser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Loser
    """
    type LoserCreatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many losers
    """
    type LoserCreateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Loser
    """
    type LoserUpdatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many losers
    """
    type LoserUpdateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Loser
    """
    type LoserDeletePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many losers
    """
    type LoserDeleteManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type containing fields of the mutated or created Woof object
    """
    type WoofReturning
      @pgReturning(type: "Woof")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Woof
    """
    type WoofCreatePayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: WoofReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many woofs
    """
    type WoofCreateManyPayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [WoofReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Woof
    """
    type WoofUpdatePayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: WoofReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many woofs
    """
    type WoofUpdateManyPayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [WoofReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Woof
    """
    type WoofDeletePayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: WoofReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many woofs
    """
    type WoofDeleteManyPayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [WoofReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type WoofEdge {
      """
      The item at the end of the edge
      """
      node: Woof! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Woof
    """
    type WoofConnection
      @pgConnection(type: "Woof")
    {
      """
      A list of edges
      """
      edges: [WoofEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Woof
      @key(fields: "id")
      @pgTable(name: "woof", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique User
      """
      user(
        """
        Input for unique User lookup
        """
        lookup: UserLookupInput!,
      ): User @pgSelectOne
      """
      Query and paginate multiple users
      """
      users(
        """
        Filter for User
        """
        filter: UserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [UserOrderByInput!],
      ): UserConnection! @pgSelectMany
      """
      Lookup multiple users for subgraph joins
      """
      userLookup(
        """
        Filter users with an array of keys
        """
        lookup: UserManyLookupInput @inaccessible,
      ): [User]! @pgLookup @lookup @inaccessible
      """
      Query a unique Woof
      """
      woof(
        """
        Input for unique Woof lookup
        """
        lookup: WoofLookupInput!,
      ): Woof @pgSelectOne
      """
      Query and paginate multiple woofs
      """
      woofs(
        """
        Filter for Woof
        """
        filter: WoofFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [WoofOrderByInput!],
      ): WoofConnection! @pgSelectMany
      """
      Lookup multiple woofs for subgraph joins
      """
      woofLookup(
        """
        Filter woofs with an array of keys
        """
        lookup: WoofManyLookupInput @inaccessible,
      ): [Woof]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single User
      """
      userCreate(
        """
        Input for creating a single User
        """
        input: UserCreateInput!,
      ): UserCreatePayload! @pgInsertOne
      """
      Create multiple users
      """
      userCreateMany(
        """
        Input for creating multiple User instances
        """
        input: [UserCreateInput!]!,
      ): UserCreateManyPayload! @pgInsertMany
      """
      Update a unique User
      """
      userUpdate(
        """
        Lookup input for unique User update
        """
        lookup: UserLookupInput!,
        """
        Input for updating a User
        """
        input: UserUpdateInput!,
      ): UserUpdatePayload! @pgUpdateOne
      """
      Update multiple users
      """
      userUpdateMany(
        """
        Filter for updating multiple User instances
        """
        filter: UserFilterInput,
        """
        Input for updating multiple User instances
        """
        input: UserUpdateInput!,
      ): UserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique User
      """
      userDelete(
        """
        Lookup input for unique User deletion
        """
        lookup: UserLookupInput!,
      ): UserDeletePayload! @pgDeleteOne
      """
      Delete multiple users
      """
      userDeleteMany(
        """
        Filter for User deletion
        """
        filter: UserFilterInput,
      ): UserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Loser
      """
      loserCreate(
        """
        Input for creating a single Loser
        """
        input: LoserCreateInput!,
      ): LoserCreatePayload! @pgInsertOne
      """
      Create multiple losers
      """
      loserCreateMany(
        """
        Input for creating multiple Loser instances
        """
        input: [LoserCreateInput!]!,
      ): LoserCreateManyPayload! @pgInsertMany
      """
      Update a unique Loser
      """
      loserUpdate(
        """
        Lookup input for unique Loser update
        """
        lookup: LoserLookupInput!,
        """
        Input for updating a Loser
        """
        input: LoserUpdateInput!,
      ): LoserUpdatePayload! @pgUpdateOne
      """
      Update multiple losers
      """
      loserUpdateMany(
        """
        Filter for updating multiple Loser instances
        """
        filter: LoserFilterInput,
        """
        Input for updating multiple Loser instances
        """
        input: LoserUpdateInput!,
      ): LoserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Loser
      """
      loserDelete(
        """
        Lookup input for unique Loser deletion
        """
        lookup: LoserLookupInput!,
      ): LoserDeletePayload! @pgDeleteOne
      """
      Delete multiple losers
      """
      loserDeleteMany(
        """
        Filter for Loser deletion
        """
        filter: LoserFilterInput,
      ): LoserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Woof
      """
      woofCreate(
        """
        Input for creating a single Woof
        """
        input: WoofCreateInput!,
      ): WoofCreatePayload! @pgInsertOne
      """
      Create multiple woofs
      """
      woofCreateMany(
        """
        Input for creating multiple Woof instances
        """
        input: [WoofCreateInput!]!,
      ): WoofCreateManyPayload! @pgInsertMany
      """
      Update a unique Woof
      """
      woofUpdate(
        """
        Lookup input for unique Woof update
        """
        lookup: WoofLookupInput!,
        """
        Input for updating a Woof
        """
        input: WoofUpdateInput!,
      ): WoofUpdatePayload! @pgUpdateOne
      """
      Update multiple woofs
      """
      woofUpdateMany(
        """
        Filter for updating multiple Woof instances
        """
        filter: WoofFilterInput,
        """
        Input for updating multiple Woof instances
        """
        input: WoofUpdateInput!,
      ): WoofUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Woof
      """
      woofDelete(
        """
        Lookup input for unique Woof deletion
        """
        lookup: WoofLookupInput!,
      ): WoofDeletePayload! @pgDeleteOne
      """
      Delete multiple woofs
      """
      woofDeleteMany(
        """
        Filter for Woof deletion
        """
        filter: WoofFilterInput,
      ): WoofDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn schema_mutations_setting_takes_precedence_over_global_setting() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        enable_mutations = false

        [schemas.b]
        enable_mutations = true
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Loser objects for subgraph joins.
    """
    input LoserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserEdge {
      """
      The item at the end of the edge
      """
      node: User! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for User
    """
    type UserConnection
      @pgConnection(type: "User")
    {
      """
      A list of edges
      """
      edges: [UserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type containing fields of the mutated or created Loser object
    """
    type LoserReturning
      @pgReturning(type: "Loser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Loser
    """
    type LoserCreatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many losers
    """
    type LoserCreateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Loser
    """
    type LoserUpdatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many losers
    """
    type LoserUpdateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Loser
    """
    type LoserDeletePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many losers
    """
    type LoserDeleteManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type LoserEdge {
      """
      The item at the end of the edge
      """
      node: Loser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Loser
    """
    type LoserConnection
      @pgConnection(type: "Loser")
    {
      """
      A list of edges
      """
      edges: [LoserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique User
      """
      user(
        """
        Input for unique User lookup
        """
        lookup: UserLookupInput!,
      ): User @pgSelectOne
      """
      Query and paginate multiple users
      """
      users(
        """
        Filter for User
        """
        filter: UserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [UserOrderByInput!],
      ): UserConnection! @pgSelectMany
      """
      Lookup multiple users for subgraph joins
      """
      userLookup(
        """
        Filter users with an array of keys
        """
        lookup: UserManyLookupInput @inaccessible,
      ): [User]! @pgLookup @lookup @inaccessible
      """
      Query a unique Loser
      """
      loser(
        """
        Input for unique Loser lookup
        """
        lookup: LoserLookupInput!,
      ): Loser @pgSelectOne
      """
      Query and paginate multiple losers
      """
      losers(
        """
        Filter for Loser
        """
        filter: LoserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [LoserOrderByInput!],
      ): LoserConnection! @pgSelectMany
      """
      Lookup multiple losers for subgraph joins
      """
      loserLookup(
        """
        Filter losers with an array of keys
        """
        lookup: LoserManyLookupInput @inaccessible,
      ): [Loser]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single Loser
      """
      loserCreate(
        """
        Input for creating a single Loser
        """
        input: LoserCreateInput!,
      ): LoserCreatePayload! @pgInsertOne
      """
      Create multiple losers
      """
      loserCreateMany(
        """
        Input for creating multiple Loser instances
        """
        input: [LoserCreateInput!]!,
      ): LoserCreateManyPayload! @pgInsertMany
      """
      Update a unique Loser
      """
      loserUpdate(
        """
        Lookup input for unique Loser update
        """
        lookup: LoserLookupInput!,
        """
        Input for updating a Loser
        """
        input: LoserUpdateInput!,
      ): LoserUpdatePayload! @pgUpdateOne
      """
      Update multiple losers
      """
      loserUpdateMany(
        """
        Filter for updating multiple Loser instances
        """
        filter: LoserFilterInput,
        """
        Input for updating multiple Loser instances
        """
        input: LoserUpdateInput!,
      ): LoserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Loser
      """
      loserDelete(
        """
        Lookup input for unique Loser deletion
        """
        lookup: LoserLookupInput!,
      ): LoserDeletePayload! @pgDeleteOne
      """
      Delete multiple losers
      """
      loserDeleteMany(
        """
        Filter for Loser deletion
        """
        filter: LoserFilterInput,
      ): LoserDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn schema_queries_setting_takes_precedence_over_global_setting() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        enable_queries = false

        [schemas.b]
        enable_queries = true
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Loser objects for subgraph joins.
    """
    input LoserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created User object
    """
    type UserReturning
      @pgReturning(type: "User")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one User
    """
    type UserCreatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many users
    """
    type UserCreateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one User
    """
    type UserUpdatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many users
    """
    type UserUpdateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one User
    """
    type UserDeletePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many users
    """
    type UserDeleteManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type containing fields of the mutated or created Loser object
    """
    type LoserReturning
      @pgReturning(type: "Loser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Loser
    """
    type LoserCreatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many losers
    """
    type LoserCreateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Loser
    """
    type LoserUpdatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many losers
    """
    type LoserUpdateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Loser
    """
    type LoserDeletePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many losers
    """
    type LoserDeleteManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type LoserEdge {
      """
      The item at the end of the edge
      """
      node: Loser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Loser
    """
    type LoserConnection
      @pgConnection(type: "Loser")
    {
      """
      A list of edges
      """
      edges: [LoserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique Loser
      """
      loser(
        """
        Input for unique Loser lookup
        """
        lookup: LoserLookupInput!,
      ): Loser @pgSelectOne
      """
      Query and paginate multiple losers
      """
      losers(
        """
        Filter for Loser
        """
        filter: LoserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [LoserOrderByInput!],
      ): LoserConnection! @pgSelectMany
      """
      Lookup multiple losers for subgraph joins
      """
      loserLookup(
        """
        Filter losers with an array of keys
        """
        lookup: LoserManyLookupInput @inaccessible,
      ): [Loser]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single User
      """
      userCreate(
        """
        Input for creating a single User
        """
        input: UserCreateInput!,
      ): UserCreatePayload! @pgInsertOne
      """
      Create multiple users
      """
      userCreateMany(
        """
        Input for creating multiple User instances
        """
        input: [UserCreateInput!]!,
      ): UserCreateManyPayload! @pgInsertMany
      """
      Update a unique User
      """
      userUpdate(
        """
        Lookup input for unique User update
        """
        lookup: UserLookupInput!,
        """
        Input for updating a User
        """
        input: UserUpdateInput!,
      ): UserUpdatePayload! @pgUpdateOne
      """
      Update multiple users
      """
      userUpdateMany(
        """
        Filter for updating multiple User instances
        """
        filter: UserFilterInput,
        """
        Input for updating multiple User instances
        """
        input: UserUpdateInput!,
      ): UserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique User
      """
      userDelete(
        """
        Lookup input for unique User deletion
        """
        lookup: UserLookupInput!,
      ): UserDeletePayload! @pgDeleteOne
      """
      Delete multiple users
      """
      userDeleteMany(
        """
        Filter for User deletion
        """
        filter: UserFilterInput,
      ): UserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Loser
      """
      loserCreate(
        """
        Input for creating a single Loser
        """
        input: LoserCreateInput!,
      ): LoserCreatePayload! @pgInsertOne
      """
      Create multiple losers
      """
      loserCreateMany(
        """
        Input for creating multiple Loser instances
        """
        input: [LoserCreateInput!]!,
      ): LoserCreateManyPayload! @pgInsertMany
      """
      Update a unique Loser
      """
      loserUpdate(
        """
        Lookup input for unique Loser update
        """
        lookup: LoserLookupInput!,
        """
        Input for updating a Loser
        """
        input: LoserUpdateInput!,
      ): LoserUpdatePayload! @pgUpdateOne
      """
      Update multiple losers
      """
      loserUpdateMany(
        """
        Filter for updating multiple Loser instances
        """
        filter: LoserFilterInput,
        """
        Input for updating multiple Loser instances
        """
        input: LoserUpdateInput!,
      ): LoserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Loser
      """
      loserDelete(
        """
        Lookup input for unique Loser deletion
        """
        lookup: LoserLookupInput!,
      ): LoserDeletePayload! @pgDeleteOne
      """
      Delete multiple losers
      """
      loserDeleteMany(
        """
        Filter for Loser deletion
        """
        filter: LoserFilterInput,
      ): LoserDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn table_mutations_setting_takes_precedence_over_global_setting() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        enable_mutations = false

        [schemas.b.tables.losers]
        enable_mutations = true
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Loser objects for subgraph joins.
    """
    input LoserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserEdge {
      """
      The item at the end of the edge
      """
      node: User! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for User
    """
    type UserConnection
      @pgConnection(type: "User")
    {
      """
      A list of edges
      """
      edges: [UserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type containing fields of the mutated or created Loser object
    """
    type LoserReturning
      @pgReturning(type: "Loser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Loser
    """
    type LoserCreatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many losers
    """
    type LoserCreateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Loser
    """
    type LoserUpdatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many losers
    """
    type LoserUpdateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Loser
    """
    type LoserDeletePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many losers
    """
    type LoserDeleteManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type LoserEdge {
      """
      The item at the end of the edge
      """
      node: Loser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Loser
    """
    type LoserConnection
      @pgConnection(type: "Loser")
    {
      """
      A list of edges
      """
      edges: [LoserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique User
      """
      user(
        """
        Input for unique User lookup
        """
        lookup: UserLookupInput!,
      ): User @pgSelectOne
      """
      Query and paginate multiple users
      """
      users(
        """
        Filter for User
        """
        filter: UserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [UserOrderByInput!],
      ): UserConnection! @pgSelectMany
      """
      Lookup multiple users for subgraph joins
      """
      userLookup(
        """
        Filter users with an array of keys
        """
        lookup: UserManyLookupInput @inaccessible,
      ): [User]! @pgLookup @lookup @inaccessible
      """
      Query a unique Loser
      """
      loser(
        """
        Input for unique Loser lookup
        """
        lookup: LoserLookupInput!,
      ): Loser @pgSelectOne
      """
      Query and paginate multiple losers
      """
      losers(
        """
        Filter for Loser
        """
        filter: LoserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [LoserOrderByInput!],
      ): LoserConnection! @pgSelectMany
      """
      Lookup multiple losers for subgraph joins
      """
      loserLookup(
        """
        Filter losers with an array of keys
        """
        lookup: LoserManyLookupInput @inaccessible,
      ): [Loser]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single Loser
      """
      loserCreate(
        """
        Input for creating a single Loser
        """
        input: LoserCreateInput!,
      ): LoserCreatePayload! @pgInsertOne
      """
      Create multiple losers
      """
      loserCreateMany(
        """
        Input for creating multiple Loser instances
        """
        input: [LoserCreateInput!]!,
      ): LoserCreateManyPayload! @pgInsertMany
      """
      Update a unique Loser
      """
      loserUpdate(
        """
        Lookup input for unique Loser update
        """
        lookup: LoserLookupInput!,
        """
        Input for updating a Loser
        """
        input: LoserUpdateInput!,
      ): LoserUpdatePayload! @pgUpdateOne
      """
      Update multiple losers
      """
      loserUpdateMany(
        """
        Filter for updating multiple Loser instances
        """
        filter: LoserFilterInput,
        """
        Input for updating multiple Loser instances
        """
        input: LoserUpdateInput!,
      ): LoserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Loser
      """
      loserDelete(
        """
        Lookup input for unique Loser deletion
        """
        lookup: LoserLookupInput!,
      ): LoserDeletePayload! @pgDeleteOne
      """
      Delete multiple losers
      """
      loserDeleteMany(
        """
        Filter for Loser deletion
        """
        filter: LoserFilterInput,
      ): LoserDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn table_queries_setting_takes_precedence_over_global_setting() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        enable_queries = false

        [schemas.b]
        enable_queries = true
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Loser objects for subgraph joins.
    """
    input LoserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created User object
    """
    type UserReturning
      @pgReturning(type: "User")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one User
    """
    type UserCreatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many users
    """
    type UserCreateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one User
    """
    type UserUpdatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many users
    """
    type UserUpdateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one User
    """
    type UserDeletePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many users
    """
    type UserDeleteManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type containing fields of the mutated or created Loser object
    """
    type LoserReturning
      @pgReturning(type: "Loser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Loser
    """
    type LoserCreatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many losers
    """
    type LoserCreateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Loser
    """
    type LoserUpdatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many losers
    """
    type LoserUpdateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Loser
    """
    type LoserDeletePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many losers
    """
    type LoserDeleteManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type LoserEdge {
      """
      The item at the end of the edge
      """
      node: Loser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Loser
    """
    type LoserConnection
      @pgConnection(type: "Loser")
    {
      """
      A list of edges
      """
      edges: [LoserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique Loser
      """
      loser(
        """
        Input for unique Loser lookup
        """
        lookup: LoserLookupInput!,
      ): Loser @pgSelectOne
      """
      Query and paginate multiple losers
      """
      losers(
        """
        Filter for Loser
        """
        filter: LoserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [LoserOrderByInput!],
      ): LoserConnection! @pgSelectMany
      """
      Lookup multiple losers for subgraph joins
      """
      loserLookup(
        """
        Filter losers with an array of keys
        """
        lookup: LoserManyLookupInput @inaccessible,
      ): [Loser]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single User
      """
      userCreate(
        """
        Input for creating a single User
        """
        input: UserCreateInput!,
      ): UserCreatePayload! @pgInsertOne
      """
      Create multiple users
      """
      userCreateMany(
        """
        Input for creating multiple User instances
        """
        input: [UserCreateInput!]!,
      ): UserCreateManyPayload! @pgInsertMany
      """
      Update a unique User
      """
      userUpdate(
        """
        Lookup input for unique User update
        """
        lookup: UserLookupInput!,
        """
        Input for updating a User
        """
        input: UserUpdateInput!,
      ): UserUpdatePayload! @pgUpdateOne
      """
      Update multiple users
      """
      userUpdateMany(
        """
        Filter for updating multiple User instances
        """
        filter: UserFilterInput,
        """
        Input for updating multiple User instances
        """
        input: UserUpdateInput!,
      ): UserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique User
      """
      userDelete(
        """
        Lookup input for unique User deletion
        """
        lookup: UserLookupInput!,
      ): UserDeletePayload! @pgDeleteOne
      """
      Delete multiple users
      """
      userDeleteMany(
        """
        Filter for User deletion
        """
        filter: UserFilterInput,
      ): UserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Loser
      """
      loserCreate(
        """
        Input for creating a single Loser
        """
        input: LoserCreateInput!,
      ): LoserCreatePayload! @pgInsertOne
      """
      Create multiple losers
      """
      loserCreateMany(
        """
        Input for creating multiple Loser instances
        """
        input: [LoserCreateInput!]!,
      ): LoserCreateManyPayload! @pgInsertMany
      """
      Update a unique Loser
      """
      loserUpdate(
        """
        Lookup input for unique Loser update
        """
        lookup: LoserLookupInput!,
        """
        Input for updating a Loser
        """
        input: LoserUpdateInput!,
      ): LoserUpdatePayload! @pgUpdateOne
      """
      Update multiple losers
      """
      loserUpdateMany(
        """
        Filter for updating multiple Loser instances
        """
        filter: LoserFilterInput,
        """
        Input for updating multiple Loser instances
        """
        input: LoserUpdateInput!,
      ): LoserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Loser
      """
      loserDelete(
        """
        Lookup input for unique Loser deletion
        """
        lookup: LoserLookupInput!,
      ): LoserDeletePayload! @pgDeleteOne
      """
      Delete multiple losers
      """
      loserDeleteMany(
        """
        Filter for Loser deletion
        """
        filter: LoserFilterInput,
      ): LoserDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn table_mutations_setting_takes_precedence_over_schema_setting() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."woof" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        enable_mutations = true

        [schemas.b]
        enable_mutations = false

        [schemas.b.tables.losers]
        enable_mutations = true
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Loser objects for subgraph joins.
    """
    input LoserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Specifies the ordering for Woof results.
    """
    input WoofOrderByInput @oneOf {
      """
      Order woofs by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Woof objects for subgraph joins.
    """
    input WoofManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Woof
    """
    input WoofLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Woof collections
    """
    input WoofCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: WoofFilterInput
    }

    """
    Filter input type for Woof objects.
    """
    input WoofFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [WoofFilterInput]
      """
      None of the filters must match
      """
      NONE: [WoofFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [WoofFilterInput]
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created User object
    """
    type UserReturning
      @pgReturning(type: "User")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one User
    """
    type UserCreatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many users
    """
    type UserCreateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one User
    """
    type UserUpdatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many users
    """
    type UserUpdateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one User
    """
    type UserDeletePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many users
    """
    type UserDeleteManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserEdge {
      """
      The item at the end of the edge
      """
      node: User! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for User
    """
    type UserConnection
      @pgConnection(type: "User")
    {
      """
      A list of edges
      """
      edges: [UserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type containing fields of the mutated or created Loser object
    """
    type LoserReturning
      @pgReturning(type: "Loser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Loser
    """
    type LoserCreatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many losers
    """
    type LoserCreateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Loser
    """
    type LoserUpdatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many losers
    """
    type LoserUpdateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Loser
    """
    type LoserDeletePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many losers
    """
    type LoserDeleteManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type LoserEdge {
      """
      The item at the end of the edge
      """
      node: Loser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Loser
    """
    type LoserConnection
      @pgConnection(type: "Loser")
    {
      """
      A list of edges
      """
      edges: [LoserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type WoofEdge {
      """
      The item at the end of the edge
      """
      node: Woof! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Woof
    """
    type WoofConnection
      @pgConnection(type: "Woof")
    {
      """
      A list of edges
      """
      edges: [WoofEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Woof
      @key(fields: "id")
      @pgTable(name: "woof", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique User
      """
      user(
        """
        Input for unique User lookup
        """
        lookup: UserLookupInput!,
      ): User @pgSelectOne
      """
      Query and paginate multiple users
      """
      users(
        """
        Filter for User
        """
        filter: UserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [UserOrderByInput!],
      ): UserConnection! @pgSelectMany
      """
      Lookup multiple users for subgraph joins
      """
      userLookup(
        """
        Filter users with an array of keys
        """
        lookup: UserManyLookupInput @inaccessible,
      ): [User]! @pgLookup @lookup @inaccessible
      """
      Query a unique Loser
      """
      loser(
        """
        Input for unique Loser lookup
        """
        lookup: LoserLookupInput!,
      ): Loser @pgSelectOne
      """
      Query and paginate multiple losers
      """
      losers(
        """
        Filter for Loser
        """
        filter: LoserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [LoserOrderByInput!],
      ): LoserConnection! @pgSelectMany
      """
      Lookup multiple losers for subgraph joins
      """
      loserLookup(
        """
        Filter losers with an array of keys
        """
        lookup: LoserManyLookupInput @inaccessible,
      ): [Loser]! @pgLookup @lookup @inaccessible
      """
      Query a unique Woof
      """
      woof(
        """
        Input for unique Woof lookup
        """
        lookup: WoofLookupInput!,
      ): Woof @pgSelectOne
      """
      Query and paginate multiple woofs
      """
      woofs(
        """
        Filter for Woof
        """
        filter: WoofFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [WoofOrderByInput!],
      ): WoofConnection! @pgSelectMany
      """
      Lookup multiple woofs for subgraph joins
      """
      woofLookup(
        """
        Filter woofs with an array of keys
        """
        lookup: WoofManyLookupInput @inaccessible,
      ): [Woof]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single User
      """
      userCreate(
        """
        Input for creating a single User
        """
        input: UserCreateInput!,
      ): UserCreatePayload! @pgInsertOne
      """
      Create multiple users
      """
      userCreateMany(
        """
        Input for creating multiple User instances
        """
        input: [UserCreateInput!]!,
      ): UserCreateManyPayload! @pgInsertMany
      """
      Update a unique User
      """
      userUpdate(
        """
        Lookup input for unique User update
        """
        lookup: UserLookupInput!,
        """
        Input for updating a User
        """
        input: UserUpdateInput!,
      ): UserUpdatePayload! @pgUpdateOne
      """
      Update multiple users
      """
      userUpdateMany(
        """
        Filter for updating multiple User instances
        """
        filter: UserFilterInput,
        """
        Input for updating multiple User instances
        """
        input: UserUpdateInput!,
      ): UserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique User
      """
      userDelete(
        """
        Lookup input for unique User deletion
        """
        lookup: UserLookupInput!,
      ): UserDeletePayload! @pgDeleteOne
      """
      Delete multiple users
      """
      userDeleteMany(
        """
        Filter for User deletion
        """
        filter: UserFilterInput,
      ): UserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Loser
      """
      loserCreate(
        """
        Input for creating a single Loser
        """
        input: LoserCreateInput!,
      ): LoserCreatePayload! @pgInsertOne
      """
      Create multiple losers
      """
      loserCreateMany(
        """
        Input for creating multiple Loser instances
        """
        input: [LoserCreateInput!]!,
      ): LoserCreateManyPayload! @pgInsertMany
      """
      Update a unique Loser
      """
      loserUpdate(
        """
        Lookup input for unique Loser update
        """
        lookup: LoserLookupInput!,
        """
        Input for updating a Loser
        """
        input: LoserUpdateInput!,
      ): LoserUpdatePayload! @pgUpdateOne
      """
      Update multiple losers
      """
      loserUpdateMany(
        """
        Filter for updating multiple Loser instances
        """
        filter: LoserFilterInput,
        """
        Input for updating multiple Loser instances
        """
        input: LoserUpdateInput!,
      ): LoserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Loser
      """
      loserDelete(
        """
        Lookup input for unique Loser deletion
        """
        lookup: LoserLookupInput!,
      ): LoserDeletePayload! @pgDeleteOne
      """
      Delete multiple losers
      """
      loserDeleteMany(
        """
        Filter for Loser deletion
        """
        filter: LoserFilterInput,
      ): LoserDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn table_queries_setting_takes_precedence_over_schema_setting() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."woof" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        enable_queries = true

        [schemas.b]
        enable_queries = false

        [schemas.b.tables.losers]
        enable_queries = true
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Loser objects for subgraph joins.
    """
    input LoserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input type to select a unique Woof
    """
    input WoofLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Woof collections
    """
    input WoofCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: WoofFilterInput
    }

    """
    Filter input type for Woof objects.
    """
    input WoofFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [WoofFilterInput]
      """
      None of the filters must match
      """
      NONE: [WoofFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [WoofFilterInput]
    }

    """
    Input for creating a new Woof
    """
    input WoofCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Woof
    """
    input WoofUpdateInput {
      _: Boolean
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created User object
    """
    type UserReturning
      @pgReturning(type: "User")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one User
    """
    type UserCreatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many users
    """
    type UserCreateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one User
    """
    type UserUpdatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many users
    """
    type UserUpdateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one User
    """
    type UserDeletePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many users
    """
    type UserDeleteManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserEdge {
      """
      The item at the end of the edge
      """
      node: User! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for User
    """
    type UserConnection
      @pgConnection(type: "User")
    {
      """
      A list of edges
      """
      edges: [UserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type containing fields of the mutated or created Loser object
    """
    type LoserReturning
      @pgReturning(type: "Loser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Loser
    """
    type LoserCreatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many losers
    """
    type LoserCreateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Loser
    """
    type LoserUpdatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many losers
    """
    type LoserUpdateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Loser
    """
    type LoserDeletePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many losers
    """
    type LoserDeleteManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type LoserEdge {
      """
      The item at the end of the edge
      """
      node: Loser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Loser
    """
    type LoserConnection
      @pgConnection(type: "Loser")
    {
      """
      A list of edges
      """
      edges: [LoserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type containing fields of the mutated or created Woof object
    """
    type WoofReturning
      @pgReturning(type: "Woof")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Woof
    """
    type WoofCreatePayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: WoofReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many woofs
    """
    type WoofCreateManyPayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [WoofReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Woof
    """
    type WoofUpdatePayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: WoofReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many woofs
    """
    type WoofUpdateManyPayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [WoofReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Woof
    """
    type WoofDeletePayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: WoofReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many woofs
    """
    type WoofDeleteManyPayload
      @pgMutation(type: "Woof")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [WoofReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Woof
      @key(fields: "id")
      @pgTable(name: "woof", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique User
      """
      user(
        """
        Input for unique User lookup
        """
        lookup: UserLookupInput!,
      ): User @pgSelectOne
      """
      Query and paginate multiple users
      """
      users(
        """
        Filter for User
        """
        filter: UserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [UserOrderByInput!],
      ): UserConnection! @pgSelectMany
      """
      Lookup multiple users for subgraph joins
      """
      userLookup(
        """
        Filter users with an array of keys
        """
        lookup: UserManyLookupInput @inaccessible,
      ): [User]! @pgLookup @lookup @inaccessible
      """
      Query a unique Loser
      """
      loser(
        """
        Input for unique Loser lookup
        """
        lookup: LoserLookupInput!,
      ): Loser @pgSelectOne
      """
      Query and paginate multiple losers
      """
      losers(
        """
        Filter for Loser
        """
        filter: LoserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [LoserOrderByInput!],
      ): LoserConnection! @pgSelectMany
      """
      Lookup multiple losers for subgraph joins
      """
      loserLookup(
        """
        Filter losers with an array of keys
        """
        lookup: LoserManyLookupInput @inaccessible,
      ): [Loser]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single User
      """
      userCreate(
        """
        Input for creating a single User
        """
        input: UserCreateInput!,
      ): UserCreatePayload! @pgInsertOne
      """
      Create multiple users
      """
      userCreateMany(
        """
        Input for creating multiple User instances
        """
        input: [UserCreateInput!]!,
      ): UserCreateManyPayload! @pgInsertMany
      """
      Update a unique User
      """
      userUpdate(
        """
        Lookup input for unique User update
        """
        lookup: UserLookupInput!,
        """
        Input for updating a User
        """
        input: UserUpdateInput!,
      ): UserUpdatePayload! @pgUpdateOne
      """
      Update multiple users
      """
      userUpdateMany(
        """
        Filter for updating multiple User instances
        """
        filter: UserFilterInput,
        """
        Input for updating multiple User instances
        """
        input: UserUpdateInput!,
      ): UserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique User
      """
      userDelete(
        """
        Lookup input for unique User deletion
        """
        lookup: UserLookupInput!,
      ): UserDeletePayload! @pgDeleteOne
      """
      Delete multiple users
      """
      userDeleteMany(
        """
        Filter for User deletion
        """
        filter: UserFilterInput,
      ): UserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Loser
      """
      loserCreate(
        """
        Input for creating a single Loser
        """
        input: LoserCreateInput!,
      ): LoserCreatePayload! @pgInsertOne
      """
      Create multiple losers
      """
      loserCreateMany(
        """
        Input for creating multiple Loser instances
        """
        input: [LoserCreateInput!]!,
      ): LoserCreateManyPayload! @pgInsertMany
      """
      Update a unique Loser
      """
      loserUpdate(
        """
        Lookup input for unique Loser update
        """
        lookup: LoserLookupInput!,
        """
        Input for updating a Loser
        """
        input: LoserUpdateInput!,
      ): LoserUpdatePayload! @pgUpdateOne
      """
      Update multiple losers
      """
      loserUpdateMany(
        """
        Filter for updating multiple Loser instances
        """
        filter: LoserFilterInput,
        """
        Input for updating multiple Loser instances
        """
        input: LoserUpdateInput!,
      ): LoserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Loser
      """
      loserDelete(
        """
        Lookup input for unique Loser deletion
        """
        lookup: LoserLookupInput!,
      ): LoserDeletePayload! @pgDeleteOne
      """
      Delete multiple losers
      """
      loserDeleteMany(
        """
        Filter for Loser deletion
        """
        filter: LoserFilterInput,
      ): LoserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Woof
      """
      woofCreate(
        """
        Input for creating a single Woof
        """
        input: WoofCreateInput!,
      ): WoofCreatePayload! @pgInsertOne
      """
      Create multiple woofs
      """
      woofCreateMany(
        """
        Input for creating multiple Woof instances
        """
        input: [WoofCreateInput!]!,
      ): WoofCreateManyPayload! @pgInsertMany
      """
      Update a unique Woof
      """
      woofUpdate(
        """
        Lookup input for unique Woof update
        """
        lookup: WoofLookupInput!,
        """
        Input for updating a Woof
        """
        input: WoofUpdateInput!,
      ): WoofUpdatePayload! @pgUpdateOne
      """
      Update multiple woofs
      """
      woofUpdateMany(
        """
        Filter for updating multiple Woof instances
        """
        filter: WoofFilterInput,
        """
        Input for updating multiple Woof instances
        """
        input: WoofUpdateInput!,
      ): WoofUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Woof
      """
      woofDelete(
        """
        Lookup input for unique Woof deletion
        """
        lookup: WoofLookupInput!,
      ): WoofDeletePayload! @pgDeleteOne
      """
      Delete multiple woofs
      """
      woofDeleteMany(
        """
        Filter for Woof deletion
        """
        filter: WoofFilterInput,
      ): WoofDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn disable_queries_per_view() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql("CREATE SCHEMA a").await;
        api.execute_sql("CREATE SCHEMA b").await;

        let schema = indoc! {r#"
            CREATE TABLE "a"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE VIEW "a"."meow" AS SELECT id FROM "a"."users"
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "b"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE VIEW "b"."woof" AS SELECT id FROM "b"."losers"
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.a.views.meow.columns.id]
        unique = true
        nullable = false

        [schemas.b.views.woof.columns.id]
        unique = true
        nullable = false

        [schemas.b.views.woof]
        enable_queries = false
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.3.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgLookup",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "@pgMutation",
          "@pgReturning",
          "PgKeyType",
          "PgColumnType",
          "PgRelationType"
        ]
      )
      @link(
        url: "https://specs.grafbase.com/composite-schema/v1",
        import: [
          "@lookup",
          "@key"
        ]
      )
      @link(
        url: "https://specs.apollo.dev/federation/v2.3",
        import: [
          "@shareable",
          "@inaccessible"
        ]
      )
      @pgDatabase(name: "default")

    """
    JSON data type
    """
    scalar JSON

    """
    Binary data type
    """
    scalar Bytes

    """
    Big integer data type
    """
    scalar BigInt

    """
    Decimal data type
    """
    scalar Decimal

    """
    Specifies the direction for ordering results.
    """
    enum OrderDirection {
      """
      Specifies an ascending order for a given orderBy argument.
      """
      ASC
      """
      Specifies a descending order for a given orderBy argument.
      """
      DESC
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for User collections
    """
    input UserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: UserFilterInput
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFilterInput]
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Lookup input type for Loser objects for subgraph joins.
    """
    input LoserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Filter input type for Loser collections
    """
    input LoserCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: LoserFilterInput
    }

    """
    Filter input type for Loser objects.
    """
    input LoserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [LoserFilterInput]
      """
      None of the filters must match
      """
      NONE: [LoserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [LoserFilterInput]
    }

    """
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Search filter input for String type.
    """
    input StringFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: String
      """
      The value is not the one given
      """
      ne: String
      """
      The value is greater than the one given
      """
      gt: String
      """
      The value is less than the one given
      """
      lt: String
      """
      The value is greater than, or equal to the one given
      """
      gte: String
      """
      The value is less than, or equal to the one given
      """
      lte: String
      """
      The given input is part of the column value
      """
      like: String
      """
      The value is in the given array of values
      """
      in: [String!]
      """
      The value is not in the given array of values
      """
      nin: [String!]
      """
      A negation of the given filter
      """
      not: StringFilterInput
    }

    """
    Update input for String type.
    """
    input StringUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: String
    }

    """
    Update input for String array type.
    """
    input StringArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [String]
      """
      Append an array value to the column.
      """
      append: [String]
      """
      Prepend an array value to the column.
      """
      prepend: [String]
    }

    """
    Search filter input for BigInt type.
    """
    input BigIntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: BigInt
      """
      The value is not the one given
      """
      ne: BigInt
      """
      The value is greater than the one given
      """
      gt: BigInt
      """
      The value is less than the one given
      """
      lt: BigInt
      """
      The value is greater than, or equal to the one given
      """
      gte: BigInt
      """
      The value is less than, or equal to the one given
      """
      lte: BigInt
      """
      The value is in the given array of values
      """
      in: [BigInt!]
      """
      The value is not in the given array of values
      """
      nin: [BigInt!]
      """
      A negation of the given filter
      """
      not: BigIntFilterInput
    }

    """
    Update input for BigInt type.
    """
    input BigIntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: BigInt
      """
      Increments the value of a field by the specified value.
      """
      increment: BigInt
      """
      Decrements the value of a field by the specified value.
      """
      decrement: BigInt
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: BigInt
      """
      Divides the value of a field by the specified value.
      """
      divide: BigInt
    }

    """
    Update input for BigInt array type.
    """
    input BigIntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [BigInt]
      """
      Append an array value to the column.
      """
      append: [BigInt]
      """
      Prepend an array value to the column.
      """
      prepend: [BigInt]
    }

    """
    Search filter input for Int type.
    """
    input IntFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Int
      """
      The value is not the one given
      """
      ne: Int
      """
      The value is greater than the one given
      """
      gt: Int
      """
      The value is less than the one given
      """
      lt: Int
      """
      The value is greater than, or equal to the one given
      """
      gte: Int
      """
      The value is less than, or equal to the one given
      """
      lte: Int
      """
      The value is in the given array of values
      """
      in: [Int!]
      """
      The value is not in the given array of values
      """
      nin: [Int!]
      """
      A negation of the given filter
      """
      not: IntFilterInput
    }

    """
    Update input for Int type.
    """
    input IntUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Int
      """
      Increments the value of a field by the specified value.
      """
      increment: Int
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Int
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Int
      """
      Divides the value of a field by the specified value.
      """
      divide: Int
    }

    """
    Update input for Int array type.
    """
    input IntArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Int]
      """
      Append an array value to the column.
      """
      append: [Int]
      """
      Prepend an array value to the column.
      """
      prepend: [Int]
    }

    """
    Search filter input for Float type.
    """
    input FloatFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Float
      """
      The value is not the one given
      """
      ne: Float
      """
      The value is greater than the one given
      """
      gt: Float
      """
      The value is less than the one given
      """
      lt: Float
      """
      The value is greater than, or equal to the one given
      """
      gte: Float
      """
      The value is less than, or equal to the one given
      """
      lte: Float
      """
      The value is in the given array of values
      """
      in: [Float!]
      """
      The value is not in the given array of values
      """
      nin: [Float!]
      """
      A negation of the given filter
      """
      not: FloatFilterInput
    }

    """
    Update input for Float type.
    """
    input FloatUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Float
      """
      Increments the value of a field by the specified value.
      """
      increment: Float
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Float
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Float
      """
      Divides the value of a field by the specified value.
      """
      divide: Float
    }

    """
    Update input for Float array type.
    """
    input FloatArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Float]
      """
      Append an array value to the column.
      """
      append: [Float]
      """
      Prepend an array value to the column.
      """
      prepend: [Float]
    }

    """
    Search filter input for Boolean type.
    """
    input BooleanFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Boolean
      """
      The value is not the one given
      """
      ne: Boolean
      """
      The value is greater than the one given
      """
      gt: Boolean
      """
      The value is less than the one given
      """
      lt: Boolean
      """
      The value is greater than, or equal to the one given
      """
      gte: Boolean
      """
      The value is less than, or equal to the one given
      """
      lte: Boolean
      """
      The value is in the given array of values
      """
      in: [Boolean!]
      """
      The value is not in the given array of values
      """
      nin: [Boolean!]
      """
      A negation of the given filter
      """
      not: BooleanFilterInput
    }

    """
    Update input for Boolean type.
    """
    input BooleanUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Boolean
    }

    """
    Update input for Boolean array type.
    """
    input BooleanArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Boolean]
      """
      Append an array value to the column.
      """
      append: [Boolean]
      """
      Prepend an array value to the column.
      """
      prepend: [Boolean]
    }

    """
    Search filter input for Decimal type.
    """
    input DecimalFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Decimal
      """
      The value is not the one given
      """
      ne: Decimal
      """
      The value is greater than the one given
      """
      gt: Decimal
      """
      The value is less than the one given
      """
      lt: Decimal
      """
      The value is greater than, or equal to the one given
      """
      gte: Decimal
      """
      The value is less than, or equal to the one given
      """
      lte: Decimal
      """
      The value is in the given array of values
      """
      in: [Decimal!]
      """
      The value is not in the given array of values
      """
      nin: [Decimal!]
      """
      A negation of the given filter
      """
      not: DecimalFilterInput
    }

    """
    Update input for Decimal type.
    """
    input DecimalUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Decimal
      """
      Increments the value of a field by the specified value.
      """
      increment: Decimal
      """
      Decrements the value of a field by the specified value.
      """
      decrement: Decimal
      """
      Multiplies the value of a field by the specified value.
      """
      multiply: Decimal
      """
      Divides the value of a field by the specified value.
      """
      divide: Decimal
    }

    """
    Update input for Decimal array type.
    """
    input DecimalArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Decimal]
      """
      Append an array value to the column.
      """
      append: [Decimal]
      """
      Prepend an array value to the column.
      """
      prepend: [Decimal]
    }

    """
    Search filter input for Bytes type.
    """
    input BytesFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: Bytes
      """
      The value is not the one given
      """
      ne: Bytes
      """
      The value is greater than the one given
      """
      gt: Bytes
      """
      The value is less than the one given
      """
      lt: Bytes
      """
      The value is greater than, or equal to the one given
      """
      gte: Bytes
      """
      The value is less than, or equal to the one given
      """
      lte: Bytes
      """
      The value is in the given array of values
      """
      in: [Bytes!]
      """
      The value is not in the given array of values
      """
      nin: [Bytes!]
      """
      A negation of the given filter
      """
      not: BytesFilterInput
    }

    """
    Update input for Bytes type.
    """
    input BytesUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: Bytes
    }

    """
    Update input for Bytes array type.
    """
    input BytesArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [Bytes]
      """
      Append an array value to the column.
      """
      append: [Bytes]
      """
      Prepend an array value to the column.
      """
      prepend: [Bytes]
    }

    """
    Search filter input for JSON type.
    """
    input JSONFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: JSON
      """
      The value is not the one given
      """
      ne: JSON
      """
      The value is greater than the one given
      """
      gt: JSON
      """
      The value is less than the one given
      """
      lt: JSON
      """
      The value is greater than, or equal to the one given
      """
      gte: JSON
      """
      The value is less than, or equal to the one given
      """
      lte: JSON
      """
      The value is in the given array of values
      """
      in: [JSON!]
      """
      The value is not in the given array of values
      """
      nin: [JSON!]
      """
      A negation of the given filter
      """
      not: JSONFilterInput
    }

    """
    Update input for JSON type.
    """
    input JSONUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: JSON
      """
      Append JSON value to the column.
      """
      append: JSON
      """
      Prepend JSON value to the column.
      """
      prepend: JSON
      """
      Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.
      """
      deleteKey: String
      """
      Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.
      """
      deleteElem: Int
      """
      Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.
      """
      deleteAtPath: [String!]
    }

    """
    Update input for JSON array type.
    """
    input JSONArrayUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: [JSON]
      """
      Append an array value to the column.
      """
      append: [JSON]
      """
      Prepend an array value to the column.
      """
      prepend: [JSON]
    }

    """
    Search filter input for String array type.
    """
    input StringArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [String]
      """
      The value is not the one given
      """
      ne: [String]
      """
      The value is greater than the one given
      """
      gt: [String]
      """
      The value is less than the one given
      """
      lt: [String]
      """
      The value is greater than, or equal to the one given
      """
      gte: [String]
      """
      The value is less than, or equal to the one given
      """
      lte: [String]
      """
      The value is in the given array of values
      """
      in: [[String]!]
      """
      The value is not in the given array of values
      """
      nin: [[String]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [String]
      """
      Checks if the array is contained within the provided array
      """
      contained: [String]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [String]
      """
      A negation of the given filter
      """
      not: StringArrayFilterInput
    }

    """
    Search filter input for Int array type.
    """
    input IntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Int]
      """
      The value is not the one given
      """
      ne: [Int]
      """
      The value is greater than the one given
      """
      gt: [Int]
      """
      The value is less than the one given
      """
      lt: [Int]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Int]
      """
      The value is less than, or equal to the one given
      """
      lte: [Int]
      """
      The value is in the given array of values
      """
      in: [[Int]!]
      """
      The value is not in the given array of values
      """
      nin: [[Int]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Int]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Int]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Int]
      """
      A negation of the given filter
      """
      not: IntArrayFilterInput
    }

    """
    Search filter input for BigInt array type.
    """
    input BigIntArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [BigInt]
      """
      The value is not the one given
      """
      ne: [BigInt]
      """
      The value is greater than the one given
      """
      gt: [BigInt]
      """
      The value is less than the one given
      """
      lt: [BigInt]
      """
      The value is greater than, or equal to the one given
      """
      gte: [BigInt]
      """
      The value is less than, or equal to the one given
      """
      lte: [BigInt]
      """
      The value is in the given array of values
      """
      in: [[BigInt]!]
      """
      The value is not in the given array of values
      """
      nin: [[BigInt]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [BigInt]
      """
      Checks if the array is contained within the provided array
      """
      contained: [BigInt]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [BigInt]
      """
      A negation of the given filter
      """
      not: BigIntArrayFilterInput
    }

    """
    Search filter input for Decimal array type.
    """
    input DecimalArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Decimal]
      """
      The value is not the one given
      """
      ne: [Decimal]
      """
      The value is greater than the one given
      """
      gt: [Decimal]
      """
      The value is less than the one given
      """
      lt: [Decimal]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Decimal]
      """
      The value is less than, or equal to the one given
      """
      lte: [Decimal]
      """
      The value is in the given array of values
      """
      in: [[Decimal]!]
      """
      The value is not in the given array of values
      """
      nin: [[Decimal]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Decimal]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Decimal]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Decimal]
      """
      A negation of the given filter
      """
      not: DecimalArrayFilterInput
    }

    """
    Search filter input for Float array type.
    """
    input FloatArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Float]
      """
      The value is not the one given
      """
      ne: [Float]
      """
      The value is greater than the one given
      """
      gt: [Float]
      """
      The value is less than the one given
      """
      lt: [Float]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Float]
      """
      The value is less than, or equal to the one given
      """
      lte: [Float]
      """
      The value is in the given array of values
      """
      in: [[Float]!]
      """
      The value is not in the given array of values
      """
      nin: [[Float]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Float]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Float]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Float]
      """
      A negation of the given filter
      """
      not: FloatArrayFilterInput
    }

    """
    Search filter input for Boolean array type.
    """
    input BooleanArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Boolean]
      """
      The value is not the one given
      """
      ne: [Boolean]
      """
      The value is greater than the one given
      """
      gt: [Boolean]
      """
      The value is less than the one given
      """
      lt: [Boolean]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Boolean]
      """
      The value is less than, or equal to the one given
      """
      lte: [Boolean]
      """
      The value is in the given array of values
      """
      in: [[Boolean]!]
      """
      The value is not in the given array of values
      """
      nin: [[Boolean]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Boolean]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Boolean]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Boolean]
      """
      A negation of the given filter
      """
      not: BooleanArrayFilterInput
    }

    """
    Search filter input for Bytes array type.
    """
    input BytesArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [Bytes]
      """
      The value is not the one given
      """
      ne: [Bytes]
      """
      The value is greater than the one given
      """
      gt: [Bytes]
      """
      The value is less than the one given
      """
      lt: [Bytes]
      """
      The value is greater than, or equal to the one given
      """
      gte: [Bytes]
      """
      The value is less than, or equal to the one given
      """
      lte: [Bytes]
      """
      The value is in the given array of values
      """
      in: [[Bytes]!]
      """
      The value is not in the given array of values
      """
      nin: [[Bytes]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [Bytes]
      """
      Checks if the array is contained within the provided array
      """
      contained: [Bytes]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [Bytes]
      """
      A negation of the given filter
      """
      not: BytesArrayFilterInput
    }

    """
    Search filter input for JSON array type.
    """
    input JSONArrayFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: [JSON]
      """
      The value is not the one given
      """
      ne: [JSON]
      """
      The value is greater than the one given
      """
      gt: [JSON]
      """
      The value is less than the one given
      """
      lt: [JSON]
      """
      The value is greater than, or equal to the one given
      """
      gte: [JSON]
      """
      The value is less than, or equal to the one given
      """
      lte: [JSON]
      """
      The value is in the given array of values
      """
      in: [[JSON]!]
      """
      The value is not in the given array of values
      """
      nin: [[JSON]!]
      """
      Checks if the array contains all elements of the provided array
      """
      contains: [JSON]
      """
      Checks if the array is contained within the provided array
      """
      contained: [JSON]
      """
      Checks if the array has any elements in common with the provided array
      """
      overlaps: [JSON]
      """
      A negation of the given filter
      """
      not: JSONArrayFilterInput
    }

    """
    Information about pagination in a collection of objects
    """
    type PageInfo
      @shareable
    {
      """
      When paginating backwards, are there more items?
      """
      hasPreviousPage: Boolean! @shareable
      """
      When paginating forwards, are there more items?
      """
      hasNextPage: Boolean! @shareable
      """
      The cursor of the first item in the page
      """
      startCursor: String! @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created User object
    """
    type UserReturning
      @pgReturning(type: "User")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one User
    """
    type UserCreatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many users
    """
    type UserCreateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one User
    """
    type UserUpdatePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many users
    """
    type UserUpdateManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one User
    """
    type UserDeletePayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many users
    """
    type UserDeleteManyPayload
      @pgMutation(type: "User")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserEdge {
      """
      The item at the end of the edge
      """
      node: User! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for User
    """
    type UserConnection
      @pgConnection(type: "User")
    {
      """
      A list of edges
      """
      edges: [UserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type containing fields of the mutated or created Loser object
    """
    type LoserReturning
      @pgReturning(type: "Loser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when creating one Loser
    """
    type LoserCreatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating many losers
    """
    type LoserCreateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Loser
    """
    type LoserUpdatePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating many losers
    """
    type LoserUpdateManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Loser
    """
    type LoserDeletePayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: LoserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many losers
    """
    type LoserDeleteManyPayload
      @pgMutation(type: "Loser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [LoserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type LoserEdge {
      """
      The item at the end of the edge
      """
      node: Loser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    The connection type for Loser
    """
    type LoserConnection
      @pgConnection(type: "Loser")
    {
      """
      A list of edges
      """
      edges: [LoserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      """
      Query a unique User
      """
      user(
        """
        Input for unique User lookup
        """
        lookup: UserLookupInput!,
      ): User @pgSelectOne
      """
      Query and paginate multiple users
      """
      users(
        """
        Filter for User
        """
        filter: UserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [UserOrderByInput!],
      ): UserConnection! @pgSelectMany
      """
      Lookup multiple users for subgraph joins
      """
      userLookup(
        """
        Filter users with an array of keys
        """
        lookup: UserManyLookupInput @inaccessible,
      ): [User]! @pgLookup @lookup @inaccessible
      """
      Query a unique Loser
      """
      loser(
        """
        Input for unique Loser lookup
        """
        lookup: LoserLookupInput!,
      ): Loser @pgSelectOne
      """
      Query and paginate multiple losers
      """
      losers(
        """
        Filter for Loser
        """
        filter: LoserFilterInput,
        """
        Limit the number of results, from the beginning
        """
        first: Int,
        """
        Limit the number of results, from the end
        """
        last: Int,
        """
        Cursor for pagination, select items before the cursor. Use together with `last`.
        """
        before: String,
        """
        Cursor for pagination, select items after the cursor. Use together with `first`.
        """
        after: String,
        """
        Order the results by selected fields
        """
        orderBy: [LoserOrderByInput!],
      ): LoserConnection! @pgSelectMany
      """
      Lookup multiple losers for subgraph joins
      """
      loserLookup(
        """
        Filter losers with an array of keys
        """
        lookup: LoserManyLookupInput @inaccessible,
      ): [Loser]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single User
      """
      userCreate(
        """
        Input for creating a single User
        """
        input: UserCreateInput!,
      ): UserCreatePayload! @pgInsertOne
      """
      Create multiple users
      """
      userCreateMany(
        """
        Input for creating multiple User instances
        """
        input: [UserCreateInput!]!,
      ): UserCreateManyPayload! @pgInsertMany
      """
      Update a unique User
      """
      userUpdate(
        """
        Lookup input for unique User update
        """
        lookup: UserLookupInput!,
        """
        Input for updating a User
        """
        input: UserUpdateInput!,
      ): UserUpdatePayload! @pgUpdateOne
      """
      Update multiple users
      """
      userUpdateMany(
        """
        Filter for updating multiple User instances
        """
        filter: UserFilterInput,
        """
        Input for updating multiple User instances
        """
        input: UserUpdateInput!,
      ): UserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique User
      """
      userDelete(
        """
        Lookup input for unique User deletion
        """
        lookup: UserLookupInput!,
      ): UserDeletePayload! @pgDeleteOne
      """
      Delete multiple users
      """
      userDeleteMany(
        """
        Filter for User deletion
        """
        filter: UserFilterInput,
      ): UserDeleteManyPayload! @pgDeleteMany
      """
      Create a single Loser
      """
      loserCreate(
        """
        Input for creating a single Loser
        """
        input: LoserCreateInput!,
      ): LoserCreatePayload! @pgInsertOne
      """
      Create multiple losers
      """
      loserCreateMany(
        """
        Input for creating multiple Loser instances
        """
        input: [LoserCreateInput!]!,
      ): LoserCreateManyPayload! @pgInsertMany
      """
      Update a unique Loser
      """
      loserUpdate(
        """
        Lookup input for unique Loser update
        """
        lookup: LoserLookupInput!,
        """
        Input for updating a Loser
        """
        input: LoserUpdateInput!,
      ): LoserUpdatePayload! @pgUpdateOne
      """
      Update multiple losers
      """
      loserUpdateMany(
        """
        Filter for updating multiple Loser instances
        """
        filter: LoserFilterInput,
        """
        Input for updating multiple Loser instances
        """
        input: LoserUpdateInput!,
      ): LoserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Loser
      """
      loserDelete(
        """
        Lookup input for unique Loser deletion
        """
        lookup: LoserLookupInput!,
      ): LoserDeletePayload! @pgDeleteOne
      """
      Delete multiple losers
      """
      loserDeleteMany(
        """
        Filter for Loser deletion
        """
        filter: LoserFilterInput,
      ): LoserDeleteManyPayload! @pgDeleteMany
    }
    "#);
}
