//! Live pagination coverage: the real `paginate()` path against NetBox —
//! first-page math, walking every page, and `fetch_all` (the `list_all()`
//! merge). Uses IP addresses since the seed produces enough to span pages.

use std::collections::BTreeSet;

use serde_json::json;

use super::harness::{assert_page_shape, params, results, skip_unless_live, slim};
use crate::tools::ipam::{self, IpAddressesListParams};

fn count_of(resp: &serde_json::Value) -> u64 {
    resp["count"].as_u64().expect("count present")
}

#[tokio::test]
async fn first_page_reports_has_more_and_next_offset() {
    let env = skip_unless_live!();
    let p: IpAddressesListParams = params(json!({ "limit": 5, "offset": 0 }));
    let resp = slim(ipam::ip_addresses_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "ip-addresses?limit=5&offset=0");
    let count = count_of(&resp);
    assert_eq!(
        results(&resp).len() as u64,
        count.min(5),
        "first page should hold up to `limit` rows"
    );
    if count > 5 {
        assert_eq!(resp["has_more"], true);
        assert_eq!(resp["next_offset"], 5);
    }
}

#[tokio::test]
async fn walking_pages_visits_every_row_once() {
    let env = skip_unless_live!();
    let limit = 5u64;
    let mut offset = 0u64;
    let mut seen: BTreeSet<i64> = BTreeSet::new();
    let mut total: Option<u64> = None;

    for _ in 0..200 {
        let p: IpAddressesListParams = params(json!({ "limit": limit, "offset": offset }));
        let resp = slim(ipam::ip_addresses_list(&env.client, p).await.unwrap());
        assert_page_shape(&resp, "ip-addresses page");
        total.get_or_insert(count_of(&resp));

        for r in results(&resp) {
            seen.insert(r["id"].as_i64().expect("row has id"));
        }

        if resp["has_more"].as_bool().unwrap() {
            assert_eq!(resp["next_offset"].as_u64().unwrap(), offset + limit);
            offset += limit;
        } else {
            assert_eq!(resp["next_offset"].as_u64().unwrap(), count_of(&resp));
            break;
        }
    }

    assert_eq!(
        seen.len() as u64,
        total.expect("at least one page"),
        "paging should visit every row exactly once"
    );
}

#[tokio::test]
async fn fetch_all_returns_every_row_in_one_response() {
    let env = skip_unless_live!();
    let p: IpAddressesListParams = params(json!({ "fetch_all": true }));
    let resp = slim(ipam::ip_addresses_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "ip-addresses?fetch_all=true");
    let count = count_of(&resp);
    assert_eq!(
        results(&resp).len() as u64,
        count,
        "fetch_all should return every row"
    );
    assert_eq!(resp["has_more"], false);
    assert_eq!(resp["next_offset"].as_u64().unwrap(), count);
}
