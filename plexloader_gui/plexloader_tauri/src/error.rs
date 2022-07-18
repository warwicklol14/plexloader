use anyhow::Error;

pub fn error_to_string(e: Error) -> String {
    let mut text = format!("{}", e.to_string());
    e.chain()
        .skip(1)
        .for_each(|cause| {
            text.push_str("\n");
            text.push_str(&cause.to_string());
        });
    text
}
