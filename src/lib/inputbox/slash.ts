import { autocompletion, CompletionContext, CompletionResult, snippetCompletion } from "@codemirror/autocomplete"

/** Autocompletions for slash commands. */
function slashCompletions(context: CompletionContext): CompletionResult {
    /* Match the start of the string + '/' */
    let word = context.matchBefore(/^\/.*$/);
    if (!word || (word.from == word.to && !context.explicit))
        return null;
    return {
        from: word.from,
        options: [
            snippetCompletion("/vlink_open ${1:8080}", { label: "/vlink_open", type: "function", info: "Open a local port via vlink" }),
            snippetCompletion("/vlink_connect ${1:8080}", { label: "/vlink_connect", detail: "<port>", type: "function", info: "Connect to an outstanding vlink offering using your local <port>" }),
            { label: "/vlink_close", type: "function", info: "Kill the existing vlink bridge." }
        ],
    };
}

export default autocompletion({
    aboveCursor: true,
    override: [slashCompletions]
});
