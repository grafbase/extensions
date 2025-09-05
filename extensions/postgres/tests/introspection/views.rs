use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn view_with_int_unique() {
    let api = PgTestApi::new("", |api| async move {
        let create_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT UNIQUE NOT NULL
            )
        "#};

        api.execute_sql(create_table).await;

        let create_view = indoc! {r#"
            CREATE VIEW "filtered_users" AS
            SELECT id FROM "User" WHERE id < 3;
        "#};

        api.execute_sql(create_view).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.views.filtered_users.columns.id]
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
    Filter input type for FilteredUser objects.
    """
    input FilteredUserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [FilteredUserFilterInput]
      """
      None of the filters must match
      """
      NONE: [FilteredUserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [FilteredUserFilterInput]
    }

    """
    Input type to select a unique FilteredUser
    """
    input FilteredUserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for FilteredUser objects for subgraph joins.
    """
    input FilteredUserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for FilteredUser results.
    """
    input FilteredUserOrderByInput @oneOf {
      """
      Order filteredUsers by id
      """
      id: OrderDirection
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

    type FilteredUser
      @key(fields: "id")
      @pgTable(name: "filtered_users", kind: VIEW)
      @pgKey(fields: ["id"], type: UNIQUE)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for FilteredUser
    """
    type FilteredUserConnection
      @pgConnection(type: "FilteredUser")
    {
      """
      A list of edges
      """
      edges: [FilteredUserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type FilteredUserEdge {
      """
      The item at the end of the edge
      """
      node: FilteredUser! @shareable
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
      ): [User] @pgLookup @lookup @inaccessible
      """
      Query a unique FilteredUser
      """
      filteredUser(
        """
        Input for unique FilteredUser lookup
        """
        lookup: FilteredUserLookupInput!,
      ): FilteredUser @pgSelectOne
      """
      Query and paginate multiple filteredUsers
      """
      filteredUsers(
        """
        Filter for FilteredUser
        """
        filter: FilteredUserFilterInput,
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
        orderBy: [FilteredUserOrderByInput!],
      ): FilteredUserConnection! @pgSelectMany
      """
      Lookup multiple filteredUsers for subgraph joins
      """
      filteredUserLookup(
        """
        Filter filteredUsers with an array of keys
        """
        lookup: FilteredUserManyLookupInput @inaccessible,
      ): [FilteredUser] @pgLookup @lookup @inaccessible
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
async fn materialized_view_with_int_unique() {
    let api = PgTestApi::new("", |api| async move {
        let create_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT UNIQUE NOT NULL
            )
        "#};

        api.execute_sql(create_table).await;

        let create_view = indoc! {r#"
            CREATE MATERIALIZED VIEW "filtered_users" AS
            SELECT id FROM "User" WHERE id < 3;
        "#};

        api.execute_sql(create_view).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.views.filtered_users.columns.id]
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
    Filter input type for FilteredUser objects.
    """
    input FilteredUserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [FilteredUserFilterInput]
      """
      None of the filters must match
      """
      NONE: [FilteredUserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [FilteredUserFilterInput]
    }

    """
    Input type to select a unique FilteredUser
    """
    input FilteredUserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for FilteredUser objects for subgraph joins.
    """
    input FilteredUserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for FilteredUser results.
    """
    input FilteredUserOrderByInput @oneOf {
      """
      Order filteredUsers by id
      """
      id: OrderDirection
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

    type FilteredUser
      @key(fields: "id")
      @pgTable(name: "filtered_users", kind: MATERIALIZED_VIEW)
      @pgKey(fields: ["id"], type: UNIQUE)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    """
    The connection type for FilteredUser
    """
    type FilteredUserConnection
      @pgConnection(type: "FilteredUser")
    {
      """
      A list of edges
      """
      edges: [FilteredUserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type FilteredUserEdge {
      """
      The item at the end of the edge
      """
      node: FilteredUser! @shareable
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
      ): [User] @pgLookup @lookup @inaccessible
      """
      Query a unique FilteredUser
      """
      filteredUser(
        """
        Input for unique FilteredUser lookup
        """
        lookup: FilteredUserLookupInput!,
      ): FilteredUser @pgSelectOne
      """
      Query and paginate multiple filteredUsers
      """
      filteredUsers(
        """
        Filter for FilteredUser
        """
        filter: FilteredUserFilterInput,
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
        orderBy: [FilteredUserOrderByInput!],
      ): FilteredUserConnection! @pgSelectMany
      """
      Lookup multiple filteredUsers for subgraph joins
      """
      filteredUserLookup(
        """
        Filter filteredUsers with an array of keys
        """
        lookup: FilteredUserManyLookupInput @inaccessible,
      ): [FilteredUser] @pgLookup @lookup @inaccessible
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
async fn view_with_composite_key() {
    let api = PgTestApi::new("", |api| async move {
        let create_table = indoc! {r#"
            CREATE TABLE "User" (
                name VARCHAR(255) NOT NULL,
                email_address VARCHAR(255) NOT NULL,
                PRIMARY KEY (name, email_address)
            )
        "#};

        api.execute_sql(create_table).await;

        let create_view = indoc! {r#"
            CREATE VIEW "filtered_users" AS
            SELECT name, email_address FROM "User"
        "#};

        api.execute_sql(create_view).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.views.filtered_users]
        unique_keys = [
          ["name", "email_address"]
        ]

        [schemas.public.views.filtered_users.columns.name]
        nullable = false

        [schemas.public.views.filtered_users.columns.email_address]
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
    Filter input type for FilteredUser objects.
    """
    input FilteredUserFilterInput @oneOf {
      """
      Filter by the given name
      """
      name: StringFilterInput
      """
      Filter by the given emailAddress
      """
      emailAddress: StringFilterInput
      """
      All of the filters must match
      """
      ALL: [FilteredUserFilterInput]
      """
      None of the filters must match
      """
      NONE: [FilteredUserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [FilteredUserFilterInput]
    }

    """
    Input type to select a unique FilteredUser
    """
    input FilteredUserLookupInput @oneOf {
      """
      Select FilteredUser by composite columns 'name, emailAddress'
      """
      nameEmailAddress: FilteredUserNameEmailAddressInput
    }

    """
    Lookup input type for FilteredUser objects for subgraph joins.
    """
    input FilteredUserManyLookupInput @oneOf @inaccessible {
      """
      Select FilteredUser by composite columns 'name, emailAddress'
      """
      nameEmailAddress: [FilteredUserNameEmailAddressInput!] @inaccessible
    }

    """
    Input type to select a unique FilteredUser with multiple fields
    """
    input FilteredUserNameEmailAddressInput {
      """
      Select by the 'name' field
      """
      name: String!
      """
      Select by the 'emailAddress' field
      """
      emailAddress: String!
    }

    """
    Specifies the ordering for FilteredUser results.
    """
    input FilteredUserOrderByInput @oneOf {
      """
      Order filteredUsers by name
      """
      name: OrderDirection
      """
      Order filteredUsers by emailAddress
      """
      emailAddress: OrderDirection
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
      Set field value for emailAddress
      """
      emailAddress: String!
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
      Filter by the given emailAddress
      """
      emailAddress: StringFilterInput
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
      Select User by composite columns 'name, emailAddress'
      """
      nameEmailAddress: UserNameEmailAddressInput
    }

    """
    Lookup input type for User objects for subgraph joins.
    """
    input UserManyLookupInput @oneOf @inaccessible {
      """
      Select User by composite columns 'name, emailAddress'
      """
      nameEmailAddress: [UserNameEmailAddressInput!] @inaccessible
    }

    """
    Input type to select a unique User with multiple fields
    """
    input UserNameEmailAddressInput {
      """
      Select by the 'name' field
      """
      name: String!
      """
      Select by the 'emailAddress' field
      """
      emailAddress: String!
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
      Order users by emailAddress
      """
      emailAddress: OrderDirection
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
      Update field value for emailAddress
      """
      emailAddress: StringUpdateInput
    }

    type FilteredUser
      @key(fields: "name emailAddress")
      @pgTable(name: "filtered_users", kind: VIEW)
      @pgKey(fields: ["name", "emailAddress"], type: UNIQUE)
    {
      name: String! @pgColumn(name: "name", type: VARCHAR)
      emailAddress: String! @pgColumn(name: "email_address", type: VARCHAR)
    }

    """
    The connection type for FilteredUser
    """
    type FilteredUserConnection
      @pgConnection(type: "FilteredUser")
    {
      """
      A list of edges
      """
      edges: [FilteredUserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type FilteredUserEdge {
      """
      The item at the end of the edge
      """
      node: FilteredUser! @shareable
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
      @key(fields: "name emailAddress")
      @pgTable(name: "User")
      @pgKey(fields: ["name", "emailAddress"], type: PRIMARY)
    {
      name: String! @pgColumn(name: "name", type: VARCHAR)
      emailAddress: String! @pgColumn(name: "email_address", type: VARCHAR)
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
      The value of the emailAddress field
      """
      emailAddress: String! @shareable
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
      ): [User] @pgLookup @lookup @inaccessible
      """
      Query a unique FilteredUser
      """
      filteredUser(
        """
        Input for unique FilteredUser lookup
        """
        lookup: FilteredUserLookupInput!,
      ): FilteredUser @pgSelectOne
      """
      Query and paginate multiple filteredUsers
      """
      filteredUsers(
        """
        Filter for FilteredUser
        """
        filter: FilteredUserFilterInput,
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
        orderBy: [FilteredUserOrderByInput!],
      ): FilteredUserConnection! @pgSelectMany
      """
      Lookup multiple filteredUsers for subgraph joins
      """
      filteredUserLookup(
        """
        Filter filteredUsers with an array of keys
        """
        lookup: FilteredUserManyLookupInput @inaccessible,
      ): [FilteredUser] @pgLookup @lookup @inaccessible
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
async fn view_with_relation_from_view_to_table() {
    let api = PgTestApi::new("", |api| async move {
        let create_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT UNIQUE NOT NULL
            )
        "#};

        api.execute_sql(create_table).await;

        let create_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT UNIQUE NOT NULL,
                author_id INT NOT NULL
            )
        "#};

        api.execute_sql(create_table).await;

        let create_view = indoc! {r#"
            CREATE VIEW "filtered_users" AS
            SELECT id FROM "User" WHERE id < 3;
        "#};

        api.execute_sql(create_view).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.views.filtered_users.columns.id]
        unique = true
        nullable = false

        [schemas.public.views.filtered_users.relations.filtered_users_to_blogs]
        referenced_table = "Blog"
        referencing_columns = ["id"]
        referenced_columns = ["author_id"]
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
      id: Int!
      """
      Set field value for authorId
      """
      authorId: Int!
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
      Filter by the given authorId
      """
      authorId: IntFilterInput
      """
      Filter by the related FilteredUser object
      """
      filteredUser: FilteredUserFilterInput
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
      Order blogs by authorId
      """
      authorId: OrderDirection
      """
      Order Blog results by FilteredUser fields
      """
      filteredUser: FilteredUserOrderByInput
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
      Update field value for authorId
      """
      authorId: IntUpdateInput
    }

    """
    Filter input type for FilteredUser objects.
    """
    input FilteredUserFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the related Blog objects
      """
      blogs: BlogCollectionFilterInput
      """
      All of the filters must match
      """
      ALL: [FilteredUserFilterInput]
      """
      None of the filters must match
      """
      NONE: [FilteredUserFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [FilteredUserFilterInput]
    }

    """
    Input type to select a unique FilteredUser
    """
    input FilteredUserLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for FilteredUser objects for subgraph joins.
    """
    input FilteredUserManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for FilteredUser results.
    """
    input FilteredUserOrderByInput @oneOf {
      """
      Order filteredUsers by id
      """
      id: OrderDirection
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

    type Blog
      @key(fields: "id")
      @pgTable(name: "Blog")
      @pgKey(fields: ["id"], type: UNIQUE)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      authorId: Int! @pgColumn(name: "author_id", type: INT)
      filteredUser: FilteredUser @pgRelation(name: "filtered_users_to_blogs")
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
      The value of the authorId field
      """
      authorId: Int!
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

    type FilteredUser
      @key(fields: "id")
      @pgTable(name: "filtered_users", kind: VIEW)
      @pgKey(fields: ["id"], type: UNIQUE)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      ): BlogConnection! @pgRelation(name: "filtered_users_to_blogs", fields: ["id"], references: ["authorId"])
    }

    """
    The connection type for FilteredUser
    """
    type FilteredUserConnection
      @pgConnection(type: "FilteredUser")
    {
      """
      A list of edges
      """
      edges: [FilteredUserEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type FilteredUserEdge {
      """
      The item at the end of the edge
      """
      node: FilteredUser! @shareable
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
      ): [Blog] @pgLookup @lookup @inaccessible
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
      ): [User] @pgLookup @lookup @inaccessible
      """
      Query a unique FilteredUser
      """
      filteredUser(
        """
        Input for unique FilteredUser lookup
        """
        lookup: FilteredUserLookupInput!,
      ): FilteredUser @pgSelectOne
      """
      Query and paginate multiple filteredUsers
      """
      filteredUsers(
        """
        Filter for FilteredUser
        """
        filter: FilteredUserFilterInput,
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
        orderBy: [FilteredUserOrderByInput!],
      ): FilteredUserConnection! @pgSelectMany
      """
      Lookup multiple filteredUsers for subgraph joins
      """
      filteredUserLookup(
        """
        Filter filteredUsers with an array of keys
        """
        lookup: FilteredUserManyLookupInput @inaccessible,
      ): [FilteredUser] @pgLookup @lookup @inaccessible
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
async fn view_with_relation_from_table_to_view() {
    let api = PgTestApi::new("", |api| async move {
        let create_table = indoc! {r#"
            CREATE TABLE "User" (
                id INT UNIQUE NOT NULL
            )
        "#};

        api.execute_sql(create_table).await;

        let create_table = indoc! {r#"
            CREATE TABLE "Blog" (
                id INT UNIQUE NOT NULL,
                author_id INT NOT NULL
            )
        "#};

        api.execute_sql(create_table).await;

        let create_view = indoc! {r#"
            CREATE VIEW "filtered_blogs" AS
            SELECT id, author_id FROM "Blog" WHERE id < 3;
        "#};

        api.execute_sql(create_view).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.views.filtered_blogs.columns.id]
        unique = true
        nullable = false

        [schemas.public.tables.User.relations.users_to_filtered_blogs]
        referenced_table = "Blog"
        referencing_columns = ["id"]
        referenced_columns = ["author_id"]
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
      id: Int!
      """
      Set field value for authorId
      """
      authorId: Int!
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
      Filter by the given authorId
      """
      authorId: IntFilterInput
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
      Order blogs by authorId
      """
      authorId: OrderDirection
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
      Update field value for authorId
      """
      authorId: IntUpdateInput
    }

    """
    Filter input type for FilteredBlog objects.
    """
    input FilteredBlogFilterInput @oneOf {
      """
      Filter by the given id
      """
      id: IntFilterInput
      """
      Filter by the given authorId
      """
      authorId: IntFilterInput
      """
      All of the filters must match
      """
      ALL: [FilteredBlogFilterInput]
      """
      None of the filters must match
      """
      NONE: [FilteredBlogFilterInput]
      """
      At least one of the filters must match
      """
      ANY: [FilteredBlogFilterInput]
    }

    """
    Input type to select a unique FilteredBlog
    """
    input FilteredBlogLookupInput @oneOf {
      """
      Select by the 'id' field
      """
      id: Int
    }

    """
    Lookup input type for FilteredBlog objects for subgraph joins.
    """
    input FilteredBlogManyLookupInput @oneOf @inaccessible {
      """
      Select by the 'id' field
      """
      id: [Int!] @inaccessible
    }

    """
    Specifies the ordering for FilteredBlog results.
    """
    input FilteredBlogOrderByInput @oneOf {
      """
      Order filteredBlogs by id
      """
      id: OrderDirection
      """
      Order filteredBlogs by authorId
      """
      authorId: OrderDirection
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

    type Blog
      @key(fields: "id")
      @pgTable(name: "Blog")
      @pgKey(fields: ["id"], type: UNIQUE)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      authorId: Int! @pgColumn(name: "author_id", type: INT)
      user: User @pgRelation(name: "users_to_filtered_blogs")
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
      The value of the authorId field
      """
      authorId: Int!
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

    type FilteredBlog
      @key(fields: "id")
      @pgTable(name: "filtered_blogs", kind: VIEW)
      @pgKey(fields: ["id"], type: UNIQUE)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      authorId: Int @pgColumn(name: "author_id", type: INT)
    }

    """
    The connection type for FilteredBlog
    """
    type FilteredBlogConnection
      @pgConnection(type: "FilteredBlog")
    {
      """
      A list of edges
      """
      edges: [FilteredBlogEdge!]! @shareable
      """
      Information to aid in pagination
      """
      pageInfo: PageInfo! @shareable
    }

    """
    An edge in a connection. Contains the node and its cursor
    """
    type FilteredBlogEdge {
      """
      The item at the end of the edge
      """
      node: FilteredBlog! @shareable
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
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: UNIQUE)
    {
      id: Int! @pgColumn(name: "id", type: INT)
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
      ): BlogConnection! @pgRelation(name: "users_to_filtered_blogs", fields: ["id"], references: ["authorId"])
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
      ): [Blog] @pgLookup @lookup @inaccessible
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
      ): [User] @pgLookup @lookup @inaccessible
      """
      Query a unique FilteredBlog
      """
      filteredBlog(
        """
        Input for unique FilteredBlog lookup
        """
        lookup: FilteredBlogLookupInput!,
      ): FilteredBlog @pgSelectOne
      """
      Query and paginate multiple filteredBlogs
      """
      filteredBlogs(
        """
        Filter for FilteredBlog
        """
        filter: FilteredBlogFilterInput,
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
        orderBy: [FilteredBlogOrderByInput!],
      ): FilteredBlogConnection! @pgSelectMany
      """
      Lookup multiple filteredBlogs for subgraph joins
      """
      filteredBlogLookup(
        """
        Filter filteredBlogs with an array of keys
        """
        lookup: FilteredBlogManyLookupInput @inaccessible,
      ): [FilteredBlog] @pgLookup @lookup @inaccessible
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
