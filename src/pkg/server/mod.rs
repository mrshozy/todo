use crate::preludes::TodoResult;
use axum::Router;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;

pub async fn serve_forever(router: Router, port: u16) -> TodoResult<()> {
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port)).await?;
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
