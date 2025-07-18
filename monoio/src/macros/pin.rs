/// Pins a value on the stack.
///
/// Calls to `async fn` return anonymous [`Future`] values that are `!Unpin`.
/// These values must be pinned before they can be polled. Calling `.await` will
/// handle this, but consumes the future. If it is required to call `.await` on
/// a `&mut _` reference, the caller is responsible for pinning the future.
///
/// Pinning may be done by allocating with [`Box::pin`] or by using the stack
/// with the `pin!` macro.
///
/// The following will **fail to compile**:
///
/// ```compile_fail
/// use monoio::pin;
/// async fn my_async_fn() {
///     // async logic here
/// }
///
/// #[monoio::main]
/// async fn main() {
///     let mut future = pin!(my_async_fn());
///     (&mut future).await;
/// }
/// ```
///
/// To make this work requires pinning:
///
/// ```
/// use monoio::pin;
///
/// async fn my_async_fn() {
///     // async logic here
/// }
///
/// #[monoio::main]
/// async fn main() {
///     let future = my_async_fn();
///     pin!(future);
///
///     (&mut future).await;
/// }
/// ```
///
/// Pinning is useful when using `select!` and stream operators that require `T:
/// Stream + Unpin`.
///
/// [`Future`]: trait@std::future::Future
/// [`Box::pin`]: std::boxed::Box::pin
///
/// # Usage
///
/// The `pin!` macro takes **identifiers** as arguments. It does **not** work
/// with expressions.
///
/// The following does not compile as an expression is passed to `pin!`.
///
/// ```compile_fail
/// use monoio::pin;
/// async fn my_async_fn() {
///     // async logic here
/// }
///
/// #[monoio::main]
/// async fn main() {
///     let mut future = pin!(my_async_fn());
///     (&mut future).await;
/// }
/// ```
///
/// Because assigning to a variable followed by pinning is common, there is also
/// a variant of the macro that supports doing both in one go.
///
/// ```
/// use monoio::{pin, select};
///
/// async fn my_async_fn() {
///     // async logic here
/// }
///
/// #[monoio::main]
/// async fn main() {
///     pin! {
///         let future1 = my_async_fn();
///         let future2 = my_async_fn();
///     }
///
///     select! {
///         _ = &mut future1 => {}
///         _ = &mut future2 => {}
///     }
/// }
/// ```
#[deprecated(note = "use std::pin::pin instead")]
#[macro_export]
macro_rules! pin {
    ($($x:ident),*) => { $(
        // Move the value to ensure that it is owned
        let mut $x = $x;
        // Shadow the original binding so that it can't be directly accessed
        // ever again.
        #[allow(unused_mut)]
        let mut $x = unsafe {
            $crate::macros::support::Pin::new_unchecked(&mut $x)
        };
    )* };
    ($(
            let $x:ident = $init:expr;
    )*) => {
        $(
            let $x = $init;
            $crate::pin!($x);
        )*
    };
}
