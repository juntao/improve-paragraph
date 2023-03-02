use slack_flows::{listen_to_channel, send_message_to_channel};
use openai_flows::{CompletionRequest, create_completion};

#[no_mangle]
pub fn run() {
    listen_to_channel("secondstate", "improve-paragraph", |sm| {
        let cr = CompletionRequest {
            prompt: "I want you to act as an English copy editor and proof reader. I will send you a paragraph of text in English and you will answer in corrected and improved version of my text. I want you to replace my simplified A0-level words and sentences with more beautiful and elegant, upper level English words and sentences. Keep the meaning same, but make them more literary. I want you to only reply the correction, the improvements and nothing else, do not write explanations. My text is \"".to_owned() + &sm.text + "\"",
            max_tokens: 2048,
            ..Default::default()
        };
        let r = create_completion("Agent", cr);
        r.iter().for_each(|c| {
            send_message_to_channel("secondstate", "improve-paragraph", c.to_string());
        });
    });
}
