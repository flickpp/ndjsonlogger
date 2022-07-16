use ndjsonlogger::{debug, error, info, warn};

fn main() {
    info!("example ndjsonlogger started");

    // debug! lines compile out to no-op in release builds
    debug!("this won't print if cargo run --release is run");

    // We can log key/value pairs
    // We may use either string literals, OR idents as our key
    info!("log an additional string", {
        resource_id     = "c2ba4f71ebdbd77d4a376c68692ab7e0",
        "resource.name" = "picture of hamster"
    });

    // We may log a single ident providing it is an ident
    let reason = "resource_id does not exist";
    error!("couldn't complete request", { reason });

    // JSON types
    let request_type = "fetch image";
    let status_code: u16 = 200;
    info!("http request complete", {
        "http.status_code" : u16 = status_code,
        request_type,
        image_id           : u64 = 14335072086939106204_u64
    });

    // JSON null and Option
    let mut needed_id: Option<&str> = None;
    warn!("needed_id is unknown", {
        needed_id : Option<&str> = needed_id,
        healthy   : bool           = false
    });
    needed_id = Some("12345");
    info!("found needed_id", {
        needed_id : Option<&str> = needed_id,
        healthy   : bool         = true
    });

    // JSON arrays
    let top_right = [121.5_f32, 156.];
    info!("position found", {
        ["position.bottom_left" : u32 = [21, 56]],
        ["position.top_right"   : f32 = top_right],
        // If not type is given, we assume &str - same as non-arrays
        [keys                         = ["key1", "key2"]]
    });
}
