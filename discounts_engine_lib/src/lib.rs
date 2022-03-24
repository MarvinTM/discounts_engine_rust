#[macro_use]
extern crate serde;
extern crate serde_derive;
use serde::Serialize;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
struct Discount {
    ruleId: String,
    discountType: String,
    name: String,
    applyNext: bool,
    //    incompatibleDiscounts: String, // return null
    amt: f32,
    qtyOffer: i32,
    calculatedOnDiscountEngine: bool,
    obdiscQtyoffer: i32,
    displayedTotalAmount: f32,
    fullAmt: f32,
    actualAmt: f32,
    unitDiscount: f32,
    baseGrossUnitPrice: f32,
    baseNetUnitPrice: f32,
}

#[derive(Serialize)]
struct DiscountLine {
    id: String,
    grossUnitAmount: f32,
    grossUnitPrice: f32,
    discounts: [Discount; 1],
}

#[derive(Serialize)]
struct DiscountResult {
    lines: [DiscountLine; 1],
}

#[derive(Deserialize)]
struct Ticket {
    lineId: String
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn applyDiscount(ticket: &JsValue) -> JsValue{
    let ticketTransformed : Ticket = JsValue::into_serde(ticket).unwrap();

    // receives some things
    let discounts: [Discount; 1] = [Discount {
        ruleId: String::from("C26B841C84B14FE2AB1A334DD3672E87"),
        discountType: String::from("697A7AB9FD9C4EE0A3E891D3D3CCA0A7"),
        name: String::from("name"),
        applyNext: false,
        amt: 3.5,
        qtyOffer: 3,
        calculatedOnDiscountEngine: true,
        obdiscQtyoffer: 1,
        displayedTotalAmount: 1.5,
        fullAmt: 1.5,
        actualAmt: 1.5,
        unitDiscount: 1.5,
        baseGrossUnitPrice: 2.5,
        baseNetUnitPrice: 2.5,
    }];

    let discountLines: [DiscountLine; 1] = [DiscountLine {
        id: ticketTransformed.lineId,
        grossUnitAmount: 5.1,
        grossUnitPrice: 0.1,
        discounts: discounts,
    }];

    let discountResult = DiscountResult {
        lines: discountLines,
    };

    return JsValue::from_serde(&discountResult).unwrap();
    // alert(&format!("Hello, {}!", stringify(discountResult)));
}
