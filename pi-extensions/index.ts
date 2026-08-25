/**
 * jira-to-md Pi extension
 *
 * Adds `fetch_issue` and `search_issues` tools that fetch and search Jira issues
 * via the compiled `jira-to-md` CLI (which must be installed separately).
 *
 * Installed as a pi package from this repo:
 *   pi install git:<host>/user/jira-to-md@<ref>
 * The root package.json declares this directory under `pi.extensions`, so pi
 * discovers it automatically. For the manual copy path, see README.md.
 */

import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { Type, type TypeOf } from "typebox";

// --- Tool Input Schemas ---

const FetchIssueInput = Type.Object({
  issue_key: Type.String({
    description: "The Jira issue key (e.g., 'PROJ-123')",
  }),
});

type FetchIssueInputType = TypeOf<typeof FetchIssueInput>;

const SearchIssuesInput = Type.Object({
  jql_query: Type.String({
    description:
      "JQL query string in double quotes (e.g., \"project = PROJ AND status = 'In Progress'\"))",
  }),
  output_file: Type.Optional(
    Type.String({
      description:
        "Optional file path to save output instead of inline display",
    }),
  ),
});

type SearchIssuesInputType = TypeOf<typeof SearchIssuesInput>;

export default function (pi: ExtensionAPI) {
  // --- fetch_issue tool ---

  pi.registerTool({
    name: "fetch_issue",
    label: "Fetch Jira Issue",
    description:
      "Fetch a single Jira issue by its key and return it as structured Markdown",
    promptSnippet:
      "Get details of a specific Jira ticket including metadata, description, and comments",
    parameters: FetchIssueInput,
    async execute(_toolCallId, params, signal) {
      const issueKey = (params as FetchIssueInputType).issue_key;

      const result = await pi.exec("jira-to-md", ["fetch", issueKey], {
        signal,
        timeout: 30_000,
      });

      if (result.code !== 0) {
        throw new Error(`jira-to-md failed: ${result.stderr || result.stdout}`);
      }

      return {
        content: [{ type: "text", text: result.stdout }],
        details: {},
      };
    },
  });

  // --- search_issues tool ---

  pi.registerTool({
    name: "search_issues",
    label: "Search Jira Issues",
    description:
      "Search for Jira issues using a JQL query and return all matching issues as structured Markdown",
    promptSnippet:
      "Bulk retrieve issues matching JQL criteria, with automatic pagination",
    parameters: SearchIssuesInput,
    async execute(_toolCallId, params, signal) {
      const jqlQuery = (params as SearchIssuesInputType).jql_query;
      const outputFile = (params as SearchIssuesInputType).output_file;

      const args = [jqlQuery];
      if (outputFile) {
        args.push("--output", outputFile);
      }

      const result = await pi.exec("jira-to-md", ["search", ...args], {
        signal,
        timeout: 60_000,
      });

      if (result.code !== 0) {
        throw new Error(`jira-to-md failed: ${result.stderr || result.stdout}`);
      }

      // If output was saved to file, read it back
      let output = result.stdout;
      if (outputFile && output.trim() === "") {
        const fs = await import("node:fs");
        output = fs.readFileSync(outputFile, "utf-8");
      }

      return {
        content: [{ type: "text", text: output }],
        details: {},
      };
    },
  });

  // --- session_start notification ---

  pi.on("session_start", async (_event, ctx) => {
    if (ctx.mode === "tui") {
      ctx.ui.notify("jira-to-md tools ready", "info");
    }
  });
}
