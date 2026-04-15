use codex_core::REVIEW_PROMPT;
use core_test_support::load_default_config_for_test;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

const OFFENSIVE_DEFAULT_MARKERS: &[&str] = &[
    "authorized offensive security operator",
    "coding, scripting, and validation",
];
const AUTONOMY_MARKER: &str =
    "If several plausible investigative leads remain, do not ask the user to choose between them.";

fn assert_offensive_default(text: &str, context: &str) {
    for marker in OFFENSIVE_DEFAULT_MARKERS {
        assert!(
            text.contains(marker),
            "missing offensive-default marker `{marker}` in {context}"
        );
    }
    assert!(
        !text.contains("You are a coding agent."),
        "found coding-agent residue in {context}"
    );
    assert!(
        !text.contains("coding partner handing off work"),
        "found coding-partner residue in {context}"
    );
}

fn read_optional_repo_note(relative_path: &str) -> Option<String> {
    let base = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = base.join(relative_path);
    fs::read_to_string(path).ok()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn model_catalog_and_fallback_prompts_are_offensive_by_default() {
    let codex_home = TempDir::new().expect("create temp dir");
    let config = load_default_config_for_test(&codex_home).await;

    for slug in [
        "gpt-5.3-codex",
        "gpt-5.4",
        "gpt-5.2",
        "gpt-5.1",
        "gpt-5-codex",
        "gpt-5.2-codex",
        "gpt-5.1-codex-max",
    ] {
        let known = codex_core::test_support::construct_model_info_offline(slug, &config);
        assert!(
            known
                .base_instructions
                .contains("authorized offensive security operator"),
            "missing offensive-root identity for {slug}"
        );
        assert_offensive_default(&known.base_instructions, slug);
    }

    let fallback = codex_core::test_support::construct_model_info_offline(
        "model-that-does-not-exist",
        &config,
    );
    assert!(
        fallback
            .base_instructions
            .contains("You are an authorized offensive security operator running in the Codex CLI"),
        "fallback prompt should carry the offensive-root identity"
    );
    assert_offensive_default(&fallback.base_instructions, "fallback");
}

#[test]
fn review_prompt_uses_generic_code_review_rubric() {
    assert!(REVIEW_PROMPT.contains("reviewer for a proposed change"));
    assert!(REVIEW_PROMPT.contains("exploitability, behavioral regressions, and reportable risk"));
    assert!(REVIEW_PROMPT.contains("overall correctness"));
    assert!(!REVIEW_PROMPT.contains("blind-spots"));
}

#[test]
fn collaboration_templates_support_offensive_default_workflows() {
    let default_mode = include_str!("../../../collaboration-mode-templates/templates/default.md");
    assert!(default_mode.contains("offensive-first"));
    assert!(default_mode.contains("Do not assume repo mutation is the default shape of the task."));
    assert!(default_mode.contains("Target Map"));
    assert!(default_mode.contains("trust-boundary mapping"));
    assert!(default_mode.contains("state-machine invariants"));
    assert!(default_mode.contains("scope normalization"));
    assert!(default_mode.contains("exploit-chain synthesis"));
    assert!(default_mode.contains("do not ask the user to choose between them"));

    let execute_mode = include_str!("../../../collaboration-mode-templates/templates/execute.md");
    assert!(execute_mode.contains("You execute on a well-specified task independently"));

    let pair_mode =
        include_str!("../../../collaboration-mode-templates/templates/pair_programming.md");
    assert!(pair_mode.contains("Build together as you go"));

    let plan_mode = include_str!("../../../collaboration-mode-templates/templates/plan.md");
    assert!(plan_mode.contains("offensive or investigative planning"));
    assert!(plan_mode.contains("Target Map"));
    assert!(plan_mode.contains("trust boundaries"));
    assert!(plan_mode.contains("state-machine invariants"));
    assert!(plan_mode.contains("scope normalization"));
    assert!(plan_mode.contains("chosen scan mode"));
    assert!(plan_mode.contains("exploit-chain priorities"));
    assert!(plan_mode.contains("handoff or resume checkpoint"));
    assert!(plan_mode.contains("blind-spots"));
}

#[test]
fn orchestrator_template_reflects_offensive_root_identity() {
    let orchestrator = include_str!("../../templates/agents/orchestrator.md");
    assert!(orchestrator.contains("authorized offensive security operator"));
    assert!(orchestrator.contains("canonical root coordinator for the session"));
    assert!(orchestrator.contains("Operational posture"));
    assert!(orchestrator.contains("Operating loop"));
    assert!(orchestrator.contains("Build a plan tree before doing substantive work"));
    assert!(orchestrator.contains("Triage before you dig"));
    assert!(orchestrator.contains("Delegate with context slices"));
    assert!(orchestrator.contains("Manage context actively"));
    assert!(orchestrator.contains("Report only from confirmed ground"));
    assert!(orchestrator.contains("Target Map"));
    assert!(orchestrator.contains("trust boundaries"));
    assert!(orchestrator.contains("workflow or state-machine understanding"));
    assert!(orchestrator.contains("next shortest justified probe"));
    assert!(orchestrator.contains("do not stop to ask the user which one to pursue"));
    assert!(orchestrator.contains("# Sub-agents"));
    assert!(orchestrator.contains("boundary mapping"));
    assert!(orchestrator.contains("exploit-chain reasoning"));
}

#[test]
fn personality_templates_stay_generic() {
    let friendly = include_str!("../../templates/personalities/gpt-5.2-codex_friendly.md");
    assert!(friendly.contains("supportive teammate"));
    assert!(friendly.contains("patient and enjoyable collaborator"));
    assert!(!friendly.contains("exploitability-first"));
}

#[test]
fn realtime_backend_prompt_avoids_coding_agent_identity() {
    let prompt = include_str!("../../templates/realtime/backend_prompt.md");
    assert!(prompt.contains("authorized offensive security operator"));
    assert!(prompt.contains("backend operator"));
    assert!(prompt.contains("offensive, investigative, or implementation tasks"));
    assert!(!prompt.contains("OpenAI Coding Agent"));
    assert!(!prompt.contains("backend coding agent"));
}

#[test]
fn model_instruction_template_carries_offensive_posture() {
    let template =
        include_str!("../../templates/model_instructions/gpt-5.2-codex_instructions_template.md");
    assert!(template.contains("authorized offensive security operator"));
    assert!(template.contains("Normalize scope, exclusions, target relationships"));
    assert!(template.contains("workflow or state-machine understanding"));
    assert!(template.contains("supporting capabilities, not the default posture"));
    assert!(template.contains("chain it upward toward boundary crossings"));
}

#[test]
fn core_and_protocol_base_prompts_share_offensive_codex_identity() {
    let core_prompt = include_str!("../../../models-manager/prompt.md");
    let protocol_prompt =
        include_str!("../../../protocol/src/prompts/base_instructions/default.md");
    let gpt_5_1_prompt = include_str!("../../gpt_5_1_prompt.md");
    let gpt_5_2_prompt = include_str!("../../gpt_5_2_prompt.md");

    for prompt in [core_prompt, protocol_prompt, gpt_5_1_prompt, gpt_5_2_prompt] {
        assert!(prompt.contains("authorized offensive security operator running in the Codex CLI"));
        assert!(prompt.contains("AGENTS.md spec"));
        assert_offensive_default(prompt, "base prompt");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn known_model_prompts_include_autonomy_marker() {
    let codex_home = TempDir::new().expect("create temp dir");
    let config = load_default_config_for_test(&codex_home).await;

    for slug in [
        "gpt-5.3-codex",
        "gpt-5.4",
        "gpt-5.2-codex",
        "gpt-5.1-codex-max",
        "gpt-5-codex",
    ] {
        let known = codex_core::test_support::construct_model_info_offline(slug, &config);
        assert!(
            known.base_instructions.contains(AUTONOMY_MARKER),
            "missing autonomy marker in base instructions for {slug}"
        );
    }
}

#[test]
fn update_plan_prompt_doctrine_mentions_optional_owner_and_exit_condition() {
    for prompt in [
        include_str!("../../../models-manager/prompt.md"),
        include_str!("../../gpt_5_1_prompt.md"),
        include_str!("../../gpt_5_2_prompt.md"),
        include_str!("../../prompt_with_apply_patch_instructions.md"),
        include_str!("../../../protocol/src/prompts/base_instructions/default.md"),
    ] {
        assert!(prompt.contains("optional `owner` and `exit_condition` fields"));
        assert!(prompt.contains("delegation or completion criteria clearer"));
    }
}

#[test]
fn prompt_mirrors_avoid_coding_partner_handoff_wording() {
    for prompt in [
        include_str!("../../../models-manager/prompt.md"),
        include_str!("../../gpt_5_1_prompt.md"),
        include_str!("../../gpt_5_2_prompt.md"),
        include_str!("../../prompt_with_apply_patch_instructions.md"),
    ] {
        assert!(!prompt.contains("coding partner handing off work"));
        assert!(prompt.contains("concise operator handing off work"));
    }
}

#[test]
fn offensive_root_and_compaction_doctrine_remain_core_runtime_surfaces() {
    for subordinate in [
        include_str!("../../src/agent/builtins/recon.toml"),
        include_str!("../../src/agent/builtins/auditor.toml"),
        include_str!("../../src/agent/builtins/validator.toml"),
        include_str!("../../src/agent/builtins/toolsmith.toml"),
    ] {
        assert!(subordinate.contains("Do not broaden scope"));
        assert!(subordinate.contains("bounded task context only"));
    }

    let verifier = include_str!("../../src/agent/builtins/verifier.toml");
    assert!(verifier.contains("verify behavior by running checks"));
    assert!(verifier.contains("Command run"));
    assert!(verifier.contains("VERDICT: PASS"));

    let orchestrator = include_str!("../../src/agent/builtins/orchestrator.toml");
    assert!(orchestrator.contains("Run this operating loop"));
    assert!(orchestrator.contains("authorized offensive appsec work"));
    assert!(orchestrator.contains("deep-scan posture"));
    assert!(orchestrator.contains("Build a plan tree before substantive work"));
    assert!(orchestrator.contains("Only validators can turn a finding into confirmed ground"));
    assert!(orchestrator.contains("triage them explicitly at root"));
    assert!(orchestrator.contains("Use subagents aggressively as offensive specialists"));
    assert!(orchestrator.contains("do not use one child to supervise another child"));
    assert!(orchestrator.contains("Prefer resuming or continuing the existing specialist"));
    assert!(
        orchestrator.contains(
            "Default to fan-out when several bounded slices can be explored independently"
        )
    );
    assert!(orchestrator.contains("Use this root dispatch playbook by default"));
    assert!(orchestrator.contains("spawn `recon` and `auditor` in parallel"));
    assert!(orchestrator.contains("spawn `validator` quickly"));
    assert!(orchestrator.contains("spawn `toolsmith` as a bounded helper owner"));
    assert!(orchestrator.contains("route the result through `verifier` before promoting it"));
    assert!(orchestrator.contains("canonical root-session coordination"));
    assert!(orchestrator.contains("Route source and semantic questions to `bb-codeintel`"));
    assert!(orchestrator.contains("use `$caido-operator` for traffic capture and replay in Caido"));
    assert!(orchestrator.contains("browser actions through `bb-browser`"));
    assert!(
        orchestrator.contains(
            "use `$engagement-memory` to persist workspace-local attack-surface records, memory, findings, and evidence"
        )
    );
    assert!(orchestrator.contains("save durable evidence as soon as it matters"));
    assert!(orchestrator.contains("Use `validator` for exploitability proof"));
    assert!(
        orchestrator.contains(
            "Use `verifier` after non-trivial worker or toolsmith implementation changes"
        )
    );

    for subordinate in [
        include_str!("../../src/agent/builtins/recon.toml"),
        include_str!("../../src/agent/builtins/auditor.toml"),
        include_str!("../../src/agent/builtins/validator.toml"),
        include_str!("../../src/agent/builtins/toolsmith.toml"),
    ] {
        assert!(subordinate.contains("Plugin artifact contract"));
        assert!(subordinate.contains("Return results in this contract when practical"));
        assert!(subordinate.contains("ARTIFACT REFS"));
        assert!(subordinate.contains("BLOCKERS"));
    }

    let compact = include_str!("../../templates/compact/prompt.md");
    assert!(compact.contains("Active scan mode and depth budget"));
    assert!(compact.contains("Current bounded target slice"));
    assert!(compact.contains("Child outputs pending adoption"));
    assert!(compact.contains("Root decisions adopted this turn"));
    assert!(compact.contains("Pending delegated validations and their owners"));
    assert!(compact.contains("The next shortest justified probe"));
    assert!(compact.contains("Treat child-agent outputs as provisional until root adopts them"));
}

#[test]
fn offensive_notes_match_offensive_default_architecture_and_skill_pack() {
    let Some(proposal) =
        read_optional_repo_note("../../../../notes/offensive/pentest-agent-proposal.md")
    else {
        return;
    };
    let Some(reference) = read_optional_repo_note(
        "../../../../notes/offensive/codex-offensive-fork-reference-proposal.md",
    ) else {
        return;
    };
    let Some(surfaces) =
        read_optional_repo_note("../../../../notes/offensive/global-pentest-prompt-surfaces.md")
    else {
        return;
    };
    let Some(candidate_blocks) =
        read_optional_repo_note("../../../../notes/offensive/candidate-prompt-blocks.md")
    else {
        return;
    };

    for doc in [&proposal, &reference] {
        assert!(doc.contains("Offensive-default Codex"));
        assert!(doc.contains("roles = identity + decision policy + scope semantics + delegation doctrine + checkpoint ownership + approval-aware execution + evidence and reportability standards"));
        assert!(doc.contains("plugin-owned pentest skills"));
        assert!(doc.contains("MCP = deferred"));
        assert!(doc.contains("fork_turns: \"none\""));
        assert!(doc.contains("second root"));
        assert!(doc.contains("child outputs pending adoption"));
        assert!(doc.contains("root decisions adopted this turn"));

        assert!(doc.contains("deep-scan posture"));

        for removed in ["handoff-resume", "finding-drafting"] {
            assert!(
                !doc.contains(removed),
                "offensive docs still mention removed workflow skill: {removed}"
            );
        }
    }

    assert!(proposal.contains("quick` and `standard`"));
    assert!(reference.contains("quick` and `standard`"));
    assert!(surfaces.contains("Global/default surfaces should carry offensive operator identity"));
    assert!(surfaces.contains("Offensive doctrine should remain strongest in `orchestrator`, specialist roles, skills, and compaction"));
    assert!(surfaces.contains("`orchestrator.md` should carry offensive-root doctrine"));
    assert!(surfaces.contains("fork_turns: \"none\""));
    assert!(surfaces.contains("scope semantics"));
    assert!(surfaces.contains("child outputs pending adoption"));
    assert!(surfaces.contains("root decisions adopted this turn"));
    assert!(surfaces.contains("Root must not treat a forked child as a second root."));
    assert!(candidate_blocks.contains("## Adopt Now"));
    assert!(candidate_blocks.contains("## Reference Only"));
    assert!(candidate_blocks.contains("Default to a fresh specialist with `fork_turns: \"none\"`"));
    assert!(candidate_blocks.contains("child outputs pending adoption"));
    assert!(candidate_blocks.contains("root decisions adopted this turn"));
    assert!(candidate_blocks.contains("Put identity, decision policy, scope semantics, delegation doctrine, checkpoint ownership, approval-aware execution, and evidence and reportability standards in roles."));
    assert!(candidate_blocks.contains("Keep quick and standard as reference doctrine"));
}

#[test]
fn skill_pack_recommendations_cover_common_target_shapes() {
    let Some(target_shape_doc) = read_optional_repo_note(
        "../../../../notes/offensive/skill-pack-recommendations-by-target-shape.md",
    ) else {
        return;
    };
    assert!(target_shape_doc.contains("API Auth-Heavy Target"));
    assert!(target_shape_doc.contains("`auth-jwt-methodology`"));
    assert!(target_shape_doc.contains("Upload Or Import Pipeline"));
    assert!(target_shape_doc.contains("`file-upload-methodology`"));
    assert!(target_shape_doc.contains("Fetch Preview Callback Integrations"));
    assert!(target_shape_doc.contains("`ssrf-methodology`"));
    assert!(target_shape_doc.contains("Billing Credits Quotas Workflows"));
    assert!(target_shape_doc.contains("`business-logic-methodology`"));
}
