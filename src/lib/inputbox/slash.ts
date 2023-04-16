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
            snippetCompletion("/vlan_accept ${1:8080}", { label: "/vlan_accept", detail: "<port>", type: "function", info: "Accept incoming VLAN tunnel request." }),
            { label: "/vlan_kill", type: "function", info: "Kill the existing VLAN tunnel." },
            snippetCompletion("/vlan ${1:8080}", { label: "/vlan", detail: "<port>", type: "function", info: "Request a VLAN tunnel to be opened." })
        ],
    };
}

export default autocompletion({
    aboveCursor: true,
    override: [slashCompletions]
});
