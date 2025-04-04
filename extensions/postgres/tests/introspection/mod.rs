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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by id
      id: OrderDirection
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput

    # Input for updating an existing User
    input UserUpdateInput

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the id field
      id: Int!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection
      @pgConnection
    {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type User
      @pgTable(name: "users")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by id
      id: OrderDirection
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput {
      # Set field value for id
      id: Int
    }

    # Input for updating an existing User
    input UserUpdateInput {
      # Set field value for id
      id: Int
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the id field
      id: Int!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection
      @pgConnection
    {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type User
      @pgTable(name: "users")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by id
      id: OrderDirection
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput {
      # Set field value for id
      id: Int
    }

    # Input for updating an existing User
    input UserUpdateInput {
      # Set field value for id
      id: Int
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the id field
      id: Int!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type User
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    enum StreetLight @pgEnum(name: "street_light") {
      RED @pgEnumVariant(name: "red")
      YELLOW @pgEnumVariant(name: "yellow")
      GREEN @pgEnumVariant(name: "green")
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Search filter input for StreetLight type.
    input StreetLightFilterInput @oneOf {
      # The value is exactly the one given
      eq: StreetLight
      # The value is not the one given
      ne: StreetLight
      # The value is greater than the one given
      gt: StreetLight
      # The value is less than the one given
      lt: StreetLight
      # The value is greater than, or equal to the one given
      gte: StreetLight
      # The value is less than, or equal to the one given
      lte: StreetLight
      # The value is in the given array of values
      in: [StreetLight]
      # The value is not in the given array of values
      nin: [StreetLight]
      # A negation of the given filter
      not: StreetLightFilterInput
    }

    # Specifies the ordering for A results.
    input AOrderByInput @oneOf {
      # Order as by id
      id: OrderDirection
      # Order as by val
      val: OrderDirection
    }

    # Input type to select a unique A
    input ALookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for A collections
    input ACollectionFilterInput {
      # The object is related to an object with the given fields
      contains: AFilterInput
    }

    # Filter input type for A objects.
    input AFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # Filter by the given val
      val: StreetLightFilterInput
      # All of the filters must match
      ALL: [AFilterInput]
      # None of the filters must match
      NONE: [AFilterInput]
      # At least one of the filters must match
      ANY: [AFilterInput]
    }

    # Input for creating a new A
    input ACreateInput {
      # Set field value for id
      id: Int!
      # Set field value for val
      val: StreetLight!
    }

    # Input for updating an existing A
    input AUpdateInput {
      # Set field value for id
      id: Int
      # Set field value for val
      val: StreetLight
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created A object
    type AReturning {
      # The value of the id field
      id: Int!
      # The value of the val field
      val: StreetLight!
    }

    # Return type when creating one A
    type ACreatePayload {
      # Returned item(s) from the mutation
      returning: AReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many as
    type ACreateManyPayload {
      # Returned item(s) from the mutation
      returning: [AReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one A
    type AUpdatePayload {
      # Returned item(s) from the mutation
      returning: AReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many as
    type AUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [AReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one A
    type ADeletePayload {
      # Returned item(s) from the mutation
      returning: AReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many as
    type ADeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [AReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type AEdge {
      # The item at the end of the edge
      node: A!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for A
    type AConnection
      @pgConnection
    {
      # A list of edges
      edges: [AEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type A
      @pgTable(name: "A")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      val: StreetLight! @pgColumn(name: "val", type: ENUM)
    }

    type Query {
      # Query a unique A
      a(lookup: ALookupInput!): A @pgSelectOne
      # Query and paginate multiple as
      as(filter: AFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [AOrderByInput!]): AConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single A
      aCreate(input: ACreateInput!): ACreatePayload! @pgInsertOne
      # Create multiple as
      aCreateMany(input: [ACreateInput!]!): ACreateManyPayload! @pgInsertMany
      # Update a unique A
      aUpdate(lookup: ALookupInput!, input: AUpdateInput!): AUpdatePayload! @pgUpdateOne
      # Update multiple as
      aUpdateMany(filter: AFilterInput, input: AUpdateInput!): AUpdateManyPayload! @pgUpdateMany
      # Delete a unique A
      aDelete(lookup: ALookupInput!): ADeletePayload! @pgDeleteOne
      # Delete multiple as
      aDeleteMany(filter: AFilterInput): ADeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by id
      id: OrderDirection
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput {
      # Set field value for id
      id: Int!
    }

    # Input for updating an existing User
    input UserUpdateInput {
      # Set field value for id
      id: Int
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the id field
      id: Int!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection
      @pgConnection
    {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type User
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by id
      id: OrderDirection
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput {
      # Set field value for id
      id: Int!
    }

    # Input for updating an existing User
    input UserUpdateInput {
      # Set field value for id
      id: Int
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the id field
      id: Int!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection
      @pgConnection
    {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type User
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: UNIQUE)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by id
      id: OrderDirection
      # Order users by email
      email: OrderDirection
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select by the 'email' field
      email: String
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # Filter by the given email
      email: StringFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput {
      # Set field value for id
      id: Int
      # Set field value for email
      email: String!
    }

    # Input for updating an existing User
    input UserUpdateInput {
      # Set field value for id
      id: Int
      # Set field value for email
      email: String
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the id field
      id: Int!
      # The value of the email field
      email: String!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection
      @pgConnection
    {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type User
      @pgTable(name: "User")
      @pgKey(fields: ["email"], type: UNIQUE)
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      email: String! @pgColumn(name: "email", type: VARCHAR)
    }

    type Query {
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by name
      name: OrderDirection
      # Order users by email
      email: OrderDirection
    }

    # Input type to select a unique User with multiple fields
    input UserNameEmailInput {
      # Select by the 'name' field
      name: String!
      # Select by the 'email' field
      email: String!
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select User by composite columns 'name, email'
      nameEmail: UserNameEmailInput
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given name
      name: StringFilterInput
      # Filter by the given email
      email: StringFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput {
      # Set field value for name
      name: String!
      # Set field value for email
      email: String!
    }

    # Input for updating an existing User
    input UserUpdateInput {
      # Set field value for name
      name: String
      # Set field value for email
      email: String
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the name field
      name: String!
      # The value of the email field
      email: String!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection
      @pgConnection
    {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type User
      @pgTable(name: "User")
      @pgKey(fields: ["name", "email"], type: PRIMARY)
    {
      name: String! @pgColumn(name: "name", type: VARCHAR)
      email: String! @pgColumn(name: "email", type: VARCHAR)
    }

    type Query {
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for PrivateUser results.
    input PrivateUserOrderByInput @oneOf {
      # Order privateUsers by id
      id: OrderDirection
    }

    # Input type to select a unique PrivateUser
    input PrivateUserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for PrivateUser collections
    input PrivateUserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: PrivateUserFilterInput
    }

    # Filter input type for PrivateUser objects.
    input PrivateUserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # All of the filters must match
      ALL: [PrivateUserFilterInput]
      # None of the filters must match
      NONE: [PrivateUserFilterInput]
      # At least one of the filters must match
      ANY: [PrivateUserFilterInput]
    }

    # Input for creating a new PrivateUser
    input PrivateUserCreateInput {
      # Set field value for id
      id: Int
    }

    # Input for updating an existing PrivateUser
    input PrivateUserUpdateInput {
      # Set field value for id
      id: Int
    }

    # Specifies the ordering for PublicUser results.
    input PublicUserOrderByInput @oneOf {
      # Order publicUsers by id
      id: OrderDirection
    }

    # Input type to select a unique PublicUser
    input PublicUserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for PublicUser collections
    input PublicUserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: PublicUserFilterInput
    }

    # Filter input type for PublicUser objects.
    input PublicUserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # All of the filters must match
      ALL: [PublicUserFilterInput]
      # None of the filters must match
      NONE: [PublicUserFilterInput]
      # At least one of the filters must match
      ANY: [PublicUserFilterInput]
    }

    # Input for creating a new PublicUser
    input PublicUserCreateInput {
      # Set field value for id
      id: Int
    }

    # Input for updating an existing PublicUser
    input PublicUserUpdateInput {
      # Set field value for id
      id: Int
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created PrivateUser object
    type PrivateUserReturning {
      # The value of the id field
      id: Int!
    }

    # Return type when creating one PrivateUser
    type PrivateUserCreatePayload {
      # Returned item(s) from the mutation
      returning: PrivateUserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many privateUsers
    type PrivateUserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [PrivateUserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one PrivateUser
    type PrivateUserUpdatePayload {
      # Returned item(s) from the mutation
      returning: PrivateUserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many privateUsers
    type PrivateUserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [PrivateUserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one PrivateUser
    type PrivateUserDeletePayload {
      # Returned item(s) from the mutation
      returning: PrivateUserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many privateUsers
    type PrivateUserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [PrivateUserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type PrivateUserEdge {
      # The item at the end of the edge
      node: PrivateUser!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for PrivateUser
    type PrivateUserConnection
      @pgConnection
    {
      # A list of edges
      edges: [PrivateUserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    # Return type containing fields of the mutated or created PublicUser object
    type PublicUserReturning {
      # The value of the id field
      id: Int!
    }

    # Return type when creating one PublicUser
    type PublicUserCreatePayload {
      # Returned item(s) from the mutation
      returning: PublicUserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many publicUsers
    type PublicUserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [PublicUserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one PublicUser
    type PublicUserUpdatePayload {
      # Returned item(s) from the mutation
      returning: PublicUserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many publicUsers
    type PublicUserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [PublicUserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one PublicUser
    type PublicUserDeletePayload {
      # Returned item(s) from the mutation
      returning: PublicUserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many publicUsers
    type PublicUserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [PublicUserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type PublicUserEdge {
      # The item at the end of the edge
      node: PublicUser!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for PublicUser
    type PublicUserConnection
      @pgConnection
    {
      # A list of edges
      edges: [PublicUserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type PrivateUser
      @pgTable(name: "User", schema: "private")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type PublicUser
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
    }

    type Query {
      # Query a unique PrivateUser
      privateUser(lookup: PrivateUserLookupInput!): PrivateUser @pgSelectOne
      # Query and paginate multiple privateUsers
      privateUsers(filter: PrivateUserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [PrivateUserOrderByInput!]): PrivateUserConnection! @pgSelectMany
      # Query a unique PublicUser
      publicUser(lookup: PublicUserLookupInput!): PublicUser @pgSelectOne
      # Query and paginate multiple publicUsers
      publicUsers(filter: PublicUserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [PublicUserOrderByInput!]): PublicUserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single PrivateUser
      privateUserCreate(input: PrivateUserCreateInput!): PrivateUserCreatePayload! @pgInsertOne
      # Create multiple privateUsers
      privateUserCreateMany(input: [PrivateUserCreateInput!]!): PrivateUserCreateManyPayload! @pgInsertMany
      # Update a unique PrivateUser
      privateUserUpdate(lookup: PrivateUserLookupInput!, input: PrivateUserUpdateInput!): PrivateUserUpdatePayload! @pgUpdateOne
      # Update multiple privateUsers
      privateUserUpdateMany(filter: PrivateUserFilterInput, input: PrivateUserUpdateInput!): PrivateUserUpdateManyPayload! @pgUpdateMany
      # Delete a unique PrivateUser
      privateUserDelete(lookup: PrivateUserLookupInput!): PrivateUserDeletePayload! @pgDeleteOne
      # Delete multiple privateUsers
      privateUserDeleteMany(filter: PrivateUserFilterInput): PrivateUserDeleteManyPayload! @pgDeleteMany
      # Create a single PublicUser
      publicUserCreate(input: PublicUserCreateInput!): PublicUserCreatePayload! @pgInsertOne
      # Create multiple publicUsers
      publicUserCreateMany(input: [PublicUserCreateInput!]!): PublicUserCreateManyPayload! @pgInsertMany
      # Update a unique PublicUser
      publicUserUpdate(lookup: PublicUserLookupInput!, input: PublicUserUpdateInput!): PublicUserUpdatePayload! @pgUpdateOne
      # Update multiple publicUsers
      publicUserUpdateMany(filter: PublicUserFilterInput, input: PublicUserUpdateInput!): PublicUserUpdateManyPayload! @pgUpdateMany
      # Delete a unique PublicUser
      publicUserDelete(lookup: PublicUserLookupInput!): PublicUserDeletePayload! @pgDeleteOne
      # Delete multiple publicUsers
      publicUserDeleteMany(filter: PublicUserFilterInput): PublicUserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by id
      id: OrderDirection
      # Order users by name
      name: OrderDirection
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # Filter by the given name
      name: IntArrayFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput {
      # Set field value for id
      id: Int
      # Set field value for name
      name: [Int]!
    }

    # Input for updating an existing User
    input UserUpdateInput {
      # Set field value for id
      id: Int
      # Set field value for name
      name: [Int]
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the id field
      id: Int!
      # The value of the name field
      name: [Int]!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection
      @pgConnection
    {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type User
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      name: [Int]! @pgColumn(name: "name", type: INT)
    }

    type Query {
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by id
      id: OrderDirection
      # Order users by name
      name: OrderDirection
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # Filter by the given name
      name: JSONFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput {
      # Set field value for id
      id: Int
      # Set field value for name
      name: JSON!
    }

    # Input for updating an existing User
    input UserUpdateInput {
      # Set field value for id
      id: Int
      # Set field value for name
      name: JSON
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the id field
      id: Int!
      # The value of the name field
      name: JSON!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection
      @pgConnection
    {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type User
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      name: JSON! @pgColumn(name: "name", type: JSONB)
    }

    type Query {
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by id
      id: OrderDirection
      # Order users by name
      name: OrderDirection
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # Filter by the given name
      name: JSONFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput {
      # Set field value for id
      id: Int
      # Set field value for name
      name: JSON!
    }

    # Input for updating an existing User
    input UserUpdateInput {
      # Set field value for id
      id: Int
      # Set field value for name
      name: JSON
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the id field
      id: Int!
      # The value of the name field
      name: JSON!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection
      @pgConnection
    {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type User
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      name: JSON! @pgColumn(name: "name", type: JSON)
    }

    type Query {
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for Blog results.
    input BlogOrderByInput @oneOf {
      # Order blogs by id
      id: OrderDirection
      # Order blogs by title
      title: OrderDirection
      # Order blogs by content
      content: OrderDirection
      # Order blogs by userId
      userId: OrderDirection
      # Order Blog results by User fields
      user: UserOrderByInput
    }

    # Input type to select a unique Blog
    input BlogLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for Blog collections
    input BlogCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: BlogFilterInput
    }

    # Filter input type for Blog objects.
    input BlogFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # Filter by the given title
      title: StringFilterInput
      # Filter by the given content
      content: StringFilterInput
      # Filter by the given userId
      userId: IntFilterInput
      # Filter by the related User object
      user: UserFilterInput
      # All of the filters must match
      ALL: [BlogFilterInput]
      # None of the filters must match
      NONE: [BlogFilterInput]
      # At least one of the filters must match
      ANY: [BlogFilterInput]
    }

    # Input for creating a new Blog
    input BlogCreateInput {
      # Set field value for id
      id: Int
      # Set field value for title
      title: String!
      # Set field value for content
      content: String
      # Set field value for userId
      userId: Int!
    }

    # Input for updating an existing Blog
    input BlogUpdateInput {
      # Set field value for id
      id: Int
      # Set field value for title
      title: String
      # Set field value for content
      content: String
      # Set field value for userId
      userId: Int
    }

    # Specifies the ordering for User results.
    input UserOrderByInput @oneOf {
      # Order users by id
      id: OrderDirection
      # Order users by name
      name: OrderDirection
    }

    # Input type to select a unique User
    input UserLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for User collections
    input UserCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: UserFilterInput
    }

    # Filter input type for User objects.
    input UserFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # Filter by the given name
      name: StringFilterInput
      # Filter by the related Blog objects
      blogs: BlogCollectionFilterInput
      # All of the filters must match
      ALL: [UserFilterInput]
      # None of the filters must match
      NONE: [UserFilterInput]
      # At least one of the filters must match
      ANY: [UserFilterInput]
    }

    # Input for creating a new User
    input UserCreateInput {
      # Set field value for id
      id: Int
      # Set field value for name
      name: String!
    }

    # Input for updating an existing User
    input UserUpdateInput {
      # Set field value for id
      id: Int
      # Set field value for name
      name: String
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created Blog object
    type BlogReturning {
      # The value of the id field
      id: Int!
      # The value of the title field
      title: String!
      # The value of the content field
      content: String
      # The value of the userId field
      userId: Int!
    }

    # Return type when creating one Blog
    type BlogCreatePayload {
      # Returned item(s) from the mutation
      returning: BlogReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many blogs
    type BlogCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [BlogReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one Blog
    type BlogUpdatePayload {
      # Returned item(s) from the mutation
      returning: BlogReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many blogs
    type BlogUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [BlogReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one Blog
    type BlogDeletePayload {
      # Returned item(s) from the mutation
      returning: BlogReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many blogs
    type BlogDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [BlogReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type BlogEdge {
      # The item at the end of the edge
      node: Blog!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for Blog
    type BlogConnection
      @pgConnection
    {
      # A list of edges
      edges: [BlogEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    # Return type containing fields of the mutated or created User object
    type UserReturning {
      # The value of the id field
      id: Int!
      # The value of the name field
      name: String!
    }

    # Return type when creating one User
    type UserCreatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many users
    type UserCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one User
    type UserUpdatePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many users
    type UserUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one User
    type UserDeletePayload {
      # Returned item(s) from the mutation
      returning: UserReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many users
    type UserDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [UserReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type UserEdge {
      # The item at the end of the edge
      node: User!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for User
    type UserConnection
      @pgConnection
    {
      # A list of edges
      edges: [UserEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type Blog
      @pgTable(name: "Blog")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      title: String! @pgColumn(name: "title", type: VARCHAR)
      content: String @pgColumn(name: "content", type: TEXT)
      userId: Int! @pgColumn(name: "user_id", type: INT)
      user: User @pgRelation(name: "Blog_User", fields: ["userId"], references: ["id"])
    }

    type User
      @pgTable(name: "User")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      name: String! @pgColumn(name: "name", type: VARCHAR)
      blogs(filter: BlogFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [BlogOrderByInput!]): BlogConnection! @pgRelation(name: "Blog_User")
    }

    type Query {
      # Query a unique Blog
      blog(lookup: BlogLookupInput!): Blog @pgSelectOne
      # Query and paginate multiple blogs
      blogs(filter: BlogFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [BlogOrderByInput!]): BlogConnection! @pgSelectMany
      # Query a unique User
      user(lookup: UserLookupInput!): User @pgSelectOne
      # Query and paginate multiple users
      users(filter: UserFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [UserOrderByInput!]): UserConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single Blog
      blogCreate(input: BlogCreateInput!): BlogCreatePayload! @pgInsertOne
      # Create multiple blogs
      blogCreateMany(input: [BlogCreateInput!]!): BlogCreateManyPayload! @pgInsertMany
      # Update a unique Blog
      blogUpdate(lookup: BlogLookupInput!, input: BlogUpdateInput!): BlogUpdatePayload! @pgUpdateOne
      # Update multiple blogs
      blogUpdateMany(filter: BlogFilterInput, input: BlogUpdateInput!): BlogUpdateManyPayload! @pgUpdateMany
      # Delete a unique Blog
      blogDelete(lookup: BlogLookupInput!): BlogDeletePayload! @pgDeleteOne
      # Delete multiple blogs
      blogDeleteMany(filter: BlogFilterInput): BlogDeleteManyPayload! @pgDeleteMany
      # Create a single User
      userCreate(input: UserCreateInput!): UserCreatePayload! @pgInsertOne
      # Create multiple users
      userCreateMany(input: [UserCreateInput!]!): UserCreateManyPayload! @pgInsertMany
      # Update a unique User
      userUpdate(lookup: UserLookupInput!, input: UserUpdateInput!): UserUpdatePayload! @pgUpdateOne
      # Update multiple users
      userUpdateMany(filter: UserFilterInput, input: UserUpdateInput!): UserUpdateManyPayload! @pgUpdateMany
      # Delete a unique User
      userDelete(lookup: UserLookupInput!): UserDeletePayload! @pgDeleteOne
      # Delete multiple users
      userDeleteMany(filter: UserFilterInput): UserDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Specifies the ordering for VisibleTable results.
    input VisibleTableOrderByInput @oneOf {
      # Order visibleTables by id
      id: OrderDirection
    }

    # Input type to select a unique VisibleTable
    input VisibleTableLookupInput @oneOf {
      # Select by the 'id' field
      id: String
    }

    # Filter input type for VisibleTable collections
    input VisibleTableCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: VisibleTableFilterInput
    }

    # Filter input type for VisibleTable objects.
    input VisibleTableFilterInput @oneOf {
      # Filter by the given id
      id: StringFilterInput
      # All of the filters must match
      ALL: [VisibleTableFilterInput]
      # None of the filters must match
      NONE: [VisibleTableFilterInput]
      # At least one of the filters must match
      ANY: [VisibleTableFilterInput]
    }

    # Input for creating a new VisibleTable
    input VisibleTableCreateInput {
      # Set field value for id
      id: String!
    }

    # Input for updating an existing VisibleTable
    input VisibleTableUpdateInput {
      # Set field value for id
      id: String
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created VisibleTable object
    type VisibleTableReturning {
      # The value of the id field
      id: String!
    }

    # Return type when creating one VisibleTable
    type VisibleTableCreatePayload {
      # Returned item(s) from the mutation
      returning: VisibleTableReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many visibleTables
    type VisibleTableCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [VisibleTableReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one VisibleTable
    type VisibleTableUpdatePayload {
      # Returned item(s) from the mutation
      returning: VisibleTableReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many visibleTables
    type VisibleTableUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [VisibleTableReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one VisibleTable
    type VisibleTableDeletePayload {
      # Returned item(s) from the mutation
      returning: VisibleTableReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many visibleTables
    type VisibleTableDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [VisibleTableReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type VisibleTableEdge {
      # The item at the end of the edge
      node: VisibleTable!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for VisibleTable
    type VisibleTableConnection
      @pgConnection
    {
      # A list of edges
      edges: [VisibleTableEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type VisibleTable
      @pgTable(name: "visible_table")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: String! @pgColumn(name: "id", type: TEXT)
    }

    type Query {
      # Query a unique VisibleTable
      visibleTable(lookup: VisibleTableLookupInput!): VisibleTable @pgSelectOne
      # Query and paginate multiple visibleTables
      visibleTables(filter: VisibleTableFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [VisibleTableOrderByInput!]): VisibleTableConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single VisibleTable
      visibleTableCreate(input: VisibleTableCreateInput!): VisibleTableCreatePayload! @pgInsertOne
      # Create multiple visibleTables
      visibleTableCreateMany(input: [VisibleTableCreateInput!]!): VisibleTableCreateManyPayload! @pgInsertMany
      # Update a unique VisibleTable
      visibleTableUpdate(lookup: VisibleTableLookupInput!, input: VisibleTableUpdateInput!): VisibleTableUpdatePayload! @pgUpdateOne
      # Update multiple visibleTables
      visibleTableUpdateMany(filter: VisibleTableFilterInput, input: VisibleTableUpdateInput!): VisibleTableUpdateManyPayload! @pgUpdateMany
      # Delete a unique VisibleTable
      visibleTableDelete(lookup: VisibleTableLookupInput!): VisibleTableDeletePayload! @pgDeleteOne
      # Delete multiple visibleTables
      visibleTableDeleteMany(filter: VisibleTableFilterInput): VisibleTableDeleteManyPayload! @pgDeleteMany
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
        url: "https://grafbase.com/extensions/postgres/0.1.0",
        import: [
          "@pgDatabase",
          "@pgTable",
          "@pgColumn",
          "@pgEnum",
          "@pgEnumVariant",
          "@pgRelation",
          "@pgKey",
          "@pgSelectOne",
          "@pgSelectMany",
          "@pgInsertOne",
          "@pgInsertMany",
          "@pgUpdateOne",
          "@pgUpdateMany",
          "@pgDeleteOne",
          "@pgDeleteMany",
          "@pgConnection",
          "PgKeyType",
          "PgColumnType"
        ]
      )
      @pgDatabase(name: "default")

    scalar JSON

    # Specifies the direction for ordering results.
    enum OrderDirection {
      # Specifies an ascending order for a given orderBy argument.
      ASC
      # Specifies a descending order for a given orderBy argument.
      DESC
    }

    enum AccessMode @pgEnum(name: "access_mode") {
      PUBLIC @pgEnumVariant(name: "PUBLIC")
      PUBLIC_READ @pgEnumVariant(name: "PUBLIC_READ")
      PRIVATE @pgEnumVariant(name: "PRIVATE")
    }

    enum ProjectStatus @pgEnum(name: "project_status") {
      CREATED @pgEnumVariant(name: "CREATED")
      READY @pgEnumVariant(name: "READY")
      FAILED @pgEnumVariant(name: "FAILED")
    }

    # Search filter input for String type.
    input StringFilterInput @oneOf {
      # The value is exactly the one given
      eq: String
      # The value is not the one given
      ne: String
      # The value is greater than the one given
      gt: String
      # The value is less than the one given
      lt: String
      # The value is greater than, or equal to the one given
      gte: String
      # The value is less than, or equal to the one given
      lte: String
      # The given input is part of the column value
      like: String
      # The value is in the given array of values
      in: [String]
      # The value is not in the given array of values
      nin: [String]
      # A negation of the given filter
      not: StringFilterInput
    }

    # Search filter input for Int type.
    input IntFilterInput @oneOf {
      # The value is exactly the one given
      eq: Int
      # The value is not the one given
      ne: Int
      # The value is greater than the one given
      gt: Int
      # The value is less than the one given
      lt: Int
      # The value is greater than, or equal to the one given
      gte: Int
      # The value is less than, or equal to the one given
      lte: Int
      # The value is in the given array of values
      in: [Int]
      # The value is not in the given array of values
      nin: [Int]
      # A negation of the given filter
      not: IntFilterInput
    }

    # Search filter input for Float type.
    input FloatFilterInput @oneOf {
      # The value is exactly the one given
      eq: Float
      # The value is not the one given
      ne: Float
      # The value is greater than the one given
      gt: Float
      # The value is less than the one given
      lt: Float
      # The value is greater than, or equal to the one given
      gte: Float
      # The value is less than, or equal to the one given
      lte: Float
      # The value is in the given array of values
      in: [Float]
      # The value is not in the given array of values
      nin: [Float]
      # A negation of the given filter
      not: FloatFilterInput
    }

    # Search filter input for Boolean type.
    input BooleanFilterInput @oneOf {
      # The value is exactly the one given
      eq: Boolean
      # The value is not the one given
      ne: Boolean
      # The value is greater than the one given
      gt: Boolean
      # The value is less than the one given
      lt: Boolean
      # The value is greater than, or equal to the one given
      gte: Boolean
      # The value is less than, or equal to the one given
      lte: Boolean
      # The value is in the given array of values
      in: [Boolean]
      # The value is not in the given array of values
      nin: [Boolean]
      # A negation of the given filter
      not: BooleanFilterInput
    }

    # Search filter input for JSON type.
    input JSONFilterInput @oneOf {
      # The value is exactly the one given
      eq: JSON
      # The value is not the one given
      ne: JSON
      # The value is greater than the one given
      gt: JSON
      # The value is less than the one given
      lt: JSON
      # The value is greater than, or equal to the one given
      gte: JSON
      # The value is less than, or equal to the one given
      lte: JSON
      # The value is in the given array of values
      in: [JSON]
      # The value is not in the given array of values
      nin: [JSON]
      # A negation of the given filter
      not: JSONFilterInput
    }

    # Search filter input for String array type.
    input StringArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [String]
      # The value is not the one given
      ne: [String]
      # The value is greater than the one given
      gt: [String]
      # The value is less than the one given
      lt: [String]
      # The value is greater than, or equal to the one given
      gte: [String]
      # The value is less than, or equal to the one given
      lte: [String]
      # The value is in the given array of values
      in: [[String]]
      # The value is not in the given array of values
      nin: [[String]]
      # Checks if the array contains all elements of the provided array
      contains: [String]
      # Checks if the array is contained within the provided array
      contained: [String]
      # Checks if the array has any elements in common with the provided array
      overlaps: [String]
      # A negation of the given filter
      not: StringArrayFilterInput
    }

    # Search filter input for Int array type.
    input IntArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Int]
      # The value is not the one given
      ne: [Int]
      # The value is greater than the one given
      gt: [Int]
      # The value is less than the one given
      lt: [Int]
      # The value is greater than, or equal to the one given
      gte: [Int]
      # The value is less than, or equal to the one given
      lte: [Int]
      # The value is in the given array of values
      in: [[Int]]
      # The value is not in the given array of values
      nin: [[Int]]
      # Checks if the array contains all elements of the provided array
      contains: [Int]
      # Checks if the array is contained within the provided array
      contained: [Int]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Int]
      # A negation of the given filter
      not: IntArrayFilterInput
    }

    # Search filter input for Float array type.
    input FloatArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Float]
      # The value is not the one given
      ne: [Float]
      # The value is greater than the one given
      gt: [Float]
      # The value is less than the one given
      lt: [Float]
      # The value is greater than, or equal to the one given
      gte: [Float]
      # The value is less than, or equal to the one given
      lte: [Float]
      # The value is in the given array of values
      in: [[Float]]
      # The value is not in the given array of values
      nin: [[Float]]
      # Checks if the array contains all elements of the provided array
      contains: [Float]
      # Checks if the array is contained within the provided array
      contained: [Float]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Float]
      # A negation of the given filter
      not: FloatArrayFilterInput
    }

    # Search filter input for Boolean array type.
    input BooleanArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [Boolean]
      # The value is not the one given
      ne: [Boolean]
      # The value is greater than the one given
      gt: [Boolean]
      # The value is less than the one given
      lt: [Boolean]
      # The value is greater than, or equal to the one given
      gte: [Boolean]
      # The value is less than, or equal to the one given
      lte: [Boolean]
      # The value is in the given array of values
      in: [[Boolean]]
      # The value is not in the given array of values
      nin: [[Boolean]]
      # Checks if the array contains all elements of the provided array
      contains: [Boolean]
      # Checks if the array is contained within the provided array
      contained: [Boolean]
      # Checks if the array has any elements in common with the provided array
      overlaps: [Boolean]
      # A negation of the given filter
      not: BooleanArrayFilterInput
    }

    # Search filter input for JSON array type.
    input JSONArrayFilterInput @oneOf {
      # The value is exactly the one given
      eq: [JSON]
      # The value is not the one given
      ne: [JSON]
      # The value is greater than the one given
      gt: [JSON]
      # The value is less than the one given
      lt: [JSON]
      # The value is greater than, or equal to the one given
      gte: [JSON]
      # The value is less than, or equal to the one given
      lte: [JSON]
      # The value is in the given array of values
      in: [[JSON]]
      # The value is not in the given array of values
      nin: [[JSON]]
      # Checks if the array contains all elements of the provided array
      contains: [JSON]
      # Checks if the array is contained within the provided array
      contained: [JSON]
      # Checks if the array has any elements in common with the provided array
      overlaps: [JSON]
      # A negation of the given filter
      not: JSONArrayFilterInput
    }

    # Search filter input for AccessMode type.
    input AccessModeFilterInput @oneOf {
      # The value is exactly the one given
      eq: AccessMode
      # The value is not the one given
      ne: AccessMode
      # The value is greater than the one given
      gt: AccessMode
      # The value is less than the one given
      lt: AccessMode
      # The value is greater than, or equal to the one given
      gte: AccessMode
      # The value is less than, or equal to the one given
      lte: AccessMode
      # The value is in the given array of values
      in: [AccessMode]
      # The value is not in the given array of values
      nin: [AccessMode]
      # A negation of the given filter
      not: AccessModeFilterInput
    }

    # Search filter input for ProjectStatus type.
    input ProjectStatusFilterInput @oneOf {
      # The value is exactly the one given
      eq: ProjectStatus
      # The value is not the one given
      ne: ProjectStatus
      # The value is greater than the one given
      gt: ProjectStatus
      # The value is less than the one given
      lt: ProjectStatus
      # The value is greater than, or equal to the one given
      gte: ProjectStatus
      # The value is less than, or equal to the one given
      lte: ProjectStatus
      # The value is in the given array of values
      in: [ProjectStatus]
      # The value is not in the given array of values
      nin: [ProjectStatus]
      # A negation of the given filter
      not: ProjectStatusFilterInput
    }

    # Specifies the ordering for Network results.
    input NetworkOrderByInput @oneOf {
      # Order networks by id
      id: OrderDirection
    }

    # Input type to select a unique Network
    input NetworkLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for Network collections
    input NetworkCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: NetworkFilterInput
    }

    # Filter input type for Network objects.
    input NetworkFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # Filter by the related Project objects
      projects: ProjectCollectionFilterInput
      # All of the filters must match
      ALL: [NetworkFilterInput]
      # None of the filters must match
      NONE: [NetworkFilterInput]
      # At least one of the filters must match
      ANY: [NetworkFilterInput]
    }

    # Input for creating a new Network
    input NetworkCreateInput {
      # Set field value for id
      id: Int
    }

    # Input for updating an existing Network
    input NetworkUpdateInput {
      # Set field value for id
      id: Int
    }

    # Specifies the ordering for Project results.
    input ProjectOrderByInput @oneOf {
      # Order projects by id
      id: OrderDirection
      # Order projects by accessMode
      accessMode: OrderDirection
      # Order projects by status
      status: OrderDirection
      # Order projects by networkId
      networkId: OrderDirection
      # Order Project results by Network fields
      network: NetworkOrderByInput
    }

    # Input type to select a unique Project
    input ProjectLookupInput @oneOf {
      # Select by the 'id' field
      id: Int
    }

    # Filter input type for Project collections
    input ProjectCollectionFilterInput {
      # The object is related to an object with the given fields
      contains: ProjectFilterInput
    }

    # Filter input type for Project objects.
    input ProjectFilterInput @oneOf {
      # Filter by the given id
      id: IntFilterInput
      # Filter by the given accessMode
      accessMode: AccessModeFilterInput
      # Filter by the given status
      status: ProjectStatusFilterInput
      # Filter by the given networkId
      networkId: IntFilterInput
      # Filter by the related Network object
      network: NetworkFilterInput
      # All of the filters must match
      ALL: [ProjectFilterInput]
      # None of the filters must match
      NONE: [ProjectFilterInput]
      # At least one of the filters must match
      ANY: [ProjectFilterInput]
    }

    # Input for creating a new Project
    input ProjectCreateInput {
      # Set field value for id
      id: Int
      # Set field value for accessMode
      accessMode: AccessMode!
      # Set field value for status
      status: ProjectStatus
      # Set field value for networkId
      networkId: Int
    }

    # Input for updating an existing Project
    input ProjectUpdateInput {
      # Set field value for id
      id: Int
      # Set field value for accessMode
      accessMode: AccessMode
      # Set field value for status
      status: ProjectStatus
      # Set field value for networkId
      networkId: Int
    }

    # Information about pagination in a collection of objects
    type PageInfo {
      # When paginating backwards, are there more items?
      hasPreviousPage: Boolean!
      # When paginating forwards, are there more items?
      hasNextPage: Boolean!
      # The cursor of the first item in the page
      startCursor: String!
      # The cursor of the last item in the page
      endCursor: String!
    }

    # Return type containing fields of the mutated or created Network object
    type NetworkReturning {
      # The value of the id field
      id: Int!
    }

    # Return type when creating one Network
    type NetworkCreatePayload {
      # Returned item(s) from the mutation
      returning: NetworkReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many networks
    type NetworkCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [NetworkReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one Network
    type NetworkUpdatePayload {
      # Returned item(s) from the mutation
      returning: NetworkReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many networks
    type NetworkUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [NetworkReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one Network
    type NetworkDeletePayload {
      # Returned item(s) from the mutation
      returning: NetworkReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many networks
    type NetworkDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [NetworkReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type NetworkEdge {
      # The item at the end of the edge
      node: Network!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for Network
    type NetworkConnection
      @pgConnection
    {
      # A list of edges
      edges: [NetworkEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    # Return type containing fields of the mutated or created Project object
    type ProjectReturning {
      # The value of the id field
      id: Int!
      # The value of the accessMode field
      accessMode: AccessMode!
      # The value of the status field
      status: ProjectStatus!
      # The value of the networkId field
      networkId: Int
    }

    # Return type when creating one Project
    type ProjectCreatePayload {
      # Returned item(s) from the mutation
      returning: ProjectReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when creating many projects
    type ProjectCreateManyPayload {
      # Returned item(s) from the mutation
      returning: [ProjectReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating one Project
    type ProjectUpdatePayload {
      # Returned item(s) from the mutation
      returning: ProjectReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when updating many projects
    type ProjectUpdateManyPayload {
      # Returned item(s) from the mutation
      returning: [ProjectReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting one Project
    type ProjectDeletePayload {
      # Returned item(s) from the mutation
      returning: ProjectReturning
      # The number of rows mutated
      rowCount: Int!
    }

    # Return type when deleting many projects
    type ProjectDeleteManyPayload {
      # Returned item(s) from the mutation
      returning: [ProjectReturning]!
      # The number of rows mutated
      rowCount: Int!
    }

    # An edge in a connection. Contains the node and its cursor
    type ProjectEdge {
      # The item at the end of the edge
      node: Project!
      # A cursor for use in pagination
      cursor: String!
    }

    # The connection type for Project
    type ProjectConnection
      @pgConnection
    {
      # A list of edges
      edges: [ProjectEdge!]!
      # Information to aid in pagination
      pageInfo: PageInfo!
    }

    type Network
      @pgTable(name: "networks")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      projects(filter: ProjectFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [ProjectOrderByInput!]): ProjectConnection! @pgRelation(name: "projects_network_id_fkey")
    }

    type Project
      @pgTable(name: "projects")
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      accessMode: AccessMode! @pgColumn(name: "access_mode", type: ENUM)
      status: ProjectStatus! @pgColumn(name: "status", type: ENUM)
      networkId: Int @pgColumn(name: "network_id", type: INT)
      network: Network @pgRelation(name: "projects_network_id_fkey", fields: ["networkId"], references: ["id"])
    }

    type Query {
      # Query a unique Network
      network(lookup: NetworkLookupInput!): Network @pgSelectOne
      # Query and paginate multiple networks
      networks(filter: NetworkFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [NetworkOrderByInput!]): NetworkConnection! @pgSelectMany
      # Query a unique Project
      project(lookup: ProjectLookupInput!): Project @pgSelectOne
      # Query and paginate multiple projects
      projects(filter: ProjectFilterInput, first: Int, last: Int, before: String, after: String, orderBy: [ProjectOrderByInput!]): ProjectConnection! @pgSelectMany
    }

    type Mutation {
      # Create a single Network
      networkCreate(input: NetworkCreateInput!): NetworkCreatePayload! @pgInsertOne
      # Create multiple networks
      networkCreateMany(input: [NetworkCreateInput!]!): NetworkCreateManyPayload! @pgInsertMany
      # Update a unique Network
      networkUpdate(lookup: NetworkLookupInput!, input: NetworkUpdateInput!): NetworkUpdatePayload! @pgUpdateOne
      # Update multiple networks
      networkUpdateMany(filter: NetworkFilterInput, input: NetworkUpdateInput!): NetworkUpdateManyPayload! @pgUpdateMany
      # Delete a unique Network
      networkDelete(lookup: NetworkLookupInput!): NetworkDeletePayload! @pgDeleteOne
      # Delete multiple networks
      networkDeleteMany(filter: NetworkFilterInput): NetworkDeleteManyPayload! @pgDeleteMany
      # Create a single Project
      projectCreate(input: ProjectCreateInput!): ProjectCreatePayload! @pgInsertOne
      # Create multiple projects
      projectCreateMany(input: [ProjectCreateInput!]!): ProjectCreateManyPayload! @pgInsertMany
      # Update a unique Project
      projectUpdate(lookup: ProjectLookupInput!, input: ProjectUpdateInput!): ProjectUpdatePayload! @pgUpdateOne
      # Update multiple projects
      projectUpdateMany(filter: ProjectFilterInput, input: ProjectUpdateInput!): ProjectUpdateManyPayload! @pgUpdateMany
      # Delete a unique Project
      projectDelete(lookup: ProjectLookupInput!): ProjectDeletePayload! @pgDeleteOne
      # Delete multiple projects
      projectDeleteMany(filter: ProjectFilterInput): ProjectDeleteManyPayload! @pgDeleteMany
    }
    "#);
}
