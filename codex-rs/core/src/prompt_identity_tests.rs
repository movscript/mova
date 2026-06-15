const IDENTITY_PROMPTS: &[(&str, &str)] = &[
    (
        "gpt_5_codex_prompt.md",
        include_str!("../gpt_5_codex_prompt.md"),
    ),
    (
        "gpt-5.1-codex-max_prompt.md",
        include_str!("../gpt-5.1-codex-max_prompt.md"),
    ),
    (
        "gpt-5.2-codex_prompt.md",
        include_str!("../gpt-5.2-codex_prompt.md"),
    ),
    ("gpt_5_1_prompt.md", include_str!("../gpt_5_1_prompt.md")),
    ("gpt_5_2_prompt.md", include_str!("../gpt_5_2_prompt.md")),
    (
        "prompt_with_apply_patch_instructions.md",
        include_str!("../prompt_with_apply_patch_instructions.md"),
    ),
    (
        "gpt-5.2-codex_instructions_template.md",
        include_str!("../templates/model_instructions/gpt-5.2-codex_instructions_template.md"),
    ),
    (
        "realtime/backend_prompt.md",
        include_str!("../../prompts/templates/realtime/backend_prompt.md"),
    ),
];

#[test]
fn assistant_identity_prompts_use_mova() {
    for (name, prompt) in IDENTITY_PROMPTS {
        let identity_line = prompt
            .lines()
            .find(|line| line.starts_with("You are "))
            .unwrap_or_default();
        assert!(
            identity_line.contains("You are Mova"),
            "{name} should introduce the assistant as Mova"
        );
        assert!(
            identity_line.contains("do not identify yourself as Codex"),
            "{name} should explicitly prevent Codex self-identification"
        );
        assert!(
            !identity_line.contains("You are Codex"),
            "{name} must not introduce the assistant as Codex"
        );
        assert!(
            !identity_line.contains("Codex CLI"),
            "{name} must not present the runtime as Codex CLI"
        );
    }
}
