# Opt-in plaintext messages for multi-agent V2

This fork adds an explicit plaintext transport for V2 agent messages. It retains
the native agent tree, live mailbox delivery, `spawn_agent`, `send_message`, and
`followup_task`. This is intended for collaboration between providers that cannot
interpret each other's encrypted message arguments.

Enable it in `config.toml`:

```toml
[features.multi_agent_v2]
enabled = true
plaintext_messages = true
```

The default remains `false`. When enabled, the default tool namespace becomes
`collaboration_plaintext`. The reserved `collaboration` namespace is rejected in
this mode. The client declares ordinary string message arguments and stores their
delivery as `AgentMessage::Plaintext`; no gateway tool renaming or response-marker
injection is required. A missing `encrypted_function_args` field in an upstream
tool call is valid in this explicit mode.

The setting controls collaboration messages only. It does not decrypt private
reasoning, compaction state, or historical ciphertext. For cross-provider tasks,
use `fork_turns="none"` and provide the necessary public context in the task.

For full-history forks in plaintext mode, default child-role guidance clarifies
that the latest `NEW_TASK` addressed to the child is its current assignment and
that the parent's delegation step is already complete. This prevents repeating
the inherited parent task. Explicitly configured roles and applicable delegation
restrictions are preserved.

## Validation

Regression coverage includes configuration defaults, namespace selection, V2 tool
schemas, explicit plaintext/encrypted classification, and preservation of child
role guidance. The public patch contains no provider credentials or private
session transcripts.

Previous isolated live checks exercised GPT/DeepSeek, GPT/GPT, and
DeepSeek/DeepSeek collaboration. DeepSeek/DeepSeek covered live messaging,
file-writing, same-child follow-up, and parent-process restart/resume, including
two successful full-history runs. Upstream probes checked complete native tool
declarations and streamed argument delivery. These are local acceptance results,
not a claim that the entire Codex test suite passes.

Builds and test compilation must run with resource limits. The local acceptance
environment caps memory at 5 GiB, swap at 512 MiB, aggregate CPU at two cores, and
all build storage at a fixed 32 GiB ext4 volume. It also limits physical disk I/O
and makes the filesystem outside that volume read-only. Missing resource controls
must fail closed; do not retry an out-of-memory build with higher limits.
