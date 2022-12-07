# Omi - Object Mapping Intelligently

A library that maps object structures to relational databases and provides
interfaces for common queries and updates.

[![License](https://img.shields.io/github/license/amphitheatre-app/omi)](https://github.com/amphitheatre-app/omi/blob/master/LICENSE)
[![GitHub contributors](https://img.shields.io/github/contributors/amphitheatre-app/omi)](https://github.com/amphitheatre-app/omi/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/amphitheatre-app/omi)](https://github.com/amphitheatre-app/omi/issues)

## Install

Omi is under active development and when adding a dependency you will need to
add it via the git repository method like the following:

```toml
# Cargo.toml
[dependencies]
omi = { git = "https://github.com/amphitheatre-app/omi" }
```

## Getting Started

Let's start with a quick preview of some of Omi's main current features, which
will be updated here and in possible future special documents as they become
available, so stay tuned!

## Entity definition

```rust
use omi::prelude::*

#[entity(table = "products")]
#[derive(Entity, Queryable, Creatable, Updatable, Deletable)]
pub struct Product {
  /// Auto incremented primary key
  #[column(primary, auto, null)]
  id: u64,

  /// The title of product
  #[column(length = 255, default = "")]
  title: String

  /// The brand of this product
  #[relation(belongs_to)]
  brand: Brand,

  /// Has many reviews
  #[relation(has_many)
  reviews: Vec<Review>,
}
```

## Querying

Using the `one()` method allows you to retrieve a single record without setting
any filter conditions, which is equivalent to "`SELECT * FROM products offset 0
limit 1`". Of course, you can also specify filter fields:

```rust
Product::find().one(db);
```

Typically, we use the `all()` method to get multiple rows, which will query the
database based on the filters you submit, the equivalent SQL is `SELECT * FROM
products`.

```rust
Product::find().all(db);
```

Omi's `filter()` method provides a more advanced filtering capability, where you
can simply enter a tuple, or a vector of combinations of conditions:

```rust
// SQL: SELECT * FROM products WHERE id=123
Product::find().filter(("id", 123)).all(db);

// SQL: SELECT * FROM products WHERE foo=123 AND bar=456
Product::find()
    .filter([("foo", 123), ("bar", 456)])
    .all(db);
```

Omi can also be more advanced conditional expressions to support `AND`, `OR`
filtering, the equivalent SQL is `SELECT * FROM products WHERE (title like
"*abc" AND brand_id=123) OR (title like "*xyz" AND brand_id=456)`.

```rust
Product::find()
    .filter(
        Or(
            And([("title", "*abc"), ("brand_id", 123)]),
            And([("title", "*xyz"), ("brand_id", 456)]),
        )
    )
    .all(db);
```

The `order_by()` method takes a tuple to specify the fields to be sorted and the
direction, or it can be transferred to a tuple vector to sort multiple fields.

```rust
// single field
Product::find()
    .filter(("id", 123))
    .order_by(("id", Desc))
    .all(db);

// multiple fields
Product::find()
    .filter(("id", 123))
    .order_by([("id", Desc), ("id", Desc)])
    .all(db);
```

And the group by operation is similar.

```rust
Product::find().group_by(["brand_id")]).all(db);
```

Finally, the `offset` and `limit` limits are essential for query statements

```rust
Product::find().offset(0).limit(20).all(db);
```

## Create

Create an instance of your entity and simply call the `create()` method to insert a
new record into the database:

```rust
Product::create(product).execute(db);
```

## Update

The first way is to make changes to the instance and then call the `update()` method
to update it. Omi will automatically recognize the model of this instance and
compare the differences, saving only the changed part of the fields to the
database.

```rust
Product::update(product).execute(db);
```

Of course, you can also call the `update()` method to update the specified fields
raw, which eliminates the need for prior data loading, reduces the number of
database reads, and, in certain cases, optimizes the processing speed of the
application.

```rust
// single field
Product::update(product)
    .filter(("id", 123))
    .set("title", "abc")
    .execute(db);

// multiple fields
Product::update(product)
    .filter(("id", 123))
    .set([("title", "abc"), ("brand_id", "456")])
    .execute(db);
```

## Delete

```rust
Product::delete(product).execute(db);
```

## Relations

You can call the `include()` or `exclude()` methods to include or exclude associated
objects, and by default all associated data for this model will be included.

```rust
Product::find().filter(("id", 123)).include("reviews").one(db);
Product::find().filter(("id", 123)).exclude("reviews").one(db);
```

## Transaction

```rust
omi::transaction.start();
omi::transaction.rollback();
omi::transaction.commit();
```

## Raw SQL Querying

```rust
omi::sql::<Product>(
    format!("SELECT * FROM products where id = {}", 123)
).execute(db);
```

## Developing guide

If you're new to Rust, the first thing you'll need to do is follow the setup
instructions to install the Rust compiler and build tools. Once you're set up,
you can use Cargo to build and run your programs.

You can develop this library using either the stable or the nightly version of
Rust. If you want to use the latest and greatest features of the Rust language,
you can switch to the nightly channel using the following command:

```shell
rustup default nightly
```

Alternatively, you can set the version manually:

```shell
rustup install nightly-2022-12-01
rustup override add nightly-2022-12-01
```

## Contributing

If anything feels off, or if you feel that some functionality is missing, please
check out the contributing page. There you will find instructions for sharing
your feedback, building the tool locally, and submitting pull requests to the
project.

## License

Omi is licensed under [Apache License 2.0](https://github.com/amphitheatre-app/amphitheatre/blob/master/LICENSE).
See [LICENSE](https://github.com/amphitheatre-app/amphitheatre/blob/master/LICENSE) for more information.
