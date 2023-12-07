mod behaviour;
mod codec;
mod config;
mod node;
mod demo_msg;
mod demo_net_service;
mod demo_node_status;


pub async fn run<S: State>(addr: SocketAddr, node: Node<S>) {
    let router = new_router(node);

    info!("📣 HTTP server listening on {addr}");
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to run http server");
}

pub fn new_router<S: State>(node: Node<S>) -> Router {
    Router::new()
        .route("/blocks", get(get_blocks::<S>))
        .route("/blocks/:number", get(get_block::<S>))
        .route("/balances", get(get_balances::<S>))
        .route("/account/nonce", get(next_account_nonce::<S>))
        .route("/transfer", post(transfer::<S>))
        .fallback(not_found)
        .layer(Extension(node))
}

async fn get_blocks<S: State>(
    Extension(node): Extension<Node<S>>,
    Query(params): Query<GetBlocksReq>,
) -> impl IntoResponse {
    info!("📣 >> get_blocks by: {:?}", params);
    let blocks: Vec<BlockResp> = node
        .get_blocks(params.from_number)
        .into_iter()
        .map(BlockResp::from)
        .collect();
    info!("📣 << get_blocks response: {:?}", blocks);

    Json(blocks)
}

async fn get_block<S: State>(
    Extension(node): Extension<Node<S>>,
    Path(number): Path<u64>,
) -> impl IntoResponse {
    info!("📣 >> get_block by: {:?}", number);
    let block = node.get_block(number).map(BlockResp::from);
    info!("📣 << get_block response: {:?}", block);

    Json(block)
}

async fn get_balances<S: State>(Extension(node): Extension<Node<S>>) -> impl IntoResponse {
    info!("📣 >> get_balances");
    let resp = json!({
        "last_block_hash": node.last_block_hash(),
        "balances": node.get_balances(),
    });
    info!("📣 << get_balances response: {:?}", resp);

    Json(resp)
}

async fn next_account_nonce<S: State>(
    Extension(node): Extension<Node<S>>,
    Query(params): Query<NonceReq>,
) -> impl IntoResponse {
    info!("📣 >> next_account_nonce by: {:?}", params);
    let resp = json!({ "nonce": node.next_account_nonce(&params.account) });
    info!("📣 << next_account_nonce response: {:?}", resp);

    Json(resp)
}

async fn transfer<S: State>(
    Extension(node): Extension<Node<S>>,
    Json(tx): Json<TxReq>,
) -> Result<impl IntoResponse, HttpError> {
    info!("📣 >> transfer: {:?}", tx);
    let resp = node.transfer(&tx.from, &tx.to, tx.value, tx.nonce);
    info!("📣 << transfer response: {:?}", resp);

    resp?;
    Ok(Json(json!({"success": true})))
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}