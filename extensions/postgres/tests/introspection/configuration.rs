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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      user: User! @pgRelation(name: "b_to_a", fields: ["id"], references: ["id"])
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      loser: Loser @pgRelation(name: "b_to_a")
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new Woof
    """
    input WoofCreateInput {
      _: Boolean
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
    Input type to select a unique Woof
    """
    input WoofLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Woof results.
    """
    input WoofOrderByInput @oneOf {
      """
      Order woofs by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Woof
    """
    input WoofUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Woof
      @key(fields: "id")
      @pgTable(name: "woof", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new Woof
    """
    input WoofCreateInput {
      _: Boolean
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
    Input type to select a unique Woof
    """
    input WoofLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Woof results.
    """
    input WoofOrderByInput @oneOf {
      """
      Order woofs by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Woof
    """
    input WoofUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Woof
      @key(fields: "id")
      @pgTable(name: "woof", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
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
    Input type to select a unique Woof
    """
    input WoofLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Woof results.
    """
    input WoofOrderByInput @oneOf {
      """
      Order woofs by id
      """
      id: OrderDirection
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Woof
      @key(fields: "id")
      @pgTable(name: "woof", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new Woof
    """
    input WoofCreateInput {
      _: Boolean
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
    Input type to select a unique Woof
    """
    input WoofLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Input for updating an existing Woof
    """
    input WoofUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Woof
      @key(fields: "id")
      @pgTable(name: "woof", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
async fn disable_queries_globally_for_views() {
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
        enable_queries = false

        [schemas.a.views.meow.columns.id]
        unique = true
        nullable = false

        [schemas.b.views.woof.columns.id]
        unique = true
        nullable = false
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Meow
      @key(fields: "id")
      @pgTable(name: "meow", schema: "a", kind: VIEW)
      @pgKey(fields: ["id"], type: UNIQUE)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Woof
      @key(fields: "id")
      @pgTable(name: "woof", schema: "b", kind: VIEW)
      @pgKey(fields: ["id"], type: UNIQUE)
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Filter input type for Meow objects.
    """
    input MeowFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [MeowFilterInput]
      """
      None of the filters must match
      """
      NONE: [MeowFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [MeowFilterInput]
    }

    """
    Input type to select a unique Meow
    """
    input MeowLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for Meow objects for subgraph joins.
    """
    input MeowManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for Meow results.
    """
    input MeowOrderByInput @oneOf {
      """
      Order meows by id
      """
      id: OrderDirection
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "b")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Meow
      @key(fields: "id")
      @pgTable(name: "meow", schema: "a", kind: VIEW)
      @pgKey(fields: ["id"], type: UNIQUE)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for Meow
    """
    type MeowConnection
      @pgConnection(type: "Meow")
    {
      """
      A list of edges
      """
      edges: [MeowEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type MeowEdge {
      """
      The item at the end of the edge
      """
      node: Meow! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "a")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Woof
      @key(fields: "id")
      @pgTable(name: "woof", schema: "b", kind: VIEW)
      @pgKey(fields: ["id"], type: UNIQUE)
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
      Query a unique Meow
      """
      meow(
        """
        Input for unique Meow lookup
        """
        lookup: MeowLookupInput!,
      ): Meow @pgSelectOne
      """
      Query and paginate multiple meows
      """
      meows(
        """
        Filter for Meow
        """
        filter: MeowFilterInput,
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
        orderBy: [MeowOrderByInput!],
      ): MeowConnection! @pgSelectMany
      """
      Lookup multiple meows for subgraph joins
      """
      meowLookup(
        """
        Filter meows with an array of keys
        """
        lookup: MeowManyLookupInput @inaccessible,
      ): [Meow]! @pgLookup @lookup @inaccessible
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
async fn schema_allowlist() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "public"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "private";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "private"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "other";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "other"."posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"
        schema_allowlist = ["public"]
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
    }
    "#);
}

#[tokio::test]
async fn schema_allowlist_should_hide_relations() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "public"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "private";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "private"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                CONSTRAINT fk_losers_users FOREIGN KEY (id) REFERENCES "public"."users" (id)
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"
        schema_allowlist = ["public"]
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
    }
    "#);
}

#[tokio::test]
async fn schema_denylist() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "public"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "private";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "private"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "other";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "other"."posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"
        schema_denylist = ["public"]
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new Post
    """
    input PostCreateInput {
      _: Boolean
    }

    """
    Filter input type for Post objects.
    """
    input PostFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [PostFilterInput]
      """
      None of the filters must match
      """
      NONE: [PostFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [PostFilterInput]
    }

    """
    Input type to select a unique Post
    """
    input PostLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for Post objects for subgraph joins.
    """
    input PostManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for Post results.
    """
    input PostOrderByInput @oneOf {
      """
      Order posts by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Post
    """
    input PostUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "private")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type Post
      @key(fields: "id")
      @pgTable(name: "posts", schema: "other")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for Post
    """
    type PostConnection
      @pgConnection(type: "Post")
    {
      """
      A list of edges
      """
      edges: [PostEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many posts
    """
    type PostCreateManyPayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one Post
    """
    type PostCreatePayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many posts
    """
    type PostDeleteManyPayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Post
    """
    type PostDeletePayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type PostEdge {
      """
      The item at the end of the edge
      """
      node: Post! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created Post object
    """
    type PostReturning
      @pgReturning(type: "Post")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when updating many posts
    """
    type PostUpdateManyPayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Post
    """
    type PostUpdatePayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type Query {
      """
      Query a unique Post
      """
      post(
        """
        Input for unique Post lookup
        """
        lookup: PostLookupInput!,
      ): Post @pgSelectOne
      """
      Query and paginate multiple posts
      """
      posts(
        """
        Filter for Post
        """
        filter: PostFilterInput,
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
        orderBy: [PostOrderByInput!],
      ): PostConnection! @pgSelectMany
      """
      Lookup multiple posts for subgraph joins
      """
      postLookup(
        """
        Filter posts with an array of keys
        """
        lookup: PostManyLookupInput @inaccessible,
      ): [Post]! @pgLookup @lookup @inaccessible
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
      Create a single Post
      """
      postCreate(
        """
        Input for creating a single Post
        """
        input: PostCreateInput!,
      ): PostCreatePayload! @pgInsertOne
      """
      Create multiple posts
      """
      postCreateMany(
        """
        Input for creating multiple Post instances
        """
        input: [PostCreateInput!]!,
      ): PostCreateManyPayload! @pgInsertMany
      """
      Update a unique Post
      """
      postUpdate(
        """
        Lookup input for unique Post update
        """
        lookup: PostLookupInput!,
        """
        Input for updating a Post
        """
        input: PostUpdateInput!,
      ): PostUpdatePayload! @pgUpdateOne
      """
      Update multiple posts
      """
      postUpdateMany(
        """
        Filter for updating multiple Post instances
        """
        filter: PostFilterInput,
        """
        Input for updating multiple Post instances
        """
        input: PostUpdateInput!,
      ): PostUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Post
      """
      postDelete(
        """
        Lookup input for unique Post deletion
        """
        lookup: PostLookupInput!,
      ): PostDeletePayload! @pgDeleteOne
      """
      Delete multiple posts
      """
      postDeleteMany(
        """
        Filter for Post deletion
        """
        filter: PostFilterInput,
      ): PostDeleteManyPayload! @pgDeleteMany
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
async fn schema_allowlist_and_denylist_precedence() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "public"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "private";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "private"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "other";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "other"."posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    // Test that denylist takes precedence over allowlist
    // Allowlist includes public and private, but denylist excludes private
    // So only public should be included
    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"
        schema_allowlist = ["public", "private"]
        schema_denylist = ["private"]
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
    }
    "#);
}

#[tokio::test]
async fn schema_empty_allowlist() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "public"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "private";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "private"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    // An empty allowlist should exclude all schemas
    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"
        schema_allowlist = []
    "#};

    let result = api.introspect_with_config(config).await;

    // Expect only the schema extension with no types
    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    "#);
}

#[tokio::test]
async fn schema_multiple_allowlists() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "public"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "private";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "private"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "other";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "other"."posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "admin";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "admin"."settings" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    // Include only private and other schemas
    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"
        schema_allowlist = ["private", "other"]
    "#};

    let result = api.introspect_with_config(config).await;

    // Expect only private.losers and other.posts
    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new Post
    """
    input PostCreateInput {
      _: Boolean
    }

    """
    Filter input type for Post objects.
    """
    input PostFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [PostFilterInput]
      """
      None of the filters must match
      """
      NONE: [PostFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [PostFilterInput]
    }

    """
    Input type to select a unique Post
    """
    input PostLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for Post objects for subgraph joins.
    """
    input PostManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for Post results.
    """
    input PostOrderByInput @oneOf {
      """
      Order posts by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Post
    """
    input PostUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "private")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type Post
      @key(fields: "id")
      @pgTable(name: "posts", schema: "other")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for Post
    """
    type PostConnection
      @pgConnection(type: "Post")
    {
      """
      A list of edges
      """
      edges: [PostEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many posts
    """
    type PostCreateManyPayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one Post
    """
    type PostCreatePayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many posts
    """
    type PostDeleteManyPayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Post
    """
    type PostDeletePayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type PostEdge {
      """
      The item at the end of the edge
      """
      node: Post! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created Post object
    """
    type PostReturning
      @pgReturning(type: "Post")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when updating many posts
    """
    type PostUpdateManyPayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Post
    """
    type PostUpdatePayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type Query {
      """
      Query a unique Post
      """
      post(
        """
        Input for unique Post lookup
        """
        lookup: PostLookupInput!,
      ): Post @pgSelectOne
      """
      Query and paginate multiple posts
      """
      posts(
        """
        Filter for Post
        """
        filter: PostFilterInput,
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
        orderBy: [PostOrderByInput!],
      ): PostConnection! @pgSelectMany
      """
      Lookup multiple posts for subgraph joins
      """
      postLookup(
        """
        Filter posts with an array of keys
        """
        lookup: PostManyLookupInput @inaccessible,
      ): [Post]! @pgLookup @lookup @inaccessible
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
      Create a single Post
      """
      postCreate(
        """
        Input for creating a single Post
        """
        input: PostCreateInput!,
      ): PostCreatePayload! @pgInsertOne
      """
      Create multiple posts
      """
      postCreateMany(
        """
        Input for creating multiple Post instances
        """
        input: [PostCreateInput!]!,
      ): PostCreateManyPayload! @pgInsertMany
      """
      Update a unique Post
      """
      postUpdate(
        """
        Lookup input for unique Post update
        """
        lookup: PostLookupInput!,
        """
        Input for updating a Post
        """
        input: PostUpdateInput!,
      ): PostUpdatePayload! @pgUpdateOne
      """
      Update multiple posts
      """
      postUpdateMany(
        """
        Filter for updating multiple Post instances
        """
        filter: PostFilterInput,
        """
        Input for updating multiple Post instances
        """
        input: PostUpdateInput!,
      ): PostUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Post
      """
      postDelete(
        """
        Lookup input for unique Post deletion
        """
        lookup: PostLookupInput!,
      ): PostDeletePayload! @pgDeleteOne
      """
      Delete multiple posts
      """
      postDeleteMany(
        """
        Filter for Post deletion
        """
        filter: PostFilterInput,
      ): PostDeleteManyPayload! @pgDeleteMany
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
async fn schema_multiple_denylists() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "public"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "private";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "private"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "other";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "other"."posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "admin";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "admin"."settings" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    // Exclude private and other schemas
    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"
        schema_denylist = ["private", "other"]
    "#};

    let result = api.introspect_with_config(config).await;

    // Expect only public.users and admin.settings
    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Setting
    """
    input SettingCreateInput {
      _: Boolean
    }

    """
    Filter input type for Setting objects.
    """
    input SettingFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [SettingFilterInput]
      """
      None of the filters must match
      """
      NONE: [SettingFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [SettingFilterInput]
    }

    """
    Input type to select a unique Setting
    """
    input SettingLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for Setting objects for subgraph joins.
    """
    input SettingManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for Setting results.
    """
    input SettingOrderByInput @oneOf {
      """
      Order settings by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Setting
    """
    input SettingUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type Setting
      @key(fields: "id")
      @pgTable(name: "settings", schema: "admin")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for Setting
    """
    type SettingConnection
      @pgConnection(type: "Setting")
    {
      """
      A list of edges
      """
      edges: [SettingEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many settings
    """
    type SettingCreateManyPayload
      @pgMutation(type: "Setting")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [SettingReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one Setting
    """
    type SettingCreatePayload
      @pgMutation(type: "Setting")
    {
      """
      Returned item(s) from the mutation
      """
      returning: SettingReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many settings
    """
    type SettingDeleteManyPayload
      @pgMutation(type: "Setting")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [SettingReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Setting
    """
    type SettingDeletePayload
      @pgMutation(type: "Setting")
    {
      """
      Returned item(s) from the mutation
      """
      returning: SettingReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type SettingEdge {
      """
      The item at the end of the edge
      """
      node: Setting! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created Setting object
    """
    type SettingReturning
      @pgReturning(type: "Setting")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when updating many settings
    """
    type SettingUpdateManyPayload
      @pgMutation(type: "Setting")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [SettingReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Setting
    """
    type SettingUpdatePayload
      @pgMutation(type: "Setting")
    {
      """
      Returned item(s) from the mutation
      """
      returning: SettingReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Query {
      """
      Query a unique Setting
      """
      setting(
        """
        Input for unique Setting lookup
        """
        lookup: SettingLookupInput!,
      ): Setting @pgSelectOne
      """
      Query and paginate multiple settings
      """
      settings(
        """
        Filter for Setting
        """
        filter: SettingFilterInput,
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
        orderBy: [SettingOrderByInput!],
      ): SettingConnection! @pgSelectMany
      """
      Lookup multiple settings for subgraph joins
      """
      settingLookup(
        """
        Filter settings with an array of keys
        """
        lookup: SettingManyLookupInput @inaccessible,
      ): [Setting]! @pgLookup @lookup @inaccessible
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
      Create a single Setting
      """
      settingCreate(
        """
        Input for creating a single Setting
        """
        input: SettingCreateInput!,
      ): SettingCreatePayload! @pgInsertOne
      """
      Create multiple settings
      """
      settingCreateMany(
        """
        Input for creating multiple Setting instances
        """
        input: [SettingCreateInput!]!,
      ): SettingCreateManyPayload! @pgInsertMany
      """
      Update a unique Setting
      """
      settingUpdate(
        """
        Lookup input for unique Setting update
        """
        lookup: SettingLookupInput!,
        """
        Input for updating a Setting
        """
        input: SettingUpdateInput!,
      ): SettingUpdatePayload! @pgUpdateOne
      """
      Update multiple settings
      """
      settingUpdateMany(
        """
        Filter for updating multiple Setting instances
        """
        filter: SettingFilterInput,
        """
        Input for updating multiple Setting instances
        """
        input: SettingUpdateInput!,
      ): SettingUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Setting
      """
      settingDelete(
        """
        Lookup input for unique Setting deletion
        """
        lookup: SettingLookupInput!,
      ): SettingDeletePayload! @pgDeleteOne
      """
      Delete multiple settings
      """
      settingDeleteMany(
        """
        Filter for Setting deletion
        """
        filter: SettingFilterInput,
      ): SettingDeleteManyPayload! @pgDeleteMany
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
async fn schema_denylist_should_hide_relations() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "public"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "private";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "private"."losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                CONSTRAINT fk_losers_users FOREIGN KEY (id) REFERENCES "public"."users" (id)
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    // Denylist public schema
    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"
        schema_denylist = ["public"]
    "#};

    let result = api.introspect_with_config(config).await;

    // Expect private.losers but without the relation to public.users
    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers", schema: "private")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
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
async fn table_allowlist() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"

        [schemas.public]
        table_allowlist = ["losers"]
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Loser
    """
    input LoserCreateInput {
      _: Boolean
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
    Input type to select a unique Loser
    """
    input LoserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for Loser results.
    """
    input LoserOrderByInput @oneOf {
      """
      Order losers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Loser
    """
    input LoserUpdateInput {
      _: Boolean
    }

    type Loser
      @key(fields: "id")
      @pgTable(name: "losers")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
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
async fn table_denylist() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"

        [schemas.public]
        table_denylist = ["losers"]
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Post
    """
    input PostCreateInput {
      _: Boolean
    }

    """
    Filter input type for Post objects.
    """
    input PostFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [PostFilterInput]
      """
      None of the filters must match
      """
      NONE: [PostFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [PostFilterInput]
    }

    """
    Input type to select a unique Post
    """
    input PostLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for Post objects for subgraph joins.
    """
    input PostManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for Post results.
    """
    input PostOrderByInput @oneOf {
      """
      Order posts by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Post
    """
    input PostUpdateInput {
      _: Boolean
    }

    """
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type Post
      @key(fields: "id")
      @pgTable(name: "posts")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for Post
    """
    type PostConnection
      @pgConnection(type: "Post")
    {
      """
      A list of edges
      """
      edges: [PostEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many posts
    """
    type PostCreateManyPayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one Post
    """
    type PostCreatePayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many posts
    """
    type PostDeleteManyPayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Post
    """
    type PostDeletePayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type PostEdge {
      """
      The item at the end of the edge
      """
      node: Post! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created Post object
    """
    type PostReturning
      @pgReturning(type: "Post")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when updating many posts
    """
    type PostUpdateManyPayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Post
    """
    type PostUpdatePayload
      @pgMutation(type: "Post")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Query {
      """
      Query a unique Post
      """
      post(
        """
        Input for unique Post lookup
        """
        lookup: PostLookupInput!,
      ): Post @pgSelectOne
      """
      Query and paginate multiple posts
      """
      posts(
        """
        Filter for Post
        """
        filter: PostFilterInput,
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
        orderBy: [PostOrderByInput!],
      ): PostConnection! @pgSelectMany
      """
      Lookup multiple posts for subgraph joins
      """
      postLookup(
        """
        Filter posts with an array of keys
        """
        lookup: PostManyLookupInput @inaccessible,
      ): [Post]! @pgLookup @lookup @inaccessible
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
      Create a single Post
      """
      postCreate(
        """
        Input for creating a single Post
        """
        input: PostCreateInput!,
      ): PostCreatePayload! @pgInsertOne
      """
      Create multiple posts
      """
      postCreateMany(
        """
        Input for creating multiple Post instances
        """
        input: [PostCreateInput!]!,
      ): PostCreateManyPayload! @pgInsertMany
      """
      Update a unique Post
      """
      postUpdate(
        """
        Lookup input for unique Post update
        """
        lookup: PostLookupInput!,
        """
        Input for updating a Post
        """
        input: PostUpdateInput!,
      ): PostUpdatePayload! @pgUpdateOne
      """
      Update multiple posts
      """
      postUpdateMany(
        """
        Filter for updating multiple Post instances
        """
        filter: PostFilterInput,
        """
        Input for updating multiple Post instances
        """
        input: PostUpdateInput!,
      ): PostUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Post
      """
      postDelete(
        """
        Lookup input for unique Post deletion
        """
        lookup: PostLookupInput!,
      ): PostDeletePayload! @pgDeleteOne
      """
      Delete multiple posts
      """
      postDeleteMany(
        """
        Filter for Post deletion
        """
        filter: PostFilterInput,
      ): PostDeleteManyPayload! @pgDeleteMany
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
async fn allowlist_denylist_precedence() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "losers" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"

        [schemas.public]
        table_allowlist = ["users", "posts"]
        table_denylist = ["posts"]
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
    }
    "#);
}

#[tokio::test]
async fn allowlist_with_foreign_key_relationships() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                user_id INT,
                CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id)
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "comments" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                post_id INT,
                CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES posts(id)
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"

        [schemas.public]
        table_denylist = ["posts"]
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Comment
    """
    input CommentCreateInput {
      """
      Set field value for postId
      """
      postId: Int
    }

    """
    Filter input type for Comment objects.
    """
    input CommentFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the given postId
      """
      postId: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [CommentFilterInput]
      """
      None of the filters must match
      """
      NONE: [CommentFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [CommentFilterInput]
    }

    """
    Input type to select a unique Comment
    """
    input CommentLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for Comment objects for subgraph joins.
    """
    input CommentManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for Comment results.
    """
    input CommentOrderByInput @oneOf {
      """
      Order comments by id
      """
      id: OrderDirection
      """
      Order comments by postId
      """
      postId: OrderDirection
    }

    """
    Input for updating an existing Comment
    """
    input CommentUpdateInput {
      """
      Update field value for postId
      """
      postId: IntUpdateInput
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
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    type Comment
      @key(fields: "id")
      @pgTable(name: "comments")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      postId: Int @pgColumn(name: "post_id", type: INT)
    }

    """
    The connection type for Comment
    """
    type CommentConnection
      @pgConnection(type: "Comment")
    {
      """
      A list of edges
      """
      edges: [CommentEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many comments
    """
    type CommentCreateManyPayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one Comment
    """
    type CommentCreatePayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many comments
    """
    type CommentDeleteManyPayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Comment
    """
    type CommentDeletePayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type CommentEdge {
      """
      The item at the end of the edge
      """
      node: Comment! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created Comment object
    """
    type CommentReturning
      @pgReturning(type: "Comment")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
      """
      The value of the postId field
      """
      postId: Int
    }

    """
    Return type when updating many comments
    """
    type CommentUpdateManyPayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Comment
    """
    type CommentUpdatePayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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

    type Query {
      """
      Query a unique Comment
      """
      comment(
        """
        Input for unique Comment lookup
        """
        lookup: CommentLookupInput!,
      ): Comment @pgSelectOne
      """
      Query and paginate multiple comments
      """
      comments(
        """
        Filter for Comment
        """
        filter: CommentFilterInput,
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
        orderBy: [CommentOrderByInput!],
      ): CommentConnection! @pgSelectMany
      """
      Lookup multiple comments for subgraph joins
      """
      commentLookup(
        """
        Filter comments with an array of keys
        """
        lookup: CommentManyLookupInput @inaccessible,
      ): [Comment]! @pgLookup @lookup @inaccessible
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
      Create a single Comment
      """
      commentCreate(
        """
        Input for creating a single Comment
        """
        input: CommentCreateInput!,
      ): CommentCreatePayload! @pgInsertOne
      """
      Create multiple comments
      """
      commentCreateMany(
        """
        Input for creating multiple Comment instances
        """
        input: [CommentCreateInput!]!,
      ): CommentCreateManyPayload! @pgInsertMany
      """
      Update a unique Comment
      """
      commentUpdate(
        """
        Lookup input for unique Comment update
        """
        lookup: CommentLookupInput!,
        """
        Input for updating a Comment
        """
        input: CommentUpdateInput!,
      ): CommentUpdatePayload! @pgUpdateOne
      """
      Update multiple comments
      """
      commentUpdateMany(
        """
        Filter for updating multiple Comment instances
        """
        filter: CommentFilterInput,
        """
        Input for updating multiple Comment instances
        """
        input: CommentUpdateInput!,
      ): CommentUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Comment
      """
      commentDelete(
        """
        Lookup input for unique Comment deletion
        """
        lookup: CommentLookupInput!,
      ): CommentDeletePayload! @pgDeleteOne
      """
      Delete multiple comments
      """
      commentDeleteMany(
        """
        Filter for Comment deletion
        """
        filter: CommentFilterInput,
      ): CommentDeleteManyPayload! @pgDeleteMany
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
async fn multiple_schema_support() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE SCHEMA "schema1";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "schema1"."users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "schema1"."posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE SCHEMA "schema2";
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "schema2"."comments" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "schema2"."tags" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"

        [schemas.schema1]
        table_allowlist = ["users"]

        [schemas.schema2]
        table_denylist = ["tags"]
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Input for creating a new Comment
    """
    input CommentCreateInput {
      _: Boolean
    }

    """
    Filter input type for Comment objects.
    """
    input CommentFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [CommentFilterInput]
      """
      None of the filters must match
      """
      NONE: [CommentFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [CommentFilterInput]
    }

    """
    Input type to select a unique Comment
    """
    input CommentLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for Comment objects for subgraph joins.
    """
    input CommentManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for Comment results.
    """
    input CommentOrderByInput @oneOf {
      """
      Order comments by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Comment
    """
    input CommentUpdateInput {
      _: Boolean
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
    Input for creating a new User
    """
    input UserCreateInput {
      _: Boolean
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
    Input type to select a unique User
    """
    input UserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
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
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      _: Boolean
    }

    type Comment
      @key(fields: "id")
      @pgTable(name: "comments", schema: "schema2")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for Comment
    """
    type CommentConnection
      @pgConnection(type: "Comment")
    {
      """
      A list of edges
      """
      edges: [CommentEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many comments
    """
    type CommentCreateManyPayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one Comment
    """
    type CommentCreatePayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many comments
    """
    type CommentDeleteManyPayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Comment
    """
    type CommentDeletePayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type CommentEdge {
      """
      The item at the end of the edge
      """
      node: Comment! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created Comment object
    """
    type CommentReturning
      @pgReturning(type: "Comment")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when updating many comments
    """
    type CommentUpdateManyPayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Comment
    """
    type CommentUpdatePayload
      @pgMutation(type: "Comment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type User
      @key(fields: "id")
      @pgTable(name: "users", schema: "schema1")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      Query a unique Comment
      """
      comment(
        """
        Input for unique Comment lookup
        """
        lookup: CommentLookupInput!,
      ): Comment @pgSelectOne
      """
      Query and paginate multiple comments
      """
      comments(
        """
        Filter for Comment
        """
        filter: CommentFilterInput,
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
        orderBy: [CommentOrderByInput!],
      ): CommentConnection! @pgSelectMany
      """
      Lookup multiple comments for subgraph joins
      """
      commentLookup(
        """
        Filter comments with an array of keys
        """
        lookup: CommentManyLookupInput @inaccessible,
      ): [Comment]! @pgLookup @lookup @inaccessible
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
      Create a single Comment
      """
      commentCreate(
        """
        Input for creating a single Comment
        """
        input: CommentCreateInput!,
      ): CommentCreatePayload! @pgInsertOne
      """
      Create multiple comments
      """
      commentCreateMany(
        """
        Input for creating multiple Comment instances
        """
        input: [CommentCreateInput!]!,
      ): CommentCreateManyPayload! @pgInsertMany
      """
      Update a unique Comment
      """
      commentUpdate(
        """
        Lookup input for unique Comment update
        """
        lookup: CommentLookupInput!,
        """
        Input for updating a Comment
        """
        input: CommentUpdateInput!,
      ): CommentUpdatePayload! @pgUpdateOne
      """
      Update multiple comments
      """
      commentUpdateMany(
        """
        Filter for updating multiple Comment instances
        """
        filter: CommentFilterInput,
        """
        Input for updating multiple Comment instances
        """
        input: CommentUpdateInput!,
      ): CommentUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Comment
      """
      commentDelete(
        """
        Lookup input for unique Comment deletion
        """
        lookup: CommentLookupInput!,
      ): CommentDeletePayload! @pgDeleteOne
      """
      Delete multiple comments
      """
      commentDeleteMany(
        """
        Filter for Comment deletion
        """
        filter: CommentFilterInput,
      ): CommentDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn empty_allowlist_behavior() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;

        let table = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(table).await;

        let view = indoc! {r#"
            CREATE VIEW "users_posts" AS
                SELECT users.id AS user_id, posts.id AS post_id
                FROM "users" JOIN "posts"
                ON "users"."id" = "posts"."id"
        "#};

        api.execute_sql(view).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.4.7"

        [schemas.public.views.users_posts.columns.user_id]
        unique = true
        nullable = false

        [schemas.public.views.users_posts.columns.post_id]
        unique = true
        nullable = false

        [schemas.public]
        table_allowlist = []
    "#};

    let result = api.introspect_with_config(config).await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.4.7",
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
        url: "https://specs.grafbase.com/composite-schemas/v1",
        import: [
          "@lookup",
          "@key",
          "@derive",
          "@is"
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
    Filter input type for UsersPost objects.
    """
    input UsersPostFilterInput @oneOf {
      """
      Filter by the given userId
      """
      userId: IntFilterInput
      """
      Filter by the given postId
      """
      postId: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [UsersPostFilterInput]
      """
      None of the filters must match
      """
      NONE: [UsersPostFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UsersPostFilterInput]
    }

    """
    Input type to select a unique UsersPost
    """
    input UsersPostLookupInput @oneOf {
      """
      Select by the 'postId' field
      """
      postId: Int
      """
      Select by the 'userId' field
      """
      userId: Int
    }

    """
    Lookup input type for UsersPost objects for subgraph joins.
    """
    input UsersPostManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'postId' field
      """
      postId: [Int!] @inaccessible
      """
      Select by the 'userId' field
      """
      userId: [Int!] @inaccessible
    }

    """
    Specifies the ordering for UsersPost results.
    """
    input UsersPostOrderByInput @oneOf {
      """
      Order usersPosts by userId
      """
      userId: OrderDirection
      """
      Order usersPosts by postId
      """
      postId: OrderDirection
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
      startCursor: String @shareable
      """
      The cursor of the last item in the page
      """
      endCursor: String @shareable
    }

    type UsersPost
      @key(fields: "postId")
      @key(fields: "userId")
      @pgTable(name: "users_posts", kind: VIEW)
      @pgKey(fields: ["postId"], type: UNIQUE)
      @pgKey(fields: ["userId"], type: UNIQUE)
    {
      userId: Int! @pgColumn(name: "user_id", type: INT)
      postId: Int! @pgColumn(name: "post_id", type: INT)
    }

    """
    The connection type for UsersPost
    """
    type UsersPostConnection
      @pgConnection(type: "UsersPost")
    {
      """
      A list of edges
      """
      edges: [UsersPostEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UsersPostEdge {
      """
      The item at the end of the edge
      """
      node: UsersPost! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    type Query {
      """
      Query a unique UsersPost
      """
      usersPost(
        """
        Input for unique UsersPost lookup
        """
        lookup: UsersPostLookupInput!,
      ): UsersPost @pgSelectOne
      """
      Query and paginate multiple usersPosts
      """
      usersPosts(
        """
        Filter for UsersPost
        """
        filter: UsersPostFilterInput,
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
        orderBy: [UsersPostOrderByInput!],
      ): UsersPostConnection! @pgSelectMany
      """
      Lookup multiple usersPosts for subgraph joins
      """
      usersPostLookup(
        """
        Filter usersPosts with an array of keys
        """
        lookup: UsersPostManyLookupInput @inaccessible,
      ): [UsersPost]! @pgLookup @lookup @inaccessible
    }
    "#);
}
