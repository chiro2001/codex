//! Covers role composition and attribution for captured model instructions.

use super::*;
use codex_context_fragments::AnnotatedContent;
use codex_context_fragments::RenderedFragment;
use codex_protocol::models::ContentItemMetadata;
use pretty_assertions::assert_eq;

#[test]
fn role_segment_filters_base_and_appends_bundled_guidance() {
    let shared = DEFAULT_MULTI_AGENT_V2_SHARED_USAGE_HINT_TEXT;
    let wait = DEFAULT_MULTI_AGENT_V2_WAIT_AGENT_USAGE_HINT_TEXT;
    let model_override = DEFAULT_MULTI_AGENT_V2_MODEL_OVERRIDE_USAGE_HINT_TEXT;
    let expected_body = format!(
        "Role.\n## Work\nContinue.\n{shared}\n{wait}\n\nThere are 2 available concurrency slots, meaning that up to 2 agents can be active at once, including you.\n\n{model_override}"
    );
    for marked in [false, true] {
        let instructions = MultiAgentRoleInstructions::Composed {
            base: "Role.\n## Plan tool\nOmit role checklist guidance.\n## Work\nContinue."
                .to_string(),
            marked,
            omit_update_plan_instructions: true,
            max_concurrency: 2,
            wait_agent_enabled: true,
            expose_model_overrides: true,
        };
        let expected_text = if marked {
            format!("<multi_agent_role>{expected_body}</multi_agent_role>")
        } else {
            expected_body.clone()
        };
        assert_eq!(
            instructions.render_fragment(),
            RenderedFragment::new(
                "developer",
                AnnotatedContent::text(
                    expected_text,
                    ContentItemKind("multi_agent.role_instructions".to_string()),
                    ContentItemMetadata::harness(),
                ),
            ),
        );
    }
}

#[test]
fn plaintext_task_scope_preserves_runtime_guidance_and_catalog_attribution() {
    let role = MultiAgentRoleInstructions::Composed {
        base: "Child role.\n## Plan tool\nOmit this plan section.".to_string(),
        marked: true,
        omit_update_plan_instructions: true,
        max_concurrency: 7,
        wait_agent_enabled: true,
        expose_model_overrides: true,
    }
    .with_plaintext_task_scope();
    let text = role.body();
    assert!(text.contains(PLAINTEXT_CHILD_TASK_SCOPE));
    assert!(!text.contains("Omit this plan section"));
    assert!(text.contains("7 available concurrency slots"));
    assert!(text.contains(DEFAULT_MULTI_AGENT_V2_WAIT_AGENT_USAGE_HINT_TEXT));
    assert!(text.contains(DEFAULT_MULTI_AGENT_V2_MODEL_OVERRIDE_USAGE_HINT_TEXT));
    assert_eq!(
        role.markers(),
        ("<multi_agent_role>", "</multi_agent_role>")
    );
}

#[test]
fn plaintext_task_scope_preserves_explicit_user_role_override() {
    let role = MultiAgentRoleInstructions::Configured("User-owned role guidance.".to_string());
    assert_eq!(role.clone().with_plaintext_task_scope(), role);
}
