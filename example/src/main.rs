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
        "http.status_code" : u16  = status_code,
        request_type,
        image_id           : u64  = 14335072086939106204_u64,
        // We MAY give a &str type but this will be assumed
        // if the type is omitted entirely
        a_string           : &str = "hello world",
        // We also support signed integers and floats
        neg_int            : i64  = -124_i64,
        a_float            : f32  = 1.2456_f32
    });

    // JSON null and Option
    let mut needed_id: Option<&str> = None;
    warn!("needed_id is unknown", {
        needed_id : Option<&str> = needed_id,
        healthy   : bool           = false
    });
    needed_id = Some("12345");
    let data_size: usize = 1234;
    info!("found needed_id", {
        needed_id : Option<&str> = needed_id,
        healthy   : bool         = true,
        data_size : usize        = data_size
    });

    // JSON arrays
    let top_right = [121.0_f32, 156.0];
    info!("position found", {
        ["position.bottom_left" : u32         = [21, 56]],
        ["position.top_right"   : f32         = top_right],
        // If not type is given, we assume &str - same as non-arrays
        [keys                                 = ["key1", "key2"]]
    });

    // Primative types may be an Option - arrays may not be
    let int: Option<u32> = None;
    let float: Option<f64> = None;
    let s: Option<&str> = None;
    info!("all primative types may be Options, arrays cannot be", {
        int   : Option<u32>  = int,
        float : Option<f64>  = float,
        s     : Option<&str> = s
    });

    // We try to support special values gracefully
    info!("special values", {
            // This will serialize as a string
            number : f64 = f64::INFINITY,
            // Put tab character into key
            "odd_\t_key" = "boo to a goose\r\nand you!\r\n"
    });
}
