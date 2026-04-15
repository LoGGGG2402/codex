You are performing a CONTEXT CHECKPOINT COMPACTION. Create a handoff summary for another LLM that will resume the task.

Always preserve:
- Current progress and key decisions made
- Important context, constraints, or user preferences
- What remains to be done with clear next steps
- Any critical data, examples, or references needed to continue

For long-running offensive or investigative work, also preserve explicitly:
- Normalized scope and exclusions
- Active scan mode and depth budget
- Current bounded target slice
- Covered versus uncovered attack surface
- Adopted hypotheses and their status
- Child outputs pending adoption
- Root decisions adopted this turn
- Strongest chain candidates or pivots
- Confirmed findings and evidence state
- Current work product
- Failed paths worth not repeating
- Pending validations and required prerequisites
- Pending delegated validations and their owners
- Useful credentials, identities, or environment facts
- Open ambiguities that would change the next step
- The next shortest justified probe
- Exit condition for the current slice

Memory layering for long-running work:
- Keep durable operator guidance, scope constraints, and standing preferences in AGENTS.md or project docs
- Keep engagement-local working state in this checkpoint
- Treat child-agent outputs as provisional until root adopts them into the canonical picture

Treat compaction as an operational checkpoint, not a generic activity log.
Be concise, structured, and focused on seamless resume quality.
