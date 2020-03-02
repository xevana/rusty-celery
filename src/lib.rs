//! A Rust implementation of [Celery](http://www.celeryproject.org/) for producing and consuming
//! asyncronous tasks with a distributed message queue.
//!
//! # Examples
//!
//! Define tasks by decorating functions with the [`task`](attr.task.html) attribute:
//!
//! ```rust
//! #[celery::task]
//! fn add(x: i32, y: i32) -> i32 {
//!     x + y
//! }
//! ```
//!
//! Then create a [`Celery`](struct.Celery.html) app with the [`app`](macro.app.html)
//! macro and register your tasks with it:
//!
//! ```rust,no_run
//! # #[celery::task]
//! # fn add(x: i32, y: i32) -> i32 {
//! #     x + y
//! # }
//! let my_app = celery::app!(
//!     broker = AMQP { std::env::var("AMQP_ADDR").unwrap() },
//!     tasks = [add],
//!     task_routes = [],
//! );
//! ```
//!
//! The Celery app can be used as either a producer or consumer (worker). To send tasks to a
//! queue for a worker to consume, use the [`Celery::send_task`](struct.Celery.html#method.send_task) method:
//!
//! ```rust,no_run
//! # #[celery::task]
//! # fn add(x: i32, y: i32) -> i32 {
//! #     x + y
//! # }
//! # #[tokio::main]
//! # async fn main() -> Result<(), exitfailure::ExitFailure> {
//! # let my_app = celery::app!(
//! #     broker = AMQP { std::env::var("AMQP_ADDR").unwrap() },
//! #     tasks = [add],
//! #     task_routes = [],
//! # );
//! my_app.send_task(add::new(1, 2)).await?;
//! #   Ok(())
//! # }
//! ```
//!
//! And to act as worker and consume tasks sent to a queue by a producer, use the
//! [`Celery::consume`](struct.Celery.html#method.consume) method:
//!
//! ```rust,no_run
//! # #[celery::task]
//! # fn add(x: i32, y: i32) -> i32 {
//! #     x + y
//! # }
//! # #[tokio::main]
//! # async fn main() -> Result<(), exitfailure::ExitFailure> {
//! # let my_app = celery::app!(
//! #     broker = AMQP { std::env::var("AMQP_ADDR").unwrap() },
//! #     tasks = [add],
//! #     task_routes = [],
//! # );
//! my_app.consume().await?;
//! # Ok(())
//! # }
//! ```

#![doc(
    html_favicon_url = "https://structurely-images.s3-us-west-2.amazonaws.com/logos/rusty-celery.ico"
)]
#![doc(
    html_logo_url = "https://structurely-images.s3-us-west-2.amazonaws.com/logos/rusty-celery-4.png"
)]

mod app;
pub use app::{Celery, CeleryBuilder};
pub mod broker;
pub mod error;
pub mod protocol;
pub mod task;

#[cfg(feature = "codegen")]
mod codegen;

/// A procedural macro for generating a [`Task`](task/trait.Task.html) from a function.
///
/// # Parameters
///
/// - `name`: The name to use when registering the task. Should be unique. If not given the name
/// will be set to the name of the function being decorated.
/// - `timeout`: Corresponds to [`Task::timeout`](trait.Task.html#method.timeout).
/// - `max_retries`: Corresponds to [`Task::max_retries`](trait.Task.html#method.max_retries).
/// - `min_retry_delay`: Corresponds to [`Task::min_retry_delay`](trait.Task.html#method.min_retry_delay).
/// - `max_retry_delay`: Corresponds to [`Task::max_retry_delay`](trait.Task.html#method.max_retry_delay).
/// - `acks_late`: Corresponds to [`Task::acks_late`](trait.Task.html#method.acks_late).
/// - `bind`: A bool. If true, the task will be run like an instance method and so the function's
/// first argument should be a reference to `Self`. Note however that Rust won't allow you to call
/// the argument `self`. Instead, you could use `task` or just `t`.
///
/// For more information see the [tasks chapter](https://rusty-celery.github.io/guide/defining-tasks.html)
/// in the Rusty Celery Book.
///
/// ## Examples
///
/// Create a task named `add` with all of the default options:
///
/// ```rust
/// #[celery::task]
/// fn add(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// ```
///
/// Use a name different from the function name:
///
/// ```rust
/// #[celery::task(name = "sum")]
/// fn add(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// ```
///
/// Customize the default retry behavior:
///
/// ```rust
/// #[celery::task(
///     timeout = 3,
///     max_retries = 100,
///     min_retry_delay = 1,
///     max_retry_delay = 60,
/// )]
/// async fn io_task() {
///     // Do some async IO work that could possible fail, such as an HTTP request...
/// }
/// ```
///
/// Bind the function to the task instance so it runs like an instance method:
///
/// ```rust
/// # use celery::task::Task;
/// #[celery::task(bind = true)]
/// fn bound_task(task: &Self) {
///     println!("Hello, World! From {}", task.name());
/// }
/// ```
#[cfg(feature = "codegen")]
pub use codegen::task;

#[cfg(feature = "codegen")]
#[doc(hidden)]
pub mod export;

#[cfg(feature = "codegen")]
extern crate futures;

#[cfg(feature = "codegen")]
extern crate once_cell;

#[cfg(feature = "codegen")]
extern crate async_trait;

#[cfg(feature = "codegen")]
extern crate serde;
