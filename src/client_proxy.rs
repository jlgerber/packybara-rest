use postgres::{Client, NoTls, Config, ToStatement, RowIter, Statement, CopyInWriter, CopyOutReader, Transaction, TransactionBuilder, CancelToken};
use tokio_postgres::tls::{MakeTlsConnect, TlsConnect};
use tokio_postgres::types::{ToSql, Type};
use tokio_postgres::{Error, Row, SimpleQueryMessage, Socket};
use rocket_contrib::databases::{database, Poolable};

pub struct ClientProxy(Client);

//#[cfg(feature = "postgres_pool")]
impl Poolable for ClientProxy {
    type Manager = r2d2_postgres::PostgresConnectionManager<postgres::tls::NoTls>;
    type Error = DbError<postgres::Error>;

    fn pool(config: DatabaseConfig<'_>) -> Result<r2d2::Pool<Self::Manager>, Self::Error> {
        let manager = r2d2_postgres::PostgresConnectionManager::new(
            config.url.parse().map_err(DbError::Custom)?,
            postgres::tls::NoTls,
        );

        r2d2::Pool::builder().max_size(config.pool_size).build(manager)
            .map_err(DbError::PoolError)
    }
}

impl ClientProxy {
   
    /// A convenience function which parses a configuration string into a `Config` and then connects to the database.
    ///
    /// See the documentation for [`Config`] for information about the connection syntax.
    ///
    /// [`Config`]: config/struct.Config.html
    pub fn connect<T>(params: &str, tls_mode: T) -> Result<Client, Error>
    where
        T: MakeTlsConnect<Socket> + 'static + Send,
        T::TlsConnect: Send,
        T::Stream: Send,
        <T::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        Client::connect(params, tls_mode)
    }

    /// Returns a new `Config` object which can be used to configure and connect to a database.
    pub fn configure() -> Config {
       Client::configure()
    }

    /// Executes a statement, returning the number of rows modified.
    ///
    /// A statement may contain parameters, specified by `$n`, where `n` is the index of the parameter of the list
    /// provided, 1-indexed.
    ///
    /// If the statement does not modify any rows (e.g. `SELECT`), 0 is returned.
    ///
    /// The `query` argument can either be a `Statement`, or a raw query string. If the same statement will be
    /// repeatedly executed (perhaps with different query parameters), consider preparing the statement up front
    /// with the `prepare` method.
    ///
    /// # Panics
    ///
    /// Panics if the number of parameters provided does not match the number expected.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    ///
    /// # fn main() -> Result<(), postgres::Error> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let bar = 1i32;
    /// let baz = true;
    /// let rows_updated = client.execute(
    ///     "UPDATE foo SET bar = $1 WHERE baz = $2",
    ///     &[&bar, &baz],
    /// )?;
    ///
    /// println!("{} rows updated", rows_updated);
    /// # Ok(())
    /// # }
    /// ```
    pub fn execute<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement,
    {
        self.0.execute(query, params)
    }

    /// Executes a statement, returning the resulting rows.
    ///
    /// A statement may contain parameters, specified by `$n`, where `n` is the index of the parameter of the list
    /// provided, 1-indexed.
    ///
    /// The `query` argument can either be a `Statement`, or a raw query string. If the same statement will be
    /// repeatedly executed (perhaps with different query parameters), consider preparing the statement up front
    /// with the `prepare` method.
    ///
    /// # Panics
    ///
    /// Panics if the number of parameters provided does not match the number expected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    ///
    /// # fn main() -> Result<(), postgres::Error> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let baz = true;
    /// for row in client.query("SELECT foo FROM bar WHERE baz = $1", &[&baz])? {
    ///     let foo: i32 = row.get("foo");
    ///     println!("foo: {}", foo);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        self.0.query(query, params)
    }

    /// Executes a statement which returns a single row, returning it.
    ///
    /// Returns an error if the query does not return exactly one row.
    ///
    /// A statement may contain parameters, specified by `$n`, where `n` is the index of the parameter of the list
    /// provided, 1-indexed.
    ///
    /// The `query` argument can either be a `Statement`, or a raw query string. If the same statement will be
    /// repeatedly executed (perhaps with different query parameters), consider preparing the statement up front
    /// with the `prepare` method.
    ///
    /// # Panics
    ///
    /// Panics if the number of parameters provided does not match the number expected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    ///
    /// # fn main() -> Result<(), postgres::Error> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let baz = true;
    /// let row = client.query_one("SELECT foo FROM bar WHERE baz = $1", &[&baz])?;
    /// let foo: i32 = row.get("foo");
    /// println!("foo: {}", foo);
    /// # Ok(())
    /// # }
    /// ```
    pub fn query_one<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement,
    {
        self.0.query_one(query, params)
    }

    /// Executes a statement which returns zero or one rows, returning it.
    ///
    /// Returns an error if the query returns more than one row.
    ///
    /// A statement may contain parameters, specified by `$n`, where `n` is the index of the parameter of the list
    /// provided, 1-indexed.
    ///
    /// The `query` argument can either be a `Statement`, or a raw query string. If the same statement will be
    /// repeatedly executed (perhaps with different query parameters), consider preparing the statement up front
    /// with the `prepare` method.
    ///
    /// # Panics
    ///
    /// Panics if the number of parameters provided does not match the number expected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    ///
    /// # fn main() -> Result<(), postgres::Error> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let baz = true;
    /// let row = client.query_opt("SELECT foo FROM bar WHERE baz = $1", &[&baz])?;
    /// match row {
    ///     Some(row) => {
    ///         let foo: i32 = row.get("foo");
    ///         println!("foo: {}", foo);
    ///     }
    ///     None => println!("no matching foo"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn query_opt<T>(
        &mut self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        self.0.query_opt(query, params)
    }

    /// A maximally-flexible version of `query`.
    ///
    /// It takes an iterator of parameters rather than a slice, and returns an iterator of rows rather than collecting
    /// them into an array.
    ///
    /// # Panics
    ///
    /// Panics if the number of parameters provided does not match the number expected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    /// use fallible_iterator::FallibleIterator;
    /// use std::iter;
    ///
    /// # fn main() -> Result<(), postgres::Error> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let baz = true;
    /// let mut it = client.query_raw("SELECT foo FROM bar WHERE baz = $1", iter::once(&baz as _))?;
    ///
    /// while let Some(row) = it.next()? {
    ///     let foo: i32 = row.get("foo");
    ///     println!("foo: {}", foo);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn query_raw<'a, T, I>(&mut self, query: &T, params: I) -> Result<RowIter<'_>, Error>
    where
        T: ?Sized + ToStatement,
        I: IntoIterator<Item = &'a dyn ToSql>,
        I::IntoIter: ExactSizeIterator,
    {
       self.0.query_raw(query, params)
    }

    /// Creates a new prepared statement.
    ///
    /// Prepared statements can be executed repeatedly, and may contain query parameters (indicated by `$1`, `$2`, etc),
    /// which are set when executed. Prepared statements can only be used with the connection that created them.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    ///
    /// # fn main() -> Result<(), postgres::Error> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let statement = client.prepare("SELECT name FROM people WHERE id = $1")?;
    ///
    /// for id in 0..10 {
    ///     let rows = client.query(&statement, &[&id])?;
    ///     let name: &str = rows[0].get(0);
    ///     println!("name: {}", name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn prepare(&mut self, query: &str) -> Result<Statement, Error> {
        self.0.prepare(query)
    }

    /// Like `prepare`, but allows the types of query parameters to be explicitly specified.
    ///
    /// The list of types may be smaller than the number of parameters - the types of the remaining parameters will be
    /// inferred. For example, `client.prepare_typed(query, &[])` is equivalent to `client.prepare(query)`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    /// use postgres::types::Type;
    ///
    /// # fn main() -> Result<(), postgres::Error> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let statement = client.prepare_typed(
    ///     "SELECT name FROM people WHERE id = $1",
    ///     &[Type::INT8],
    /// )?;
    ///
    /// for id in 0..10 {
    ///     let rows = client.query(&statement, &[&id])?;
    ///     let name: &str = rows[0].get(0);
    ///     println!("name: {}", name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn prepare_typed(&mut self, query: &str, types: &[Type]) -> Result<Statement, Error> {
        self.0.prepare_typed(query, types)
    }

    /// Executes a `COPY FROM STDIN` statement, returning the number of rows created.
    ///
    /// The `query` argument can either be a `Statement`, or a raw query string. The data in the provided reader is
    /// passed along to the server verbatim; it is the caller's responsibility to ensure it uses the proper format.
    /// PostgreSQL does not support parameters in `COPY` statements, so this method does not take any.
    ///
    /// The copy *must* be explicitly completed via the `finish` method. If it is not, the copy will be aborted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    /// use std::io::Write;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let mut writer = client.copy_in("COPY people FROM stdin")?;
    /// writer.write_all(b"1\tjohn\n2\tjane\n")?;
    /// writer.finish()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy_in<T>(&mut self, query: &T) -> Result<CopyInWriter<'_>, Error>
    where
        T: ?Sized + ToStatement,
    {
        self.0.copy_in(query)
    }

    /// Executes a `COPY TO STDOUT` statement, returning a reader of the resulting data.
    ///
    /// The `query` argument can either be a `Statement`, or a raw query string. PostgreSQL does not support parameters
    /// in `COPY` statements, so this method does not take any.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    /// use std::io::Read;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let mut reader = client.copy_out("COPY people TO stdout")?;
    /// let mut buf = vec![];
    /// reader.read_to_end(&mut buf)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy_out<T>(&mut self, query: &T) -> Result<CopyOutReader<'_>, Error>
    where
        T: ?Sized + ToStatement,
    {
        self.0.copy_out(query)
    }

    /// Executes a sequence of SQL statements using the simple query protocol.
    ///
    /// Statements should be separated by semicolons. If an error occurs, execution of the sequence will stop at that
    /// point. The simple query protocol returns the values in rows as strings rather than in their binary encodings,
    /// so the associated row type doesn't work with the `FromSql` trait. Rather than simply returning the rows, this
    /// method returns a sequence of an enum which indicates either the completion of one of the commands, or a row of
    /// data. This preserves the framing between the separate statements in the request.
    ///
    /// This is a simple convenience method over `simple_query_iter`.
    ///
    /// # Warning
    ///
    /// Prepared statements should be use for any query which contains user-specified data, as they provided the
    /// functionality to safely imbed that data in the request. Do not form statements via string concatenation and pass
    /// them to this method!
    pub fn simple_query(&mut self, query: &str) -> Result<Vec<SimpleQueryMessage>, Error> {
        self.0.simple_query(query)
    }

    /// Executes a sequence of SQL statements using the simple query protocol.
    ///
    /// Statements should be separated by semicolons. If an error occurs, execution of the sequence will stop at that
    /// point. This is intended for use when, for example, initializing a database schema.
    ///
    /// # Warning
    ///
    /// Prepared statements should be use for any query which contains user-specified data, as they provided the
    /// functionality to safely embed that data in the request. Do not form statements via string concatenation and pass
    /// them to this method!
    pub fn batch_execute(&mut self, query: &str) -> Result<(), Error> {
        self.0.batch_execute(query)
    }

    /// Begins a new database transaction.
    ///
    /// The transaction will roll back by default - use the `commit` method to commit it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    ///
    /// # fn main() -> Result<(), postgres::Error> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let mut transaction = client.transaction()?;
    /// transaction.execute("UPDATE foo SET bar = 10", &[])?;
    /// // ...
    ///
    /// transaction.commit()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn transaction(&mut self) -> Result<Transaction<'_>, Error> {
        self.0.transaction()
    }

    /// Returns a builder for a transaction with custom settings.
    ///
    /// Unlike the `transaction` method, the builder can be used to control the transaction's isolation level and other
    /// attributes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, IsolationLevel, NoTls};
    ///
    /// # fn main() -> Result<(), postgres::Error> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let mut transaction = client.build_transaction()
    ///     .isolation_level(IsolationLevel::RepeatableRead)
    ///     .start()?;
    /// transaction.execute("UPDATE foo SET bar = 10", &[])?;
    /// // ...
    ///
    /// transaction.commit()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build_transaction(&mut self) -> TransactionBuilder<'_> {
        self.0.build_transaction()
    }

    /// Constructs a cancellation token that can later be used to request
    /// cancellation of a query running on this connection.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use postgres::{Client, NoTls};
    /// use postgres::error::SqlState;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    ///
    /// let cancel_token = client.cancel_token();
    ///
    /// thread::spawn(move || {
    ///     // Abort the query after 5s.
    ///     thread::sleep(Duration::from_secs(5));
    ///     cancel_token.cancel_query(NoTls);
    /// });
    ///
    /// match client.simple_query("SELECT long_running_query()") {
    ///     Err(e) if e.code() == Some(&SqlState::QUERY_CANCELED) => {
    ///         // Handle canceled query.
    ///     }
    ///     Err(err) => return Err(err.into()),
    ///     Ok(rows) => {
    ///         // ...
    ///     }
    /// }
    /// // ...
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_token(&self) -> CancelToken {
        self.0.cancel_token()
    }

    /// Determines if the client's connection has already closed.
    ///
    /// If this returns `true`, the client is no longer usable.
    pub fn is_closed(&self) -> bool {
        self.0.is_closed()
    }
}