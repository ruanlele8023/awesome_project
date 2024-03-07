use std::collections::HashMap;

pub fn query_page(args: &HashMap<String, String>) -> (u64, u64) {
    let mut offset: u64 = 0;
    let mut limit: u64 = 20;

    if let Some(v) = args.get("size") {
        let size = v.parse::<u64>().unwrap_or_default();

        if size > 0 {
            limit = size
        }
    }

    if limit > 100 {
        limit = 100
    }

    if let Some(v) = args.get("page") {
        let page = v.parse::<u64>().unwrap_or_default();

        if page > 0 {
            offset = (page - 1) * limit
        }
    }

    (offset, limit)
}
