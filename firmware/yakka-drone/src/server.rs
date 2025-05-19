use embassy_executor::{_export::TaskPoolRef, SpawnToken, Spawner};
use embassy_net::{Stack, tcp::TcpSocket};
use embassy_time::Duration;

use yakka_subsystem::Subsystem;

/// A TCP server with rx/tx buffers of size `N`.
#[derive(Debug, Clone, Copy)]
pub struct Server<const P: u16, const N: usize = 0x1000>;

impl<const P: u16, const N: usize> Server<P, N> {}

impl<const P: u16, const N: usize> Subsystem for Server<P, N> {
    type Handle = ();

    type Context = Stack<'static>;

    fn subsystem_with_context(
        self,
        target_stack: Self::Context,
    ) -> impl Future<Output = Self::Handle>
    where
        Self: Sized,
    {
        /* todo: ask for the `task` macro to support generics */
        fn socket_task<const P: u16, const N: usize>(
            target_stack: Stack<'static>,
        ) -> SpawnToken<impl Sized> {
            async fn socket_task_async<const P: u16, const N: usize>(
                target_stack: Stack<'static>,
            ) -> ! {
                loop {
                    let (ref mut rx, ref mut tx) = ([0; N], [0; N]);

                    let mut target_socket = TcpSocket::new(target_stack, rx, tx);

                    target_socket.set_timeout(Some(Duration::from_secs(1)));

                    target_socket.set_keep_alive(Some(Duration::from_millis(500)));

                    match target_socket.accept(P).await {
                        Ok(..) => {
                            let mut target_buffer = [0; N];

                            loop {
                                if let Ok(byte_count) = target_socket.read(&mut target_buffer).await
                                {
                                    let target_bytes = &target_buffer[..byte_count];

                                    let _ = target_socket.write(target_bytes).await;
                                }
                            }
                        }
                        Err(..) => (),
                    }
                }
            }

            static POOL: TaskPoolRef = TaskPoolRef::new();

            // SAFETY: called with the same generic parameters
            unsafe {
                POOL.get::<_, 1>()
                    ._spawn_async_fn(move || socket_task_async::<P, N>(target_stack))
            }
        }

        async move {
            let socket_thread = socket_task::<P, N>(target_stack);

            Spawner::for_current_executor()
                .await
                .must_spawn(socket_thread);
        }
    }
}
