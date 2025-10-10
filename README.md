# Words Lookup Bot

Can look up English words from Stands4 API.

# Roadmap

<table>
    <tr>
        <td>Complete</td>
        <td>Version</td>
        <td>Feature</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.1.0</td>
        <td>Initial release with words and phrases lookup from Stands4 API</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.1.1</td>
        <td>Refactor commands processing to a trait-based approach</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.1.2</td>
        <td>Automate deployments with GitHub Actions</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.1.3</td>
        <td>Add `/help` command to show all usage</td>
    </tr>
    <tr>
        <td>:bug:</td>
        <td>0.1.3.1</td>
        <td>Fix `help` not printing all the help; Handle empty inputs for words and phrases
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.2.0</td>
        <td>Add abbreviations lookup</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.2.1</td>
        <td>Add link to website in case of 5+ definitions; Display defs & abbrs in word-search</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.2.2</td>
        <td>Add MarkdownV2 support; Fixed minor styling issues</td>
    </tr>
    <tr>
        <td>:bug:</td>
        <td>0.2.2.1</td>
        <td>Fix markdown formatting</td>
    </tr>
    <tr>
        <td>:bug:</td>
        <td>0.2.2.2</td>
        <td>Fix markdown formatting - 2nd time</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.2.3</td>
        <td>Display abbreviations in a categorized style</td>
    </tr>
    <tr>
        <td>:bug:</td>
        <td>0.2.3.1</td>
        <td>Fix abbreviations ordering; Fix abbreviations incorrectly joined sentence</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.2.4</td>
        <td>Remove UNFILLED category to fix "Message to long" error</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.3.0</td>
        <td>Refactor to functional style in preparations for inlines</td>
    </tr>
    <tr>
        <td>:bug:</td>
        <td>0.3.1</td>
        <td>Fix `\start` misbehaving</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.4.0</td>
        <td>Introduce inline lookups</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.5.0</td>
        <td>Add wordle look-ups</td>
    </tr>
    <tr>
        <td>:bug:</td>
        <td>0.5.1</td>
        <td>Fix escaping of the special characters</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.5.2</td>
        <td>Minor formatting of the existing code</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.6.0</td>
        <td>Add support for urban dictionary lookups</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.6.1</td>
        <td>Optimise urban dictionary inline lookup</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.6.2</td>
        <td>Use "u." for urban lookups</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.6.3</td>
        <td>Describe help on inlines</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.6.4</td>
        <td>Refactored out LinksProvider abstraction</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.7.0</td>
        <td>Add synonyms/antonyms lookup (for commands only)</td>
    </tr>
    <tr>
        <td>:bug:</td>
        <td>0.7.1</td>
        <td>Fix thesaurus lookup url</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.7.1</td>
        <td>Add synonyms/antonyms lookup (for inlines too)</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.7.2</td>
        <td>Update formatting of single- and multi-lined texts</td>
    </tr>
    <tr>
        <td>:white_check_mark:</td>
        <td>0.7.3</td>
        <td>Update inline help, refactor inlines composition</td>
    </tr>
    <tr>
        <td>:x:</td>
        <td>0.8.0</td>
        <td>Refactor commands into common modules</td>
    </tr>
    <tr>
        <td>:x:</td>
        <td>0.8.1</td>
        <td>Fix inlines formatting</td>
    </tr>
    <tr>
        <td>:x:</td>
        <td>0.8.2</td>
        <td>Fix synonyms & antonyms formatting</td>
    </tr>
    <tr>
        <td>:x:</td>
        <td>0.9.0</td>
        <td>Add in-place buttons for contextual synonyms/antonyms look-up</td>
    </tr>
    <tr>
        <td>:x:</td>
        <td>0.10.0</td>
        <td>Add caching of frequent requests per API route</td>
    </tr>
    <tr>
        <td>:x:</td>
        <td>0.11.0</td>
        <td>Add single api inline lookups</td>
    </tr>
    <tr>
        <td>:x:</td>
        <td>0.12.0</td>
        <td>Add more than 5 lookups to inlines; Add Full-text as an option</td>
    </tr>
    <tr>
        <td>:x:</td>
        <td>0.13.0</td>
        <td>Add renovate bot</td>
    </tr>
    <tr>
        <td>:x:</td>
        <td>0.14.0</td>
        <td>Add CodeRabbit PR reviews</td>
    </tr>
    <tr>
        <td>...</td>
        <td>...</td>
        <td>...</td>
    </tr>
    <tr>
        <td>:x:</td>
        <td>1.0.0</td>
        <td>Release of the bot to the wilds!</td>
    </tr>
</table>
