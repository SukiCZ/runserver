/// Parse the request line into the method, path, and version.
///
/// # Arguments
///
/// * `request_line` - The request line to parse.
///
/// # Returns
///
/// A tuple containing the method, path, and version.
///
/// # Example
///
/// ```
/// let request_line = "GET / HTTP/1.1";
/// let (method, path, version) = parse_request_line(request_line);
/// assert_eq!(method, "GET");
/// assert_eq!(path, "/");
/// assert_eq!(version, "HTTP/1.1");
/// ```
pub fn parse_request_line(request_line: &str) -> (&str, &str, &str) {
    let mut parts = request_line.split_whitespace();
    (
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    )
}
