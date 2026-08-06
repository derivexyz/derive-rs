use std::str::FromStr;

use bigdecimal::BigDecimal;
use derive_rs::{
    actions::{ExecuteQuoteArgs, SendQuoteArgs},
    models::{
        CancelBatchRfqsRequest, Direction, GetTickerRequest, LegUnpricedParams, PollRfqsRequest,
        PricedLegParamsAndResponse, PublicRfq, RfqGetBestQuoteRequest, RfqStatus, SendRfqRequest,
    },
    ws_client::WsClient,
};

mod common;

async fn create_priced_legs(
    _client: &WsClient,
    rfq: &PublicRfq,
) -> Vec<PricedLegParamsAndResponse> {
    let mut priced_legs = Vec::new();

    for leg in &rfq.legs {
        let get_params = GetTickerRequest::builder()
            .instrument_name(leg.instrument_name.clone())
            .try_into()
            .expect("Must convert into request.");
        let ticker = _client
            .rpc()
            .market_data()
            .get_ticker(get_params)
            .await
            .expect("Must get ticker.");
        let price = ticker.m;
        let priced_leg = PricedLegParamsAndResponse::builder()
            .instrument_name(leg.instrument_name.clone())
            .amount(leg.amount.clone())
            .direction(leg.direction)
            .price(price) // Example price, replace with actual logic
            .try_into()
            .expect("Must convert into priced leg.");

        priced_legs.push(priced_leg);
    }

    priced_legs
}

#[tokio::test]
async fn test_ws_client_login() {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Login should succeed.");
    let subaccount_id = ws_client
        .subaccount_id
        .expect("Must have subaccount id set.");

    // we first cancel all so we can start with a clean slate
    let cancel_request = CancelBatchRfqsRequest::builder()
        .subaccount_id(subaccount_id)
        .try_into()
        .expect("Must convert into request.");
    ws_client
        .rfqs()
        .cancel_batch_rfqs(cancel_request)
        .await
        .expect("Cancel batch RFQs should succeed.");

    let leg = LegUnpricedParams::builder()
        .instrument_name("ETH-PERP")
        .amount(BigDecimal::from(1))
        .direction(Direction::Buy)
        .try_into()
        .expect("Must convert into legs.");

    let request = SendRfqRequest::builder()
        .legs(vec![leg])
        .subaccount_id(subaccount_id)
        .try_into()
        .expect("Must convert into request.");

    let request = ws_client
        .rfqs()
        .send_rfq(request)
        .await
        .expect("Send RFQ should succeed.");

    let quoter = common::get_test_ws_client_2().await;
    quoter.login().await.expect("Login should succeed.");

    let poll_rfqs_request = PollRfqsRequest::builder()
        .subaccount_id(quoter.subaccount_id.expect("Should have a subaccount."))
        .try_into()
        .expect("Must convert into request.");
    let polled_requests = quoter
        .rfqs()
        .poll_rfqs(poll_rfqs_request)
        .await
        .expect("Poll RFQs should succeed.");

    println!("Polled RFQs: {:?}", polled_requests);
    assert!(
        !polled_requests.rfqs.is_empty(),
        "There should be at least one RFQ"
    );

    // we get the request which matches the one we sent
    let matching_rfq = polled_requests
        .rfqs
        .iter()
        .find(|rfq| rfq.rfq_id == request.rfq_id)
        .expect("Should find the matching RFQ.");

    println!("Matching RFQ: {:?}", matching_rfq);
    // we build a quote for the RFQ

    let priced_legs = create_priced_legs(&quoter, matching_rfq).await;

    let quote_params = SendQuoteArgs::builder()
        .rfq_id(matching_rfq.rfq_id)
        .legs(priced_legs)
        .max_fee(BigDecimal::from_str("1.5").expect("Must convert to BigDecimal"))
        // .subaccount_id(quoter.subaccount_id.expect("Should have a subaccount."))
        .build();

    // we print the legs to see what we are sending
    println!("Quote Legs: {:?}", quote_params.legs);

    let _quote = quoter
        .rfqs()
        .send_quote(quote_params)
        .await
        .expect("Send quote should succeed.");

    // we should now be able to get the quotes from the original client
    let poll_rfqs_request = RfqGetBestQuoteRequest::builder()
        .rfq_id(request.rfq_id)
        .subaccount_id(ws_client.subaccount_id.expect("Should have a subaccount."))
        .try_into()
        .expect("Must convert into request.");

    let best_quote_response = ws_client
        .rfqs()
        .get_best_quote(poll_rfqs_request)
        .await
        .expect("Get best quote should succeed.");

    let best_quote = best_quote_response
        .best_quote
        .expect("There should be a best quote.");

    println!("Best Quote: {:?}", best_quote);

    let execute_quote_args = ExecuteQuoteArgs::builder()
        .rfq_id(request.rfq_id)
        .quote_id(best_quote.quote_id)
        .legs(best_quote.legs.clone())
        .max_fee(BigDecimal::from_str("1.5").expect("Must convert to BigDecimal"))
        .build();

    let execute_response = ws_client
        .rfqs()
        .execute_best_quote(execute_quote_args)
        .await
        .expect("Execute best quote should succeed.");

    println!("Execute Response: {:?}", execute_response);
    assert!(
        execute_response.status == RfqStatus::Filled,
        "The quote execution should be successful."
    );
}
