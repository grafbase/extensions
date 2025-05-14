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
