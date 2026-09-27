# MCP protocol profile — #1060

Pinned spec: `2026-07-28` (`gaia_acp::MCP_SPEC`).

Supported: initialize, notifications/initialized, tools/list, tools/call, resources/list, resources/read, prompts/list, prompts/get, ping.

Everything else fails closed (`MethodClass::Unsupported`).

ACP map: descriptors are untrusted metadata. `tools/call` still goes through policy → approval → gateway. Prompts cannot grant capability. HTTP/OAuth off (`STREAMABLE_HTTP_ENABLED = false`).
