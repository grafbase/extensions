mod configuration;
mod views;

use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn table_with_generated_always_identity_primary_key() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
async fn table_with_generated_by_default_identity_primary_key() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY GENERATED BY DEFAULT AS IDENTITY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
      """
      Set field value for id
      """
      id: Int
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
      """
      Update field value for id
      """
      id: IntUpdateInput
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
async fn table_with_serial_primary_key() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
      """
      Set field value for id
      """
      id: Int
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
      """
      Update field value for id
      """
      id: IntUpdateInput
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
      @pgTable(name: "User")
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
async fn table_with_enum_field() {
    let api = PgTestApi::new("", |api| async move {
        let r#type = indoc! {r"
            CREATE TYPE street_light AS ENUM ('red', 'yellow', 'green');
        "};

        api.execute_sql(r#type).await;

        let table = indoc! {r#"
            CREATE TABLE "A" (
              id INT PRIMARY KEY,
              val street_light NOT NULL
            );
        "#};

        api.execute_sql(table).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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

    enum StreetLight @pgEnum(name: "street_light") {
      RED @pgEnumVariant(name: "red")
      YELLOW @pgEnumVariant(name: "yellow")
      GREEN @pgEnumVariant(name: "green")
    }

    """
    Input for creating a new A
    """
    input ACreateInput {
      """
      Set field value for id
      """
      id: Int!
      """
      Set field value for val
      """
      val: StreetLight!
    }

    """
    Filter input type for A objects.
    """
    input AFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the given val
      """
      val: StreetLightFilterInput
      """
      All of the filters must match
      """
      ALL: [AFilterInput]
      """
      None of the filters must match
      """
      NONE: [AFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [AFilterInput]
    }

    """
    Input type to select a unique A
    """
    input ALookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for A objects for subgraph joins.
    """
    input AManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for A results.
    """
    input AOrderByInput @oneOf {
      """
      Order as by id
      """
      id: OrderDirection
      """
      Order as by val
      """
      val: OrderDirection
    }

    """
    Input for updating an existing A
    """
    input AUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for val
      """
      val: StreetLightUpdateInput
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
    Search filter input for StreetLight type.
    """
    input StreetLightFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: StreetLight
      """
      The value is not the one given
      """
      ne: StreetLight
      """
      The value is greater than the one given
      """
      gt: StreetLight
      """
      The value is less than the one given
      """
      lt: StreetLight
      """
      The value is greater than, or equal to the one given
      """
      gte: StreetLight
      """
      The value is less than, or equal to the one given
      """
      lte: StreetLight
      """
      The value is in the given array of values
      """
      in: [StreetLight!]
      """
      The value is not in the given array of values
      """
      nin: [StreetLight!]
      """
      A negation of the given filter
      """
      not: StreetLightFilterInput
    }

    """
    Update input for StreetLight type.
    """
    input StreetLightUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: StreetLight
    }

    type A
      @key(fields: "id")
      @pgTable(name: "A")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      val: StreetLight! @pgColumn(name: "val", type: ENUM)
    }

    """
    The connection type for A
    """
    type AConnection
      @pgConnection(type: "A")
    {
      """
      A list of edges
      """
      edges: [AEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many as
    """
    type ACreateManyPayload
      @pgMutation(type: "A")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [AReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one A
    """
    type ACreatePayload
      @pgMutation(type: "A")
    {
      """
      Returned item(s) from the mutation
      """
      returning: AReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many as
    """
    type ADeleteManyPayload
      @pgMutation(type: "A")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [AReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one A
    """
    type ADeletePayload
      @pgMutation(type: "A")
    {
      """
      Returned item(s) from the mutation
      """
      returning: AReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type AEdge {
      """
      The item at the end of the edge
      """
      node: A! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created A object
    """
    type AReturning
      @pgReturning(type: "A")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
      """
      The value of the val field
      """
      val: StreetLight!
    }

    """
    Return type when updating many as
    """
    type AUpdateManyPayload
      @pgMutation(type: "A")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [AReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one A
    """
    type AUpdatePayload
      @pgMutation(type: "A")
    {
      """
      Returned item(s) from the mutation
      """
      returning: AReturning @shareable
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
      Query a unique A
      """
      a(
        """
        Input for unique A lookup
        """
        lookup: ALookupInput!,
      ): A @pgSelectOne
      """
      Query and paginate multiple as
      """
      as(
        """
        Filter for A
        """
        filter: AFilterInput,
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
        orderBy: [AOrderByInput!],
      ): AConnection! @pgSelectMany
      """
      Lookup multiple as for subgraph joins
      """
      aLookup(
        """
        Filter as with an array of keys
        """
        lookup: AManyLookupInput @inaccessible,
      ): [A]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single A
      """
      aCreate(
        """
        Input for creating a single A
        """
        input: ACreateInput!,
      ): ACreatePayload! @pgInsertOne
      """
      Create multiple as
      """
      aCreateMany(
        """
        Input for creating multiple A instances
        """
        input: [ACreateInput!]!,
      ): ACreateManyPayload! @pgInsertMany
      """
      Update a unique A
      """
      aUpdate(
        """
        Lookup input for unique A update
        """
        lookup: ALookupInput!,
        """
        Input for updating a A
        """
        input: AUpdateInput!,
      ): AUpdatePayload! @pgUpdateOne
      """
      Update multiple as
      """
      aUpdateMany(
        """
        Filter for updating multiple A instances
        """
        filter: AFilterInput,
        """
        Input for updating multiple A instances
        """
        input: AUpdateInput!,
      ): AUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique A
      """
      aDelete(
        """
        Lookup input for unique A deletion
        """
        lookup: ALookupInput!,
      ): ADeletePayload! @pgDeleteOne
      """
      Delete multiple as
      """
      aDeleteMany(
        """
        Filter for A deletion
        """
        filter: AFilterInput,
      ): ADeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn table_with_int_primary_key() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT PRIMARY KEY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
      """
      Set field value for id
      """
      id: Int!
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
      """
      Update field value for id
      """
      id: IntUpdateInput
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
      @pgTable(name: "User")
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
async fn table_with_int_unique() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id INT UNIQUE NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
      """
      Set field value for id
      """
      id: Int!
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
      """
      Update field value for id
      """
      id: IntUpdateInput
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
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: UNIQUE)
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
async fn table_with_serial_primary_key_string_unique() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                id SERIAL PRIMARY KEY,
                email VARCHAR(255) NOT NULL UNIQUE
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    Input for creating a new User
    """
    input UserCreateInput {
      """
      Set field value for id
      """
      id: Int
      """
      Set field value for email
      """
      email: String!
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
      Filter by the given email
      """
      email: StringFilterInput
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
      Select by the 'email' field
      """
      email: String
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
      Select by the 'email' field
      """
      email: [String!] @inaccessible
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
      Order users by email
      """
      email: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for email
      """
      email: StringUpdateInput
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
      @key(fields: "email")
      @key(fields: "id")
      @pgTable(name: "User")
      @pgKey(fields: ["email"], type: UNIQUE)
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      email: String! @pgColumn(name: "email", type: VARCHAR)
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
      """
      The value of the email field
      """
      email: String! @shareable
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
async fn table_with_composite_primary_key() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                CONSTRAINT "User_pkey" PRIMARY KEY (name, email)
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    Input for creating a new User
    """
    input UserCreateInput {
      """
      Set field value for name
      """
      name: String!
      """
      Set field value for email
      """
      email: String!
    }

    """
    Filter input type for User objects.
    """
    input UserFilterInput @oneOf {
      """
      Filter by the given name
      """
      name: StringFilterInput
      """
      Filter by the given email
      """
      email: StringFilterInput
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
      Select User by composite columns 'name, email'
      """
      nameEmail: UserNameEmailInput
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select User by composite columns 'name, email'
      """
      nameEmail: [UserNameEmailInput!] @inaccessible
    }

    """
    Input type to select a unique User with multiple fields
    """
    input UserNameEmailInput {
      """
      Select by the 'name' field
      """
      name: String!
      """
      Select by the 'email' field
      """
      email: String!
    }

    """
    Specifies the ordering for User results.
    """
    input UserOrderByInput @oneOf {
      """
      Order users by name
      """
      name: OrderDirection
      """
      Order users by email
      """
      email: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      """
      Update field value for name
      """
      name: StringUpdateInput
      """
      Update field value for email
      """
      email: StringUpdateInput
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
      @key(fields: "name email")
      @pgTable(name: "User")
      @pgKey(fields: ["name", "email"], type: PRIMARY)
    {
      name: String! @pgColumn(name: "name", type: VARCHAR)
      email: String! @pgColumn(name: "email", type: VARCHAR)
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
      The value of the name field
      """
      name: String! @shareable
      """
      The value of the email field
      """
      email: String! @shareable
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
async fn two_schemas_same_table_name() {
    let api = PgTestApi::new("", |api| async move {
        api.execute_sql(r"CREATE SCHEMA private").await;

        let schema = indoc! {r#"
            CREATE TABLE private."User" (
                id SERIAL PRIMARY KEY
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE public."User" (
                id SERIAL PRIMARY KEY
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    Input for creating a new PrivateUser
    """
    input PrivateUserCreateInput {
      """
      Set field value for id
      """
      id: Int
    }

    """
    Filter input type for PrivateUser objects.
    """
    input PrivateUserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [PrivateUserFilterInput]
      """
      None of the filters must match
      """
      NONE: [PrivateUserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [PrivateUserFilterInput]
    }

    """
    Input type to select a unique PrivateUser
    """
    input PrivateUserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for PrivateUser objects for subgraph joins.
    """
    input PrivateUserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for PrivateUser results.
    """
    input PrivateUserOrderByInput @oneOf {
      """
      Order privateUsers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing PrivateUser
    """
    input PrivateUserUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
    }

    """
    Input for creating a new PublicUser
    """
    input PublicUserCreateInput {
      """
      Set field value for id
      """
      id: Int
    }

    """
    Filter input type for PublicUser objects.
    """
    input PublicUserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [PublicUserFilterInput]
      """
      None of the filters must match
      """
      NONE: [PublicUserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [PublicUserFilterInput]
    }

    """
    Input type to select a unique PublicUser
    """
    input PublicUserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for PublicUser objects for subgraph joins.
    """
    input PublicUserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for PublicUser results.
    """
    input PublicUserOrderByInput @oneOf {
      """
      Order publicUsers by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing PublicUser
    """
    input PublicUserUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
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

    type PrivateUser
      @key(fields: "id")
      @pgTable(name: "User", schema: "private")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for PrivateUser
    """
    type PrivateUserConnection
      @pgConnection(type: "PrivateUser")
    {
      """
      A list of edges
      """
      edges: [PrivateUserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many privateUsers
    """
    type PrivateUserCreateManyPayload
      @pgMutation(type: "PrivateUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PrivateUserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one PrivateUser
    """
    type PrivateUserCreatePayload
      @pgMutation(type: "PrivateUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PrivateUserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many privateUsers
    """
    type PrivateUserDeleteManyPayload
      @pgMutation(type: "PrivateUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PrivateUserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one PrivateUser
    """
    type PrivateUserDeletePayload
      @pgMutation(type: "PrivateUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PrivateUserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type PrivateUserEdge {
      """
      The item at the end of the edge
      """
      node: PrivateUser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created PrivateUser object
    """
    type PrivateUserReturning
      @pgReturning(type: "PrivateUser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when updating many privateUsers
    """
    type PrivateUserUpdateManyPayload
      @pgMutation(type: "PrivateUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PrivateUserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one PrivateUser
    """
    type PrivateUserUpdatePayload
      @pgMutation(type: "PrivateUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PrivateUserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type PublicUser
      @key(fields: "id")
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for PublicUser
    """
    type PublicUserConnection
      @pgConnection(type: "PublicUser")
    {
      """
      A list of edges
      """
      edges: [PublicUserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many publicUsers
    """
    type PublicUserCreateManyPayload
      @pgMutation(type: "PublicUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PublicUserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one PublicUser
    """
    type PublicUserCreatePayload
      @pgMutation(type: "PublicUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PublicUserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many publicUsers
    """
    type PublicUserDeleteManyPayload
      @pgMutation(type: "PublicUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PublicUserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one PublicUser
    """
    type PublicUserDeletePayload
      @pgMutation(type: "PublicUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PublicUserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type PublicUserEdge {
      """
      The item at the end of the edge
      """
      node: PublicUser! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created PublicUser object
    """
    type PublicUserReturning
      @pgReturning(type: "PublicUser")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when updating many publicUsers
    """
    type PublicUserUpdateManyPayload
      @pgMutation(type: "PublicUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PublicUserReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one PublicUser
    """
    type PublicUserUpdatePayload
      @pgMutation(type: "PublicUser")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PublicUserReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type Query {
      """
      Query a unique PrivateUser
      """
      privateUser(
        """
        Input for unique PrivateUser lookup
        """
        lookup: PrivateUserLookupInput!,
      ): PrivateUser @pgSelectOne
      """
      Query and paginate multiple privateUsers
      """
      privateUsers(
        """
        Filter for PrivateUser
        """
        filter: PrivateUserFilterInput,
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
        orderBy: [PrivateUserOrderByInput!],
      ): PrivateUserConnection! @pgSelectMany
      """
      Lookup multiple privateUsers for subgraph joins
      """
      privateUserLookup(
        """
        Filter privateUsers with an array of keys
        """
        lookup: PrivateUserManyLookupInput @inaccessible,
      ): [PrivateUser]! @pgLookup @lookup @inaccessible
      """
      Query a unique PublicUser
      """
      publicUser(
        """
        Input for unique PublicUser lookup
        """
        lookup: PublicUserLookupInput!,
      ): PublicUser @pgSelectOne
      """
      Query and paginate multiple publicUsers
      """
      publicUsers(
        """
        Filter for PublicUser
        """
        filter: PublicUserFilterInput,
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
        orderBy: [PublicUserOrderByInput!],
      ): PublicUserConnection! @pgSelectMany
      """
      Lookup multiple publicUsers for subgraph joins
      """
      publicUserLookup(
        """
        Filter publicUsers with an array of keys
        """
        lookup: PublicUserManyLookupInput @inaccessible,
      ): [PublicUser]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single PrivateUser
      """
      privateUserCreate(
        """
        Input for creating a single PrivateUser
        """
        input: PrivateUserCreateInput!,
      ): PrivateUserCreatePayload! @pgInsertOne
      """
      Create multiple privateUsers
      """
      privateUserCreateMany(
        """
        Input for creating multiple PrivateUser instances
        """
        input: [PrivateUserCreateInput!]!,
      ): PrivateUserCreateManyPayload! @pgInsertMany
      """
      Update a unique PrivateUser
      """
      privateUserUpdate(
        """
        Lookup input for unique PrivateUser update
        """
        lookup: PrivateUserLookupInput!,
        """
        Input for updating a PrivateUser
        """
        input: PrivateUserUpdateInput!,
      ): PrivateUserUpdatePayload! @pgUpdateOne
      """
      Update multiple privateUsers
      """
      privateUserUpdateMany(
        """
        Filter for updating multiple PrivateUser instances
        """
        filter: PrivateUserFilterInput,
        """
        Input for updating multiple PrivateUser instances
        """
        input: PrivateUserUpdateInput!,
      ): PrivateUserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique PrivateUser
      """
      privateUserDelete(
        """
        Lookup input for unique PrivateUser deletion
        """
        lookup: PrivateUserLookupInput!,
      ): PrivateUserDeletePayload! @pgDeleteOne
      """
      Delete multiple privateUsers
      """
      privateUserDeleteMany(
        """
        Filter for PrivateUser deletion
        """
        filter: PrivateUserFilterInput,
      ): PrivateUserDeleteManyPayload! @pgDeleteMany
      """
      Create a single PublicUser
      """
      publicUserCreate(
        """
        Input for creating a single PublicUser
        """
        input: PublicUserCreateInput!,
      ): PublicUserCreatePayload! @pgInsertOne
      """
      Create multiple publicUsers
      """
      publicUserCreateMany(
        """
        Input for creating multiple PublicUser instances
        """
        input: [PublicUserCreateInput!]!,
      ): PublicUserCreateManyPayload! @pgInsertMany
      """
      Update a unique PublicUser
      """
      publicUserUpdate(
        """
        Lookup input for unique PublicUser update
        """
        lookup: PublicUserLookupInput!,
        """
        Input for updating a PublicUser
        """
        input: PublicUserUpdateInput!,
      ): PublicUserUpdatePayload! @pgUpdateOne
      """
      Update multiple publicUsers
      """
      publicUserUpdateMany(
        """
        Filter for updating multiple PublicUser instances
        """
        filter: PublicUserFilterInput,
        """
        Input for updating multiple PublicUser instances
        """
        input: PublicUserUpdateInput!,
      ): PublicUserUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique PublicUser
      """
      publicUserDelete(
        """
        Lookup input for unique PublicUser deletion
        """
        lookup: PublicUserLookupInput!,
      ): PublicUserDeletePayload! @pgDeleteOne
      """
      Delete multiple publicUsers
      """
      publicUserDeleteMany(
        """
        Filter for PublicUser deletion
        """
        filter: PublicUserFilterInput,
      ): PublicUserDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn table_with_an_array_column() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
           CREATE TABLE "User" (
               id SERIAL PRIMARY KEY,
               name INT[] NOT NULL
           );
       "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
      """
      Set field value for id
      """
      id: Int
      """
      Set field value for name
      """
      name: [Int]!
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
      Filter by the given name
      """
      name: IntArrayFilterInput
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
      Order users by name
      """
      name: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for name
      """
      name: IntArrayUpdateInput
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
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      name: [Int]! @pgColumn(name: "name", type: INT)
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
      """
      The value of the name field
      """
      name: [Int]!
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
async fn table_with_jsonb_column() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
           CREATE TABLE "User" (
               id SERIAL PRIMARY KEY,
               name JSONB NOT NULL
           );
       "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    Arbitrary JSON object
    """
    scalar JSON

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
    Input for creating a new User
    """
    input UserCreateInput {
      """
      Set field value for id
      """
      id: Int
      """
      Set field value for name
      """
      name: JSON!
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
      Filter by the given name
      """
      name: JSONFilterInput
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
      Order users by name
      """
      name: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for name
      """
      name: JSONUpdateInput
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
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      name: JSON! @pgColumn(name: "name", type: JSONB)
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
      """
      The value of the name field
      """
      name: JSON!
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
async fn table_with_json_column() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
           CREATE TABLE "User" (
               id SERIAL PRIMARY KEY,
               name JSON NOT NULL
           );
       "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    Arbitrary JSON object
    """
    scalar JSON

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
    Input for creating a new User
    """
    input UserCreateInput {
      """
      Set field value for id
      """
      id: Int
      """
      Set field value for name
      """
      name: JSON!
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
      Filter by the given name
      """
      name: JSONFilterInput
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
      Order users by name
      """
      name: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for name
      """
      name: JSONUpdateInput
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
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      name: JSON! @pgColumn(name: "name", type: JSON)
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
      """
      The value of the name field
      """
      name: JSON!
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
async fn two_tables_with_single_column_foreign_key() {
    let api = PgTestApi::new("", |api| async move {
        let create_user = indoc! {r#"
           CREATE TABLE "User" (
               id SERIAL PRIMARY KEY,
               name VARCHAR(255) NOT NULL
           );
       "#};

        api.execute_sql(create_user).await;

        let create_blog = indoc! {r#"
            CREATE TABLE "Blog" (
                id SERIAL PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                content TEXT,
                user_id INT NOT NULL,
                CONSTRAINT "Blog_User" FOREIGN KEY (user_id) REFERENCES "User"(id)
            )
        "#};

        api.execute_sql(create_blog).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    Filter input type for Blog collections
    """
    input BlogCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: BlogFilterInput
    }

    """
    Input for creating a new Blog
    """
    input BlogCreateInput {
      """
      Set field value for id
      """
      id: Int
      """
      Set field value for title
      """
      title: String!
      """
      Set field value for content
      """
      content: String
      """
      Set field value for userId
      """
      userId: Int!
    }

    """
    Filter input type for Blog objects.
    """
    input BlogFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the given title
      """
      title: StringFilterInput
      """
      Filter by the given content
      """
      content: StringFilterInput
      """
      Filter by the given userId
      """
      userId: IntFilterInput
      """
      Filter by the related User object
      """
      user: UserFilterInput
      """
      All of the filters must match
      """
      ALL: [BlogFilterInput]
      """
      None of the filters must match
      """
      NONE: [BlogFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [BlogFilterInput]
    }

    """
    Input type to select a unique Blog
    """
    input BlogLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for Blog objects for subgraph joins.
    """
    input BlogManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for Blog results.
    """
    input BlogOrderByInput @oneOf {
      """
      Order blogs by id
      """
      id: OrderDirection
      """
      Order blogs by title
      """
      title: OrderDirection
      """
      Order blogs by content
      """
      content: OrderDirection
      """
      Order blogs by userId
      """
      userId: OrderDirection
      """
      Order Blog results by User fields
      """
      user: UserOrderByInput
    }

    """
    Input for updating an existing Blog
    """
    input BlogUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for title
      """
      title: StringUpdateInput
      """
      Update field value for content
      """
      content: StringUpdateInput
      """
      Update field value for userId
      """
      userId: IntUpdateInput
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
    Input for creating a new User
    """
    input UserCreateInput {
      """
      Set field value for id
      """
      id: Int
      """
      Set field value for name
      """
      name: String!
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
      Filter by the given name
      """
      name: StringFilterInput
      """
      Filter by the related Blog objects
      """
      blogs: BlogCollectionFilterInput
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
      Order users by name
      """
      name: OrderDirection
    }

    """
    Input for updating an existing User
    """
    input UserUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for name
      """
      name: StringUpdateInput
    }

    type Blog
      @key(fields: "id")
      @pgTable(name: "Blog")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      title: String! @pgColumn(name: "title", type: VARCHAR)
      content: String @pgColumn(name: "content", type: TEXT)
      userId: Int! @pgColumn(name: "user_id", type: INT)
      user: User! @pgRelation(name: "Blog_User", fields: ["userId"], references: ["id"])
    }

    """
    The connection type for Blog
    """
    type BlogConnection
      @pgConnection(type: "Blog")
    {
      """
      A list of edges
      """
      edges: [BlogEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many blogs
    """
    type BlogCreateManyPayload
      @pgMutation(type: "Blog")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [BlogReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one Blog
    """
    type BlogCreatePayload
      @pgMutation(type: "Blog")
    {
      """
      Returned item(s) from the mutation
      """
      returning: BlogReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many blogs
    """
    type BlogDeleteManyPayload
      @pgMutation(type: "Blog")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [BlogReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Blog
    """
    type BlogDeletePayload
      @pgMutation(type: "Blog")
    {
      """
      Returned item(s) from the mutation
      """
      returning: BlogReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type BlogEdge {
      """
      The item at the end of the edge
      """
      node: Blog! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created Blog object
    """
    type BlogReturning
      @pgReturning(type: "Blog")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
      """
      The value of the title field
      """
      title: String!
      """
      The value of the content field
      """
      content: String
      """
      The value of the userId field
      """
      userId: Int!
    }

    """
    Return type when updating many blogs
    """
    type BlogUpdateManyPayload
      @pgMutation(type: "Blog")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [BlogReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Blog
    """
    type BlogUpdatePayload
      @pgMutation(type: "Blog")
    {
      """
      Returned item(s) from the mutation
      """
      returning: BlogReturning @shareable
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
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      name: String! @pgColumn(name: "name", type: VARCHAR)
      blogs(
        """
        Filter the related Blog instances
        """
        filter: BlogFilterInput,
        """
        Select the first Blog instances
        """
        first: Int,
        """
        Select the last Blog instances
        """
        last: Int,
        """
        Select the Blog instances before the given cursor
        """
        before: String,
        """
        Select the Blog instances after the given cursor
        """
        after: String,
        """
        Order the Blog instances by the given fields
        """
        orderBy: [BlogOrderByInput!],
      ): BlogConnection! @pgRelation(name: "Blog_User")
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
      """
      The value of the name field
      """
      name: String!
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
      Query a unique Blog
      """
      blog(
        """
        Input for unique Blog lookup
        """
        lookup: BlogLookupInput!,
      ): Blog @pgSelectOne
      """
      Query and paginate multiple blogs
      """
      blogs(
        """
        Filter for Blog
        """
        filter: BlogFilterInput,
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
        orderBy: [BlogOrderByInput!],
      ): BlogConnection! @pgSelectMany
      """
      Lookup multiple blogs for subgraph joins
      """
      blogLookup(
        """
        Filter blogs with an array of keys
        """
        lookup: BlogManyLookupInput @inaccessible,
      ): [Blog]! @pgLookup @lookup @inaccessible
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
      Create a single Blog
      """
      blogCreate(
        """
        Input for creating a single Blog
        """
        input: BlogCreateInput!,
      ): BlogCreatePayload! @pgInsertOne
      """
      Create multiple blogs
      """
      blogCreateMany(
        """
        Input for creating multiple Blog instances
        """
        input: [BlogCreateInput!]!,
      ): BlogCreateManyPayload! @pgInsertMany
      """
      Update a unique Blog
      """
      blogUpdate(
        """
        Lookup input for unique Blog update
        """
        lookup: BlogLookupInput!,
        """
        Input for updating a Blog
        """
        input: BlogUpdateInput!,
      ): BlogUpdatePayload! @pgUpdateOne
      """
      Update multiple blogs
      """
      blogUpdateMany(
        """
        Filter for updating multiple Blog instances
        """
        filter: BlogFilterInput,
        """
        Input for updating multiple Blog instances
        """
        input: BlogUpdateInput!,
      ): BlogUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Blog
      """
      blogDelete(
        """
        Lookup input for unique Blog deletion
        """
        lookup: BlogLookupInput!,
      ): BlogDeletePayload! @pgDeleteOne
      """
      Delete multiple blogs
      """
      blogDeleteMany(
        """
        Filter for Blog deletion
        """
        filter: BlogFilterInput,
      ): BlogDeleteManyPayload! @pgDeleteMany
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
async fn foreign_key_to_a_table_without_a_key_should_not_create_a_relation() {
    let api = PgTestApi::new("pg", |api| async move {
        api.execute_sql(r#"CREATE TABLE visible_table (id TEXT PRIMARY KEY)"#)
            .await;

        api.execute_sql(r#"CREATE TABLE hidden_table (visible_table TEXT NOT NULL REFERENCES visible_table(id))"#)
            .await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    Input for creating a new VisibleTable
    """
    input VisibleTableCreateInput {
      """
      Set field value for id
      """
      id: String!
    }

    """
    Filter input type for VisibleTable objects.
    """
    input VisibleTableFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: StringFilterInput
      """
      All of the filters must match
      """
      ALL: [VisibleTableFilterInput]
      """
      None of the filters must match
      """
      NONE: [VisibleTableFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [VisibleTableFilterInput]
    }

    """
    Input type to select a unique VisibleTable
    """
    input VisibleTableLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: String
    }

    """
    Lookup input type for VisibleTable objects for subgraph joins.
    """
    input VisibleTableManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [String!] @inaccessible
    }

    """
    Specifies the ordering for VisibleTable results.
    """
    input VisibleTableOrderByInput @oneOf {
      """
      Order visibleTables by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing VisibleTable
    """
    input VisibleTableUpdateInput {
      """
      Update field value for id
      """
      id: StringUpdateInput
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

    type VisibleTable
      @key(fields: "id")
      @pgTable(name: "visible_table")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: String! @pgColumn(name: "id", type: TEXT)
    }

    """
    The connection type for VisibleTable
    """
    type VisibleTableConnection
      @pgConnection(type: "VisibleTable")
    {
      """
      A list of edges
      """
      edges: [VisibleTableEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many visibleTables
    """
    type VisibleTableCreateManyPayload
      @pgMutation(type: "VisibleTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [VisibleTableReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one VisibleTable
    """
    type VisibleTableCreatePayload
      @pgMutation(type: "VisibleTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: VisibleTableReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many visibleTables
    """
    type VisibleTableDeleteManyPayload
      @pgMutation(type: "VisibleTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [VisibleTableReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one VisibleTable
    """
    type VisibleTableDeletePayload
      @pgMutation(type: "VisibleTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: VisibleTableReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type VisibleTableEdge {
      """
      The item at the end of the edge
      """
      node: VisibleTable! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created VisibleTable object
    """
    type VisibleTableReturning
      @pgReturning(type: "VisibleTable")
    {
      """
      The value of the id field
      """
      id: String! @shareable
    }

    """
    Return type when updating many visibleTables
    """
    type VisibleTableUpdateManyPayload
      @pgMutation(type: "VisibleTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [VisibleTableReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one VisibleTable
    """
    type VisibleTableUpdatePayload
      @pgMutation(type: "VisibleTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: VisibleTableReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type Query {
      """
      Query a unique VisibleTable
      """
      visibleTable(
        """
        Input for unique VisibleTable lookup
        """
        lookup: VisibleTableLookupInput!,
      ): VisibleTable @pgSelectOne
      """
      Query and paginate multiple visibleTables
      """
      visibleTables(
        """
        Filter for VisibleTable
        """
        filter: VisibleTableFilterInput,
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
        orderBy: [VisibleTableOrderByInput!],
      ): VisibleTableConnection! @pgSelectMany
      """
      Lookup multiple visibleTables for subgraph joins
      """
      visibleTableLookup(
        """
        Filter visibleTables with an array of keys
        """
        lookup: VisibleTableManyLookupInput @inaccessible,
      ): [VisibleTable]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single VisibleTable
      """
      visibleTableCreate(
        """
        Input for creating a single VisibleTable
        """
        input: VisibleTableCreateInput!,
      ): VisibleTableCreatePayload! @pgInsertOne
      """
      Create multiple visibleTables
      """
      visibleTableCreateMany(
        """
        Input for creating multiple VisibleTable instances
        """
        input: [VisibleTableCreateInput!]!,
      ): VisibleTableCreateManyPayload! @pgInsertMany
      """
      Update a unique VisibleTable
      """
      visibleTableUpdate(
        """
        Lookup input for unique VisibleTable update
        """
        lookup: VisibleTableLookupInput!,
        """
        Input for updating a VisibleTable
        """
        input: VisibleTableUpdateInput!,
      ): VisibleTableUpdatePayload! @pgUpdateOne
      """
      Update multiple visibleTables
      """
      visibleTableUpdateMany(
        """
        Filter for updating multiple VisibleTable instances
        """
        filter: VisibleTableFilterInput,
        """
        Input for updating multiple VisibleTable instances
        """
        input: VisibleTableUpdateInput!,
      ): VisibleTableUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique VisibleTable
      """
      visibleTableDelete(
        """
        Lookup input for unique VisibleTable deletion
        """
        lookup: VisibleTableLookupInput!,
      ): VisibleTableDeletePayload! @pgDeleteOne
      """
      Delete multiple visibleTables
      """
      visibleTableDeleteMany(
        """
        Filter for VisibleTable deletion
        """
        filter: VisibleTableFilterInput,
      ): VisibleTableDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn issue_november_2023() {
    let api = PgTestApi::new("pg", |api| async move {
        let create = indoc! {r"
            CREATE TYPE access_mode AS ENUM ('PUBLIC', 'PUBLIC_READ', 'PRIVATE');
        "};

        api.execute_sql(create).await;

        let create = indoc! {r"
            CREATE TYPE project_status AS ENUM ('CREATED', 'READY', 'FAILED');
        "};

        api.execute_sql(create).await;

        let create = indoc! {r"
            CREATE TABLE networks (
                id SERIAL PRIMARY KEY
            );
        "};

        api.execute_sql(create).await;

        let create = indoc! {r"
            CREATE TABLE projects (
                id SERIAL PRIMARY KEY,
                access_mode access_mode NOT NULL,
                status project_status DEFAULT 'CREATED' NOT NULL,
                network_id INT REFERENCES networks(id)
            );
        "};

        api.execute_sql(create).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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

    enum AccessMode @pgEnum(name: "access_mode") {
      PUBLIC @pgEnumVariant(name: "PUBLIC")
      PUBLIC_READ @pgEnumVariant(name: "PUBLIC_READ")
      PRIVATE @pgEnumVariant(name: "PRIVATE")
    }

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

    enum ProjectStatus @pgEnum(name: "project_status") {
      CREATED @pgEnumVariant(name: "CREATED")
      READY @pgEnumVariant(name: "READY")
      FAILED @pgEnumVariant(name: "FAILED")
    }

    """
    Search filter input for AccessMode type.
    """
    input AccessModeFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: AccessMode
      """
      The value is not the one given
      """
      ne: AccessMode
      """
      The value is greater than the one given
      """
      gt: AccessMode
      """
      The value is less than the one given
      """
      lt: AccessMode
      """
      The value is greater than, or equal to the one given
      """
      gte: AccessMode
      """
      The value is less than, or equal to the one given
      """
      lte: AccessMode
      """
      The value is in the given array of values
      """
      in: [AccessMode!]
      """
      The value is not in the given array of values
      """
      nin: [AccessMode!]
      """
      A negation of the given filter
      """
      not: AccessModeFilterInput
    }

    """
    Update input for AccessMode type.
    """
    input AccessModeUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: AccessMode
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
    Input for creating a new Network
    """
    input NetworkCreateInput {
      """
      Set field value for id
      """
      id: Int
    }

    """
    Filter input type for Network objects.
    """
    input NetworkFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the related Project objects
      """
      projects: ProjectCollectionFilterInput
      """
      All of the filters must match
      """
      ALL: [NetworkFilterInput]
      """
      None of the filters must match
      """
      NONE: [NetworkFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [NetworkFilterInput]
    }

    """
    Input type to select a unique Network
    """
    input NetworkLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for Network objects for subgraph joins.
    """
    input NetworkManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for Network results.
    """
    input NetworkOrderByInput @oneOf {
      """
      Order networks by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing Network
    """
    input NetworkUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
    }

    """
    Filter input type for Project collections
    """
    input ProjectCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: ProjectFilterInput
    }

    """
    Input for creating a new Project
    """
    input ProjectCreateInput {
      """
      Set field value for id
      """
      id: Int
      """
      Set field value for accessMode
      """
      accessMode: AccessMode!
      """
      Set field value for status
      """
      status: ProjectStatus
      """
      Set field value for networkId
      """
      networkId: Int
    }

    """
    Filter input type for Project objects.
    """
    input ProjectFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the given accessMode
      """
      accessMode: AccessModeFilterInput
      """
      Filter by the given status
      """
      status: ProjectStatusFilterInput
      """
      Filter by the given networkId
      """
      networkId: IntFilterInput
      """
      Filter by the related Network object
      """
      network: NetworkFilterInput
      """
      All of the filters must match
      """
      ALL: [ProjectFilterInput]
      """
      None of the filters must match
      """
      NONE: [ProjectFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [ProjectFilterInput]
    }

    """
    Input type to select a unique Project
    """
    input ProjectLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for Project objects for subgraph joins.
    """
    input ProjectManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for Project results.
    """
    input ProjectOrderByInput @oneOf {
      """
      Order projects by id
      """
      id: OrderDirection
      """
      Order projects by accessMode
      """
      accessMode: OrderDirection
      """
      Order projects by status
      """
      status: OrderDirection
      """
      Order projects by networkId
      """
      networkId: OrderDirection
      """
      Order Project results by Network fields
      """
      network: NetworkOrderByInput
    }

    """
    Search filter input for ProjectStatus type.
    """
    input ProjectStatusFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: ProjectStatus
      """
      The value is not the one given
      """
      ne: ProjectStatus
      """
      The value is greater than the one given
      """
      gt: ProjectStatus
      """
      The value is less than the one given
      """
      lt: ProjectStatus
      """
      The value is greater than, or equal to the one given
      """
      gte: ProjectStatus
      """
      The value is less than, or equal to the one given
      """
      lte: ProjectStatus
      """
      The value is in the given array of values
      """
      in: [ProjectStatus!]
      """
      The value is not in the given array of values
      """
      nin: [ProjectStatus!]
      """
      A negation of the given filter
      """
      not: ProjectStatusFilterInput
    }

    """
    Update input for ProjectStatus type.
    """
    input ProjectStatusUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: ProjectStatus
    }

    """
    Input for updating an existing Project
    """
    input ProjectUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for accessMode
      """
      accessMode: AccessModeUpdateInput
      """
      Update field value for status
      """
      status: ProjectStatusUpdateInput
      """
      Update field value for networkId
      """
      networkId: IntUpdateInput
    }

    type Network
      @key(fields: "id")
      @pgTable(name: "networks")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      projects(
        """
        Filter the related Project instances
        """
        filter: ProjectFilterInput,
        """
        Select the first Project instances
        """
        first: Int,
        """
        Select the last Project instances
        """
        last: Int,
        """
        Select the Project instances before the given cursor
        """
        before: String,
        """
        Select the Project instances after the given cursor
        """
        after: String,
        """
        Order the Project instances by the given fields
        """
        orderBy: [ProjectOrderByInput!],
      ): ProjectConnection! @pgRelation(name: "projects_network_id_fkey")
    }

    """
    The connection type for Network
    """
    type NetworkConnection
      @pgConnection(type: "Network")
    {
      """
      A list of edges
      """
      edges: [NetworkEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many networks
    """
    type NetworkCreateManyPayload
      @pgMutation(type: "Network")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [NetworkReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one Network
    """
    type NetworkCreatePayload
      @pgMutation(type: "Network")
    {
      """
      Returned item(s) from the mutation
      """
      returning: NetworkReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many networks
    """
    type NetworkDeleteManyPayload
      @pgMutation(type: "Network")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [NetworkReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Network
    """
    type NetworkDeletePayload
      @pgMutation(type: "Network")
    {
      """
      Returned item(s) from the mutation
      """
      returning: NetworkReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type NetworkEdge {
      """
      The item at the end of the edge
      """
      node: Network! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created Network object
    """
    type NetworkReturning
      @pgReturning(type: "Network")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when updating many networks
    """
    type NetworkUpdateManyPayload
      @pgMutation(type: "Network")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [NetworkReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Network
    """
    type NetworkUpdatePayload
      @pgMutation(type: "Network")
    {
      """
      Returned item(s) from the mutation
      """
      returning: NetworkReturning @shareable
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

    type Project
      @key(fields: "id")
      @pgTable(name: "projects")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      accessMode: AccessMode! @pgColumn(name: "access_mode", type: ENUM)
      status: ProjectStatus! @pgColumn(name: "status", type: ENUM)
      networkId: Int @pgColumn(name: "network_id", type: INT)
      network: Network @pgRelation(name: "projects_network_id_fkey", fields: ["networkId"], references: ["id"])
    }

    """
    The connection type for Project
    """
    type ProjectConnection
      @pgConnection(type: "Project")
    {
      """
      A list of edges
      """
      edges: [ProjectEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many projects
    """
    type ProjectCreateManyPayload
      @pgMutation(type: "Project")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [ProjectReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one Project
    """
    type ProjectCreatePayload
      @pgMutation(type: "Project")
    {
      """
      Returned item(s) from the mutation
      """
      returning: ProjectReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many projects
    """
    type ProjectDeleteManyPayload
      @pgMutation(type: "Project")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [ProjectReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one Project
    """
    type ProjectDeletePayload
      @pgMutation(type: "Project")
    {
      """
      Returned item(s) from the mutation
      """
      returning: ProjectReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type ProjectEdge {
      """
      The item at the end of the edge
      """
      node: Project! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created Project object
    """
    type ProjectReturning
      @pgReturning(type: "Project")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
      """
      The value of the accessMode field
      """
      accessMode: AccessMode!
      """
      The value of the status field
      """
      status: ProjectStatus!
      """
      The value of the networkId field
      """
      networkId: Int
    }

    """
    Return type when updating many projects
    """
    type ProjectUpdateManyPayload
      @pgMutation(type: "Project")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [ProjectReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one Project
    """
    type ProjectUpdatePayload
      @pgMutation(type: "Project")
    {
      """
      Returned item(s) from the mutation
      """
      returning: ProjectReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type Query {
      """
      Query a unique Network
      """
      network(
        """
        Input for unique Network lookup
        """
        lookup: NetworkLookupInput!,
      ): Network @pgSelectOne
      """
      Query and paginate multiple networks
      """
      networks(
        """
        Filter for Network
        """
        filter: NetworkFilterInput,
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
        orderBy: [NetworkOrderByInput!],
      ): NetworkConnection! @pgSelectMany
      """
      Lookup multiple networks for subgraph joins
      """
      networkLookup(
        """
        Filter networks with an array of keys
        """
        lookup: NetworkManyLookupInput @inaccessible,
      ): [Network]! @pgLookup @lookup @inaccessible
      """
      Query a unique Project
      """
      project(
        """
        Input for unique Project lookup
        """
        lookup: ProjectLookupInput!,
      ): Project @pgSelectOne
      """
      Query and paginate multiple projects
      """
      projects(
        """
        Filter for Project
        """
        filter: ProjectFilterInput,
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
        orderBy: [ProjectOrderByInput!],
      ): ProjectConnection! @pgSelectMany
      """
      Lookup multiple projects for subgraph joins
      """
      projectLookup(
        """
        Filter projects with an array of keys
        """
        lookup: ProjectManyLookupInput @inaccessible,
      ): [Project]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single Network
      """
      networkCreate(
        """
        Input for creating a single Network
        """
        input: NetworkCreateInput!,
      ): NetworkCreatePayload! @pgInsertOne
      """
      Create multiple networks
      """
      networkCreateMany(
        """
        Input for creating multiple Network instances
        """
        input: [NetworkCreateInput!]!,
      ): NetworkCreateManyPayload! @pgInsertMany
      """
      Update a unique Network
      """
      networkUpdate(
        """
        Lookup input for unique Network update
        """
        lookup: NetworkLookupInput!,
        """
        Input for updating a Network
        """
        input: NetworkUpdateInput!,
      ): NetworkUpdatePayload! @pgUpdateOne
      """
      Update multiple networks
      """
      networkUpdateMany(
        """
        Filter for updating multiple Network instances
        """
        filter: NetworkFilterInput,
        """
        Input for updating multiple Network instances
        """
        input: NetworkUpdateInput!,
      ): NetworkUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Network
      """
      networkDelete(
        """
        Lookup input for unique Network deletion
        """
        lookup: NetworkLookupInput!,
      ): NetworkDeletePayload! @pgDeleteOne
      """
      Delete multiple networks
      """
      networkDeleteMany(
        """
        Filter for Network deletion
        """
        filter: NetworkFilterInput,
      ): NetworkDeleteManyPayload! @pgDeleteMany
      """
      Create a single Project
      """
      projectCreate(
        """
        Input for creating a single Project
        """
        input: ProjectCreateInput!,
      ): ProjectCreatePayload! @pgInsertOne
      """
      Create multiple projects
      """
      projectCreateMany(
        """
        Input for creating multiple Project instances
        """
        input: [ProjectCreateInput!]!,
      ): ProjectCreateManyPayload! @pgInsertMany
      """
      Update a unique Project
      """
      projectUpdate(
        """
        Lookup input for unique Project update
        """
        lookup: ProjectLookupInput!,
        """
        Input for updating a Project
        """
        input: ProjectUpdateInput!,
      ): ProjectUpdatePayload! @pgUpdateOne
      """
      Update multiple projects
      """
      projectUpdateMany(
        """
        Filter for updating multiple Project instances
        """
        filter: ProjectFilterInput,
        """
        Input for updating multiple Project instances
        """
        input: ProjectUpdateInput!,
      ): ProjectUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique Project
      """
      projectDelete(
        """
        Lookup input for unique Project deletion
        """
        lookup: ProjectLookupInput!,
      ): ProjectDeletePayload! @pgDeleteOne
      """
      Delete multiple projects
      """
      projectDeleteMany(
        """
        Filter for Project deletion
        """
        filter: ProjectFilterInput,
      ): ProjectDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn table_with_comment() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "commented_table" (
                id INT PRIMARY KEY
            );
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            COMMENT ON TABLE "commented_table" IS 'This is a table comment.';
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    Input for creating a new CommentedTable
    """
    input CommentedTableCreateInput {
      """
      Set field value for id
      """
      id: Int!
    }

    """
    Filter input type for CommentedTable objects.
    """
    input CommentedTableFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [CommentedTableFilterInput]
      """
      None of the filters must match
      """
      NONE: [CommentedTableFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [CommentedTableFilterInput]
    }

    """
    Input type to select a unique CommentedTable
    """
    input CommentedTableLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for CommentedTable objects for subgraph joins.
    """
    input CommentedTableManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for CommentedTable results.
    """
    input CommentedTableOrderByInput @oneOf {
      """
      Order commentedTables by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing CommentedTable
    """
    input CommentedTableUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
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
    This is a table comment.
    """
    type CommentedTable
      @key(fields: "id")
      @pgTable(name: "commented_table")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for CommentedTable
    """
    type CommentedTableConnection
      @pgConnection(type: "CommentedTable")
    {
      """
      A list of edges
      """
      edges: [CommentedTableEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many commentedTables
    """
    type CommentedTableCreateManyPayload
      @pgMutation(type: "CommentedTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentedTableReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one CommentedTable
    """
    type CommentedTableCreatePayload
      @pgMutation(type: "CommentedTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentedTableReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many commentedTables
    """
    type CommentedTableDeleteManyPayload
      @pgMutation(type: "CommentedTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentedTableReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one CommentedTable
    """
    type CommentedTableDeletePayload
      @pgMutation(type: "CommentedTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentedTableReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type CommentedTableEdge {
      """
      The item at the end of the edge
      """
      node: CommentedTable! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created CommentedTable object
    """
    type CommentedTableReturning
      @pgReturning(type: "CommentedTable")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when updating many commentedTables
    """
    type CommentedTableUpdateManyPayload
      @pgMutation(type: "CommentedTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentedTableReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one CommentedTable
    """
    type CommentedTableUpdatePayload
      @pgMutation(type: "CommentedTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentedTableReturning @shareable
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
      Query a unique CommentedTable
      """
      commentedTable(
        """
        Input for unique CommentedTable lookup
        """
        lookup: CommentedTableLookupInput!,
      ): CommentedTable @pgSelectOne
      """
      Query and paginate multiple commentedTables
      """
      commentedTables(
        """
        Filter for CommentedTable
        """
        filter: CommentedTableFilterInput,
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
        orderBy: [CommentedTableOrderByInput!],
      ): CommentedTableConnection! @pgSelectMany
      """
      Lookup multiple commentedTables for subgraph joins
      """
      commentedTableLookup(
        """
        Filter commentedTables with an array of keys
        """
        lookup: CommentedTableManyLookupInput @inaccessible,
      ): [CommentedTable]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single CommentedTable
      """
      commentedTableCreate(
        """
        Input for creating a single CommentedTable
        """
        input: CommentedTableCreateInput!,
      ): CommentedTableCreatePayload! @pgInsertOne
      """
      Create multiple commentedTables
      """
      commentedTableCreateMany(
        """
        Input for creating multiple CommentedTable instances
        """
        input: [CommentedTableCreateInput!]!,
      ): CommentedTableCreateManyPayload! @pgInsertMany
      """
      Update a unique CommentedTable
      """
      commentedTableUpdate(
        """
        Lookup input for unique CommentedTable update
        """
        lookup: CommentedTableLookupInput!,
        """
        Input for updating a CommentedTable
        """
        input: CommentedTableUpdateInput!,
      ): CommentedTableUpdatePayload! @pgUpdateOne
      """
      Update multiple commentedTables
      """
      commentedTableUpdateMany(
        """
        Filter for updating multiple CommentedTable instances
        """
        filter: CommentedTableFilterInput,
        """
        Input for updating multiple CommentedTable instances
        """
        input: CommentedTableUpdateInput!,
      ): CommentedTableUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique CommentedTable
      """
      commentedTableDelete(
        """
        Lookup input for unique CommentedTable deletion
        """
        lookup: CommentedTableLookupInput!,
      ): CommentedTableDeletePayload! @pgDeleteOne
      """
      Delete multiple commentedTables
      """
      commentedTableDeleteMany(
        """
        Filter for CommentedTable deletion
        """
        filter: CommentedTableFilterInput,
      ): CommentedTableDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn table_with_commented_column() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "commented_column_table" (
                id INT PRIMARY KEY,
                "data" TEXT
            );
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            COMMENT ON COLUMN "commented_column_table"."data" IS 'This is a column comment.';
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    Input for creating a new CommentedColumnTable
    """
    input CommentedColumnTableCreateInput {
      """
      Set field value for id
      """
      id: Int!
      """
      Set field value for data
      """
      data: String
    }

    """
    Filter input type for CommentedColumnTable objects.
    """
    input CommentedColumnTableFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the given data
      """
      data: StringFilterInput
      """
      All of the filters must match
      """
      ALL: [CommentedColumnTableFilterInput]
      """
      None of the filters must match
      """
      NONE: [CommentedColumnTableFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [CommentedColumnTableFilterInput]
    }

    """
    Input type to select a unique CommentedColumnTable
    """
    input CommentedColumnTableLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for CommentedColumnTable objects for subgraph joins.
    """
    input CommentedColumnTableManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for CommentedColumnTable results.
    """
    input CommentedColumnTableOrderByInput @oneOf {
      """
      Order commentedColumnTables by id
      """
      id: OrderDirection
      """
      Order commentedColumnTables by data
      """
      data: OrderDirection
    }

    """
    Input for updating an existing CommentedColumnTable
    """
    input CommentedColumnTableUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for data
      """
      data: StringUpdateInput
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

    type CommentedColumnTable
      @key(fields: "id")
      @pgTable(name: "commented_column_table")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      """
      This is a column comment.
      """
      data: String @pgColumn(name: "data", type: TEXT)
    }

    """
    The connection type for CommentedColumnTable
    """
    type CommentedColumnTableConnection
      @pgConnection(type: "CommentedColumnTable")
    {
      """
      A list of edges
      """
      edges: [CommentedColumnTableEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many commentedColumnTables
    """
    type CommentedColumnTableCreateManyPayload
      @pgMutation(type: "CommentedColumnTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentedColumnTableReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one CommentedColumnTable
    """
    type CommentedColumnTableCreatePayload
      @pgMutation(type: "CommentedColumnTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentedColumnTableReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many commentedColumnTables
    """
    type CommentedColumnTableDeleteManyPayload
      @pgMutation(type: "CommentedColumnTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentedColumnTableReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one CommentedColumnTable
    """
    type CommentedColumnTableDeletePayload
      @pgMutation(type: "CommentedColumnTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentedColumnTableReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type CommentedColumnTableEdge {
      """
      The item at the end of the edge
      """
      node: CommentedColumnTable! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created CommentedColumnTable object
    """
    type CommentedColumnTableReturning
      @pgReturning(type: "CommentedColumnTable")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
      """
      The value of the data field
      """
      data: String
    }

    """
    Return type when updating many commentedColumnTables
    """
    type CommentedColumnTableUpdateManyPayload
      @pgMutation(type: "CommentedColumnTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [CommentedColumnTableReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one CommentedColumnTable
    """
    type CommentedColumnTableUpdatePayload
      @pgMutation(type: "CommentedColumnTable")
    {
      """
      Returned item(s) from the mutation
      """
      returning: CommentedColumnTableReturning @shareable
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
      Query a unique CommentedColumnTable
      """
      commentedColumnTable(
        """
        Input for unique CommentedColumnTable lookup
        """
        lookup: CommentedColumnTableLookupInput!,
      ): CommentedColumnTable @pgSelectOne
      """
      Query and paginate multiple commentedColumnTables
      """
      commentedColumnTables(
        """
        Filter for CommentedColumnTable
        """
        filter: CommentedColumnTableFilterInput,
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
        orderBy: [CommentedColumnTableOrderByInput!],
      ): CommentedColumnTableConnection! @pgSelectMany
      """
      Lookup multiple commentedColumnTables for subgraph joins
      """
      commentedColumnTableLookup(
        """
        Filter commentedColumnTables with an array of keys
        """
        lookup: CommentedColumnTableManyLookupInput @inaccessible,
      ): [CommentedColumnTable]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single CommentedColumnTable
      """
      commentedColumnTableCreate(
        """
        Input for creating a single CommentedColumnTable
        """
        input: CommentedColumnTableCreateInput!,
      ): CommentedColumnTableCreatePayload! @pgInsertOne
      """
      Create multiple commentedColumnTables
      """
      commentedColumnTableCreateMany(
        """
        Input for creating multiple CommentedColumnTable instances
        """
        input: [CommentedColumnTableCreateInput!]!,
      ): CommentedColumnTableCreateManyPayload! @pgInsertMany
      """
      Update a unique CommentedColumnTable
      """
      commentedColumnTableUpdate(
        """
        Lookup input for unique CommentedColumnTable update
        """
        lookup: CommentedColumnTableLookupInput!,
        """
        Input for updating a CommentedColumnTable
        """
        input: CommentedColumnTableUpdateInput!,
      ): CommentedColumnTableUpdatePayload! @pgUpdateOne
      """
      Update multiple commentedColumnTables
      """
      commentedColumnTableUpdateMany(
        """
        Filter for updating multiple CommentedColumnTable instances
        """
        filter: CommentedColumnTableFilterInput,
        """
        Input for updating multiple CommentedColumnTable instances
        """
        input: CommentedColumnTableUpdateInput!,
      ): CommentedColumnTableUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique CommentedColumnTable
      """
      commentedColumnTableDelete(
        """
        Lookup input for unique CommentedColumnTable deletion
        """
        lookup: CommentedColumnTableLookupInput!,
      ): CommentedColumnTableDeletePayload! @pgDeleteOne
      """
      Delete multiple commentedColumnTables
      """
      commentedColumnTableDeleteMany(
        """
        Filter for CommentedColumnTable deletion
        """
        filter: CommentedColumnTableFilterInput,
      ): CommentedColumnTableDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn enum_with_comment() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TYPE "commented_enum" AS ENUM ('ONE', 'TWO');
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            COMMENT ON TYPE "commented_enum" IS 'This is an enum comment.';
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "uses_commented_enum" (
                id INT PRIMARY KEY,
                val "commented_enum" NOT NULL
            );
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    This is an enum comment.
    """
    enum CommentedEnum @pgEnum(name: "commented_enum") {
      ONE @pgEnumVariant(name: "ONE")
      TWO @pgEnumVariant(name: "TWO")
    }

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
    Search filter input for CommentedEnum type.
    """
    input CommentedEnumFilterInput @oneOf {
      """
      The value is exactly the one given
      """
      eq: CommentedEnum
      """
      The value is not the one given
      """
      ne: CommentedEnum
      """
      The value is greater than the one given
      """
      gt: CommentedEnum
      """
      The value is less than the one given
      """
      lt: CommentedEnum
      """
      The value is greater than, or equal to the one given
      """
      gte: CommentedEnum
      """
      The value is less than, or equal to the one given
      """
      lte: CommentedEnum
      """
      The value is in the given array of values
      """
      in: [CommentedEnum!]
      """
      The value is not in the given array of values
      """
      nin: [CommentedEnum!]
      """
      A negation of the given filter
      """
      not: CommentedEnumFilterInput
    }

    """
    Update input for CommentedEnum type.
    """
    input CommentedEnumUpdateInput @oneOf {
      """
      Replaces the value of a field with the specified value.
      """
      set: CommentedEnum
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
    Input for creating a new UsesCommentedEnum
    """
    input UsesCommentedEnumCreateInput {
      """
      Set field value for id
      """
      id: Int!
      """
      Set field value for val
      """
      val: CommentedEnum!
    }

    """
    Filter input type for UsesCommentedEnum objects.
    """
    input UsesCommentedEnumFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the given val
      """
      val: CommentedEnumFilterInput
      """
      All of the filters must match
      """
      ALL: [UsesCommentedEnumFilterInput]
      """
      None of the filters must match
      """
      NONE: [UsesCommentedEnumFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UsesCommentedEnumFilterInput]
    }

    """
    Input type to select a unique UsesCommentedEnum
    """
    input UsesCommentedEnumLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for UsesCommentedEnum objects for subgraph joins.
    """
    input UsesCommentedEnumManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for UsesCommentedEnum results.
    """
    input UsesCommentedEnumOrderByInput @oneOf {
      """
      Order usesCommentedEnums by id
      """
      id: OrderDirection
      """
      Order usesCommentedEnums by val
      """
      val: OrderDirection
    }

    """
    Input for updating an existing UsesCommentedEnum
    """
    input UsesCommentedEnumUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for val
      """
      val: CommentedEnumUpdateInput
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

    type UsesCommentedEnum
      @key(fields: "id")
      @pgTable(name: "uses_commented_enum")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      val: CommentedEnum! @pgColumn(name: "val", type: ENUM)
    }

    """
    The connection type for UsesCommentedEnum
    """
    type UsesCommentedEnumConnection
      @pgConnection(type: "UsesCommentedEnum")
    {
      """
      A list of edges
      """
      edges: [UsesCommentedEnumEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many usesCommentedEnums
    """
    type UsesCommentedEnumCreateManyPayload
      @pgMutation(type: "UsesCommentedEnum")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UsesCommentedEnumReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one UsesCommentedEnum
    """
    type UsesCommentedEnumCreatePayload
      @pgMutation(type: "UsesCommentedEnum")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UsesCommentedEnumReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many usesCommentedEnums
    """
    type UsesCommentedEnumDeleteManyPayload
      @pgMutation(type: "UsesCommentedEnum")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UsesCommentedEnumReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one UsesCommentedEnum
    """
    type UsesCommentedEnumDeletePayload
      @pgMutation(type: "UsesCommentedEnum")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UsesCommentedEnumReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UsesCommentedEnumEdge {
      """
      The item at the end of the edge
      """
      node: UsesCommentedEnum! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created UsesCommentedEnum object
    """
    type UsesCommentedEnumReturning
      @pgReturning(type: "UsesCommentedEnum")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
      """
      The value of the val field
      """
      val: CommentedEnum!
    }

    """
    Return type when updating many usesCommentedEnums
    """
    type UsesCommentedEnumUpdateManyPayload
      @pgMutation(type: "UsesCommentedEnum")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UsesCommentedEnumReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one UsesCommentedEnum
    """
    type UsesCommentedEnumUpdatePayload
      @pgMutation(type: "UsesCommentedEnum")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UsesCommentedEnumReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type Query {
      """
      Query a unique UsesCommentedEnum
      """
      usesCommentedEnum(
        """
        Input for unique UsesCommentedEnum lookup
        """
        lookup: UsesCommentedEnumLookupInput!,
      ): UsesCommentedEnum @pgSelectOne
      """
      Query and paginate multiple usesCommentedEnums
      """
      usesCommentedEnums(
        """
        Filter for UsesCommentedEnum
        """
        filter: UsesCommentedEnumFilterInput,
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
        orderBy: [UsesCommentedEnumOrderByInput!],
      ): UsesCommentedEnumConnection! @pgSelectMany
      """
      Lookup multiple usesCommentedEnums for subgraph joins
      """
      usesCommentedEnumLookup(
        """
        Filter usesCommentedEnums with an array of keys
        """
        lookup: UsesCommentedEnumManyLookupInput @inaccessible,
      ): [UsesCommentedEnum]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single UsesCommentedEnum
      """
      usesCommentedEnumCreate(
        """
        Input for creating a single UsesCommentedEnum
        """
        input: UsesCommentedEnumCreateInput!,
      ): UsesCommentedEnumCreatePayload! @pgInsertOne
      """
      Create multiple usesCommentedEnums
      """
      usesCommentedEnumCreateMany(
        """
        Input for creating multiple UsesCommentedEnum instances
        """
        input: [UsesCommentedEnumCreateInput!]!,
      ): UsesCommentedEnumCreateManyPayload! @pgInsertMany
      """
      Update a unique UsesCommentedEnum
      """
      usesCommentedEnumUpdate(
        """
        Lookup input for unique UsesCommentedEnum update
        """
        lookup: UsesCommentedEnumLookupInput!,
        """
        Input for updating a UsesCommentedEnum
        """
        input: UsesCommentedEnumUpdateInput!,
      ): UsesCommentedEnumUpdatePayload! @pgUpdateOne
      """
      Update multiple usesCommentedEnums
      """
      usesCommentedEnumUpdateMany(
        """
        Filter for updating multiple UsesCommentedEnum instances
        """
        filter: UsesCommentedEnumFilterInput,
        """
        Input for updating multiple UsesCommentedEnum instances
        """
        input: UsesCommentedEnumUpdateInput!,
      ): UsesCommentedEnumUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique UsesCommentedEnum
      """
      usesCommentedEnumDelete(
        """
        Lookup input for unique UsesCommentedEnum deletion
        """
        lookup: UsesCommentedEnumLookupInput!,
      ): UsesCommentedEnumDeletePayload! @pgDeleteOne
      """
      Delete multiple usesCommentedEnums
      """
      usesCommentedEnumDeleteMany(
        """
        Filter for UsesCommentedEnum deletion
        """
        filter: UsesCommentedEnumFilterInput,
      ): UsesCommentedEnumDeleteManyPayload! @pgDeleteMany
    }
    "#);
}

#[tokio::test]
async fn table_with_commented_foreign_key() {
    let api = PgTestApi::new("", |api| async move {
        let create_user = indoc! {r#"
           CREATE TABLE "User_fk_comment" (
               id SERIAL PRIMARY KEY
           );
        "#};

        api.execute_sql(create_user).await;

        let create_post = indoc! {r#"
            CREATE TABLE "Post_fk_comment" (
                id SERIAL PRIMARY KEY,
                user_id INT NOT NULL,
                CONSTRAINT "Post_User_FK_Comment" FOREIGN KEY (user_id) REFERENCES "User_fk_comment"(id)
            );
        "#};

        api.execute_sql(create_post).await;

        let schema = indoc! {r#"
            COMMENT ON CONSTRAINT "Post_User_FK_Comment"
            ON "Post_fk_comment"
            IS 'Links post to its author.';
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let result = api.introspect().await;

    insta::assert_snapshot!(&result, @r#"
    extend schema
      @link(
        url: "https://grafbase.com/extensions/postgres/0.1.1",
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
    Filter input type for PostFkComment collections
    """
    input PostFkCommentCollectionFilterInput {
      """
      The object is related to an object with the given fields
      """
      contains: PostFkCommentFilterInput
    }

    """
    Input for creating a new PostFkComment
    """
    input PostFkCommentCreateInput {
      """
      Set field value for id
      """
      id: Int
      """
      Set field value for userId
      """
      userId: Int!
    }

    """
    Filter input type for PostFkComment objects.
    """
    input PostFkCommentFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the given userId
      """
      userId: IntFilterInput
      """
      Filter by the related UserFkComment object
      """
      userFkComment: UserFkCommentFilterInput
      """
      All of the filters must match
      """
      ALL: [PostFkCommentFilterInput]
      """
      None of the filters must match
      """
      NONE: [PostFkCommentFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [PostFkCommentFilterInput]
    }

    """
    Input type to select a unique PostFkComment
    """
    input PostFkCommentLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for PostFkComment objects for subgraph joins.
    """
    input PostFkCommentManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for PostFkComment results.
    """
    input PostFkCommentOrderByInput @oneOf {
      """
      Order postFkComments by id
      """
      id: OrderDirection
      """
      Order postFkComments by userId
      """
      userId: OrderDirection
      """
      Order PostFkComment results by UserFkComment fields
      """
      userFkComment: UserFkCommentOrderByInput
    }

    """
    Input for updating an existing PostFkComment
    """
    input PostFkCommentUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
      """
      Update field value for userId
      """
      userId: IntUpdateInput
    }

    """
    Input for creating a new UserFkComment
    """
    input UserFkCommentCreateInput {
      """
      Set field value for id
      """
      id: Int
    }

    """
    Filter input type for UserFkComment objects.
    """
    input UserFkCommentFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the related PostFkComment objects
      """
      postFkComments: PostFkCommentCollectionFilterInput
      """
      All of the filters must match
      """
      ALL: [UserFkCommentFilterInput]
      """
      None of the filters must match
      """
      NONE: [UserFkCommentFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [UserFkCommentFilterInput]
    }

    """
    Input type to select a unique UserFkComment
    """
    input UserFkCommentLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for UserFkComment objects for subgraph joins.
    """
    input UserFkCommentManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for UserFkComment results.
    """
    input UserFkCommentOrderByInput @oneOf {
      """
      Order userFkComments by id
      """
      id: OrderDirection
    }

    """
    Input for updating an existing UserFkComment
    """
    input UserFkCommentUpdateInput {
      """
      Update field value for id
      """
      id: IntUpdateInput
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

    type PostFkComment
      @key(fields: "id")
      @pgTable(name: "Post_fk_comment")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      userId: Int! @pgColumn(name: "user_id", type: INT)
      """
      Links post to its author.
      """
      userFkComment: UserFkComment! @pgRelation(name: "Post_User_FK_Comment", fields: ["userId"], references: ["id"])
    }

    """
    The connection type for PostFkComment
    """
    type PostFkCommentConnection
      @pgConnection(type: "PostFkComment")
    {
      """
      A list of edges
      """
      edges: [PostFkCommentEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many postFkComments
    """
    type PostFkCommentCreateManyPayload
      @pgMutation(type: "PostFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostFkCommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one PostFkComment
    """
    type PostFkCommentCreatePayload
      @pgMutation(type: "PostFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostFkCommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many postFkComments
    """
    type PostFkCommentDeleteManyPayload
      @pgMutation(type: "PostFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostFkCommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one PostFkComment
    """
    type PostFkCommentDeletePayload
      @pgMutation(type: "PostFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostFkCommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type PostFkCommentEdge {
      """
      The item at the end of the edge
      """
      node: PostFkComment! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created PostFkComment object
    """
    type PostFkCommentReturning
      @pgReturning(type: "PostFkComment")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
      """
      The value of the userId field
      """
      userId: Int!
    }

    """
    Return type when updating many postFkComments
    """
    type PostFkCommentUpdateManyPayload
      @pgMutation(type: "PostFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [PostFkCommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one PostFkComment
    """
    type PostFkCommentUpdatePayload
      @pgMutation(type: "PostFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: PostFkCommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type UserFkComment
      @key(fields: "id")
      @pgTable(name: "User_fk_comment")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      """
      Links post to its author.
      """
      postFkComments(
        """
        Filter the related PostFkComment instances
        """
        filter: PostFkCommentFilterInput,
        """
        Select the first PostFkComment instances
        """
        first: Int,
        """
        Select the last PostFkComment instances
        """
        last: Int,
        """
        Select the PostFkComment instances before the given cursor
        """
        before: String,
        """
        Select the PostFkComment instances after the given cursor
        """
        after: String,
        """
        Order the PostFkComment instances by the given fields
        """
        orderBy: [PostFkCommentOrderByInput!],
      ): PostFkCommentConnection! @pgRelation(name: "Post_User_FK_Comment")
    }

    """
    The connection type for UserFkComment
    """
    type UserFkCommentConnection
      @pgConnection(type: "UserFkComment")
    {
      """
      A list of edges
      """
      edges: [UserFkCommentEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    Return type when creating many userFkComments
    """
    type UserFkCommentCreateManyPayload
      @pgMutation(type: "UserFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserFkCommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when creating one UserFkComment
    """
    type UserFkCommentCreatePayload
      @pgMutation(type: "UserFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserFkCommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting many userFkComments
    """
    type UserFkCommentDeleteManyPayload
      @pgMutation(type: "UserFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserFkCommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when deleting one UserFkComment
    """
    type UserFkCommentDeletePayload
      @pgMutation(type: "UserFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserFkCommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type UserFkCommentEdge {
      """
      The item at the end of the edge
      """
      node: UserFkComment! @shareable
      """
      A cursor for use in pagination
      """
      cursor: String! @shareable
    }

    """
    Return type containing fields of the mutated or created UserFkComment object
    """
    type UserFkCommentReturning
      @pgReturning(type: "UserFkComment")
    {
      """
      The value of the id field
      """
      id: Int! @shareable
    }

    """
    Return type when updating many userFkComments
    """
    type UserFkCommentUpdateManyPayload
      @pgMutation(type: "UserFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: [UserFkCommentReturning]! @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    """
    Return type when updating one UserFkComment
    """
    type UserFkCommentUpdatePayload
      @pgMutation(type: "UserFkComment")
    {
      """
      Returned item(s) from the mutation
      """
      returning: UserFkCommentReturning @shareable
      """
      The number of rows mutated
      """
      rowCount: Int! @shareable
    }

    type Query {
      """
      Query a unique PostFkComment
      """
      postFkComment(
        """
        Input for unique PostFkComment lookup
        """
        lookup: PostFkCommentLookupInput!,
      ): PostFkComment @pgSelectOne
      """
      Query and paginate multiple postFkComments
      """
      postFkComments(
        """
        Filter for PostFkComment
        """
        filter: PostFkCommentFilterInput,
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
        orderBy: [PostFkCommentOrderByInput!],
      ): PostFkCommentConnection! @pgSelectMany
      """
      Lookup multiple postFkComments for subgraph joins
      """
      postFkCommentLookup(
        """
        Filter postFkComments with an array of keys
        """
        lookup: PostFkCommentManyLookupInput @inaccessible,
      ): [PostFkComment]! @pgLookup @lookup @inaccessible
      """
      Query a unique UserFkComment
      """
      userFkComment(
        """
        Input for unique UserFkComment lookup
        """
        lookup: UserFkCommentLookupInput!,
      ): UserFkComment @pgSelectOne
      """
      Query and paginate multiple userFkComments
      """
      userFkComments(
        """
        Filter for UserFkComment
        """
        filter: UserFkCommentFilterInput,
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
        orderBy: [UserFkCommentOrderByInput!],
      ): UserFkCommentConnection! @pgSelectMany
      """
      Lookup multiple userFkComments for subgraph joins
      """
      userFkCommentLookup(
        """
        Filter userFkComments with an array of keys
        """
        lookup: UserFkCommentManyLookupInput @inaccessible,
      ): [UserFkComment]! @pgLookup @lookup @inaccessible
    }

    type Mutation {
      """
      Create a single PostFkComment
      """
      postFkCommentCreate(
        """
        Input for creating a single PostFkComment
        """
        input: PostFkCommentCreateInput!,
      ): PostFkCommentCreatePayload! @pgInsertOne
      """
      Create multiple postFkComments
      """
      postFkCommentCreateMany(
        """
        Input for creating multiple PostFkComment instances
        """
        input: [PostFkCommentCreateInput!]!,
      ): PostFkCommentCreateManyPayload! @pgInsertMany
      """
      Update a unique PostFkComment
      """
      postFkCommentUpdate(
        """
        Lookup input for unique PostFkComment update
        """
        lookup: PostFkCommentLookupInput!,
        """
        Input for updating a PostFkComment
        """
        input: PostFkCommentUpdateInput!,
      ): PostFkCommentUpdatePayload! @pgUpdateOne
      """
      Update multiple postFkComments
      """
      postFkCommentUpdateMany(
        """
        Filter for updating multiple PostFkComment instances
        """
        filter: PostFkCommentFilterInput,
        """
        Input for updating multiple PostFkComment instances
        """
        input: PostFkCommentUpdateInput!,
      ): PostFkCommentUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique PostFkComment
      """
      postFkCommentDelete(
        """
        Lookup input for unique PostFkComment deletion
        """
        lookup: PostFkCommentLookupInput!,
      ): PostFkCommentDeletePayload! @pgDeleteOne
      """
      Delete multiple postFkComments
      """
      postFkCommentDeleteMany(
        """
        Filter for PostFkComment deletion
        """
        filter: PostFkCommentFilterInput,
      ): PostFkCommentDeleteManyPayload! @pgDeleteMany
      """
      Create a single UserFkComment
      """
      userFkCommentCreate(
        """
        Input for creating a single UserFkComment
        """
        input: UserFkCommentCreateInput!,
      ): UserFkCommentCreatePayload! @pgInsertOne
      """
      Create multiple userFkComments
      """
      userFkCommentCreateMany(
        """
        Input for creating multiple UserFkComment instances
        """
        input: [UserFkCommentCreateInput!]!,
      ): UserFkCommentCreateManyPayload! @pgInsertMany
      """
      Update a unique UserFkComment
      """
      userFkCommentUpdate(
        """
        Lookup input for unique UserFkComment update
        """
        lookup: UserFkCommentLookupInput!,
        """
        Input for updating a UserFkComment
        """
        input: UserFkCommentUpdateInput!,
      ): UserFkCommentUpdatePayload! @pgUpdateOne
      """
      Update multiple userFkComments
      """
      userFkCommentUpdateMany(
        """
        Filter for updating multiple UserFkComment instances
        """
        filter: UserFkCommentFilterInput,
        """
        Input for updating multiple UserFkComment instances
        """
        input: UserFkCommentUpdateInput!,
      ): UserFkCommentUpdateManyPayload! @pgUpdateMany
      """
      Delete a unique UserFkComment
      """
      userFkCommentDelete(
        """
        Lookup input for unique UserFkComment deletion
        """
        lookup: UserFkCommentLookupInput!,
      ): UserFkCommentDeletePayload! @pgDeleteOne
      """
      Delete multiple userFkComments
      """
      userFkCommentDeleteMany(
        """
        Filter for UserFkComment deletion
        """
        filter: UserFkCommentFilterInput,
      ): UserFkCommentDeleteManyPayload! @pgDeleteMany
    }
    "#);
}
