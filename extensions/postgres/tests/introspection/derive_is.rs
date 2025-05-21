use crate::PgTestApi;
use indoc::indoc;

#[tokio::test]
async fn one_column_required() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                author_id INT UNIQUE NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.tables.posts.derives.author]
        referenced_type = "User"
        fields = { authorId = "id" }
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
    Input for creating a new Post
    """
    input PostCreateInput {
      """
      Set field value for authorId
      """
      authorId: Int!
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
      Filter by the given authorId
      """
      authorId: IntFilterInput
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
      Select by the 'authorId' field
      """
      authorId: Int
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
      Select by the 'authorId' field
      """
      authorId: [Int!] @inaccessible
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
      """
      Order posts by authorId
      """
      authorId: OrderDirection
    }

    """
    Input for updating an existing Post
    """
    input PostUpdateInput {
      """
      Update field value for authorId
      """
      authorId: IntUpdateInput
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
      @key(fields: "authorId")
      @key(fields: "id")
      @pgTable(name: "posts")
      @pgKey(fields: ["authorId"], type: UNIQUE)
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      authorId: Int! @pgColumn(name: "author_id", type: INT)
      author: User! @derive @is(field: "{ authorId: id }")
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
      """
      The value of the authorId field
      """
      authorId: Int! @shareable
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
    {
      id: Int!
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
    }
    "#);
}

#[tokio::test]
async fn composite_required() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                author_email VARCHAR(25) NOT NULL,
                author_name VARCHAR(25) NOT NULL,
                UNIQUE (author_email, author_name)
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.tables.posts.derives.author]
        referenced_type = "User"
        fields = { authorEmail = "email", authorName = "name" }
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
    Input type to select a unique Post with multiple fields
    """
    input PostAuthorEmailAuthorNameInput {
      """
      Select by the 'authorEmail' field
      """
      authorEmail: String!
      """
      Select by the 'authorName' field
      """
      authorName: String!
    }

    """
    Input for creating a new Post
    """
    input PostCreateInput {
      """
      Set field value for authorEmail
      """
      authorEmail: String!
      """
      Set field value for authorName
      """
      authorName: String!
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
      Filter by the given authorEmail
      """
      authorEmail: StringFilterInput
      """
      Filter by the given authorName
      """
      authorName: StringFilterInput
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
      Select Post by composite columns 'authorEmail, authorName'
      """
      authorEmailAuthorName: PostAuthorEmailAuthorNameInput
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
      Select Post by composite columns 'authorEmail, authorName'
      """
      authorEmailAuthorName: [PostAuthorEmailAuthorNameInput!] @inaccessible
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
      """
      Order posts by authorEmail
      """
      authorEmail: OrderDirection
      """
      Order posts by authorName
      """
      authorName: OrderDirection
    }

    """
    Input for updating an existing Post
    """
    input PostUpdateInput {
      """
      Update field value for authorEmail
      """
      authorEmail: StringUpdateInput
      """
      Update field value for authorName
      """
      authorName: StringUpdateInput
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
      @key(fields: "authorEmail authorName")
      @key(fields: "id")
      @pgTable(name: "posts")
      @pgKey(fields: ["authorEmail", "authorName"], type: UNIQUE)
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      authorEmail: String! @pgColumn(name: "author_email", type: VARCHAR)
      authorName: String! @pgColumn(name: "author_name", type: VARCHAR)
      author: User! @derive @is(field: "{ authorEmail: email authorName: name }")
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
      """
      The value of the authorEmail field
      """
      authorEmail: String! @shareable
      """
      The value of the authorName field
      """
      authorName: String! @shareable
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
      @key(fields: "email name")
    {
      email: String!
      name: String!
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
    }
    "#);
}

#[tokio::test]
async fn type_already_defined() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                author_id INT UNIQUE NOT NULL
            )
        "#};

        api.execute_sql(schema).await;

        let schema = indoc! {r#"
            CREATE TABLE "users" (
                id INT PRIMARY KEY
            );
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.tables.posts.derives.author]
        referenced_type = "User"
        fields = { authorId = "id" }
    "#};

    let error = api.introspect_error(config).await;
    insta::assert_snapshot!(&error, @r#"The type User is already defined as a table."#);
}

#[tokio::test]
async fn wrong_table() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                author_id INT UNIQUE NOT NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.tables.cats.derives.author]
        referenced_type = "User"
        fields = { authorId = "id" }
    "#};

    let error = api.introspect_error(config).await;
    insta::assert_snapshot!(&error, @r#"Table `cats` not found in relation configuration."#);
}

#[tokio::test]
async fn only_column_nullable() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                author_id INT UNIQUE NULL
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.tables.posts.derives.author]
        referenced_type = "User"
        fields = { authorId = "id" }
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
    Input for creating a new Post
    """
    input PostCreateInput {
      """
      Set field value for authorId
      """
      authorId: Int
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
      Filter by the given authorId
      """
      authorId: IntFilterInput
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
      Select by the 'authorId' field
      """
      authorId: Int
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
      Select by the 'authorId' field
      """
      authorId: [Int!] @inaccessible
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
      """
      Order posts by authorId
      """
      authorId: OrderDirection
    }

    """
    Input for updating an existing Post
    """
    input PostUpdateInput {
      """
      Update field value for authorId
      """
      authorId: IntUpdateInput
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
      @key(fields: "authorId")
      @key(fields: "id")
      @pgTable(name: "posts")
      @pgKey(fields: ["authorId"], type: UNIQUE)
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      authorId: Int @pgColumn(name: "author_id", type: INT)
      author: User @derive @is(field: "{ authorId: id }")
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
      """
      The value of the authorId field
      """
      authorId: Int @shareable
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
    {
      id: Int
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
    }
    "#);
}

#[tokio::test]
async fn composite_one_nullable() {
    let api = PgTestApi::new("", |api| async move {
        let schema = indoc! {r#"
            CREATE TABLE "posts" (
                id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                author_email VARCHAR(25) NOT NULL,
                author_name VARCHAR(25) NULL,
                UNIQUE (author_email, author_name)
            )
        "#};

        api.execute_sql(schema).await;
    })
    .await;

    let config = indoc! {r#"
        extension_url = "https://grafbase.com/extensions/postgres/0.3.0"

        [schemas.public.tables.posts.derives.author]
        referenced_type = "User"
        fields = { authorEmail = "email", authorName = "name" }
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
    Input type to select a unique Post with multiple fields
    """
    input PostAuthorEmailAuthorNameInput {
      """
      Select by the 'authorEmail' field
      """
      authorEmail: String!
      """
      Select by the 'authorName' field
      """
      authorName: String
    }

    """
    Input for creating a new Post
    """
    input PostCreateInput {
      """
      Set field value for authorEmail
      """
      authorEmail: String!
      """
      Set field value for authorName
      """
      authorName: String
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
      Filter by the given authorEmail
      """
      authorEmail: StringFilterInput
      """
      Filter by the given authorName
      """
      authorName: StringFilterInput
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
      Select Post by composite columns 'authorEmail, authorName'
      """
      authorEmailAuthorName: PostAuthorEmailAuthorNameInput
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
      Select Post by composite columns 'authorEmail, authorName'
      """
      authorEmailAuthorName: [PostAuthorEmailAuthorNameInput!] @inaccessible
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
      """
      Order posts by authorEmail
      """
      authorEmail: OrderDirection
      """
      Order posts by authorName
      """
      authorName: OrderDirection
    }

    """
    Input for updating an existing Post
    """
    input PostUpdateInput {
      """
      Update field value for authorEmail
      """
      authorEmail: StringUpdateInput
      """
      Update field value for authorName
      """
      authorName: StringUpdateInput
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
      @key(fields: "authorEmail authorName")
      @key(fields: "id")
      @pgTable(name: "posts")
      @pgKey(fields: ["authorEmail", "authorName"], type: UNIQUE)
      @pgKey(fields: ["id"], type: PRIMARY)
    {
      id: Int! @pgColumn(name: "id", type: INT)
      authorEmail: String! @pgColumn(name: "author_email", type: VARCHAR)
      authorName: String @pgColumn(name: "author_name", type: VARCHAR)
      author: User! @derive @is(field: "{ authorEmail: email authorName: name }")
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
      """
      The value of the authorEmail field
      """
      authorEmail: String! @shareable
      """
      The value of the authorName field
      """
      authorName: String @shareable
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
      @key(fields: "email name")
    {
      email: String!
      name: String
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
    }
    "#);
}
