# PostgreSQL DB

## Overview

What is ORM?
![](../../img/pgsql_orm.png)

![](../../img/pgsql_orm2.png)

This would be fine for simple example. ORM is good for development, but not so good for performance. So for complex apps, direct SQL query is the solution.

**ORM (Object-Relational Mapping)**:

- Designed for relational databases such as MySQL, PostgreSQL, and SQL Server.
- Relational databases store data in tables, with each row representing an entity and columns representing the attributes of the entity.
- ORM maps objects in the application code to rows in the database tables, and it handles the conversion between the object-oriented programming model and the relational model.
- ORM libraries often provide features like lazy-loading, caching, transactions, and connection pooling.
- Examples of ORM libraries include SQLAlchemy (Python), ActiveRecord (Ruby), Hibernate (Java), and Entity Framework (C#).

## Installation

### macOS (M1)

#### PostgreSQL

```sh
$ brew install postgresql libpq
```

> Prefer to use postgresql@14

---

start/restart server:

```sh
# start
$ brew services start postgresql@14
# restart
$ brew services restart postgresql@14
```

---

stop server:

```sh
$ brew services stop postgresql@14
```

---

Also, there is an option to use GUI tool for managing postgresql server. For example, [Postgres.app](https://postgresapp.com/).

#### MySQL

#### SQLite
